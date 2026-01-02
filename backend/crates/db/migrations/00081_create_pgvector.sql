-- Migration: 00081_create_pgvector.sql
-- Epic 103: Storage & Caching Integration
-- Story 103.5: pgvector migration for RAG (Retrieval Augmented Generation)
--
-- This migration:
-- 1. Creates the document_embeddings table for storing document chunks
-- 2. Optionally enables pgvector for vector similarity search (if available)
-- 3. Adds vector-specific columns and indexes only if pgvector is available
--
-- If pgvector is not installed, this migration still succeeds but without
-- vector search capabilities (JSONB fallback for embeddings).

-- Step 1: Create the document_embeddings table (always runs)
-- This table stores chunked document text with embeddings for RAG
CREATE TABLE IF NOT EXISTS document_embeddings (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    document_id UUID NOT NULL REFERENCES documents(id) ON DELETE CASCADE,
    chunk_index INT NOT NULL DEFAULT 0,
    chunk_text TEXT NOT NULL,
    -- JSONB embedding for fallback when pgvector is not available
    embedding JSONB,
    -- Metadata about the chunk (e.g., page number, section, etc.)
    metadata JSONB DEFAULT '{}'::jsonb,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    -- Ensure unique chunks per document
    UNIQUE(document_id, chunk_index)
);

-- Create indexes for the base table
CREATE INDEX IF NOT EXISTS idx_document_embeddings_org ON document_embeddings(organization_id);
CREATE INDEX IF NOT EXISTS idx_document_embeddings_doc ON document_embeddings(document_id);
CREATE INDEX IF NOT EXISTS idx_document_embeddings_updated ON document_embeddings(updated_at);

-- Enable RLS on document_embeddings
ALTER TABLE document_embeddings ENABLE ROW LEVEL SECURITY;

-- RLS policy for document_embeddings (organization-scoped access)
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_policies
        WHERE tablename = 'document_embeddings'
        AND policyname = 'document_embeddings_org_isolation'
    ) THEN
        CREATE POLICY document_embeddings_org_isolation ON document_embeddings
            FOR ALL
            USING (organization_id = current_setting('app.current_organization_id', true)::UUID)
            WITH CHECK (organization_id = current_setting('app.current_organization_id', true)::UUID);
    END IF;
END $$;

-- Step 2: Optionally enable pgvector if available
DO $$
DECLARE
    v_extension_available BOOLEAN;
BEGIN
    -- Check if the vector extension is available on this PostgreSQL installation
    SELECT EXISTS (
        SELECT 1 FROM pg_available_extensions WHERE name = 'vector'
    ) INTO v_extension_available;

    IF v_extension_available THEN
        -- Create the extension
        CREATE EXTENSION IF NOT EXISTS vector;
        RAISE NOTICE 'pgvector extension enabled successfully';
    ELSE
        RAISE NOTICE 'pgvector extension is not available on this server - skipping vector setup. Install pgvector for production use.';
    END IF;
END $$;

-- Step 3: Add vector column if pgvector extension exists
DO $$
BEGIN
    IF EXISTS (SELECT 1 FROM pg_extension WHERE extname = 'vector') THEN
        IF NOT EXISTS (
            SELECT 1 FROM information_schema.columns
            WHERE table_name = 'document_embeddings'
            AND column_name = 'embedding_vector'
        ) THEN
            -- Add the vector column for pgvector (1536 dims = OpenAI ada-002)
            EXECUTE 'ALTER TABLE document_embeddings ADD COLUMN embedding_vector vector(1536)';
            RAISE NOTICE 'Added embedding_vector column to document_embeddings';
        END IF;
    END IF;
END $$;

-- Step 4: Create IVFFlat index for cosine similarity search (only if pgvector exists)
DO $$
BEGIN
    IF EXISTS (SELECT 1 FROM pg_extension WHERE extname = 'vector') THEN
        IF NOT EXISTS (
            SELECT 1 FROM pg_indexes
            WHERE indexname = 'idx_document_embeddings_vector'
        ) THEN
            EXECUTE 'CREATE INDEX idx_document_embeddings_vector ON document_embeddings USING ivfflat (embedding_vector vector_cosine_ops) WITH (lists = 100)';
            RAISE NOTICE 'Created IVFFlat index for vector similarity search';
        END IF;
    END IF;
END $$;

-- Step 5: Create helper functions only if pgvector is available
DO $$
BEGIN
    IF EXISTS (SELECT 1 FROM pg_extension WHERE extname = 'vector') THEN
        -- Create a function to migrate existing JSONB embeddings to vector format
        EXECUTE $func$
            CREATE OR REPLACE FUNCTION migrate_jsonb_to_vector()
            RETURNS void AS $inner$
            DECLARE
                rec RECORD;
                emb_array float8[];
            BEGIN
                FOR rec IN
                    SELECT id, embedding
                    FROM document_embeddings
                    WHERE embedding IS NOT NULL
                    AND embedding_vector IS NULL
                    AND jsonb_typeof(embedding) = 'array'
                LOOP
                    SELECT ARRAY(SELECT (e)::float8 FROM jsonb_array_elements_text(rec.embedding) e)
                    INTO emb_array;

                    IF array_length(emb_array, 1) = 1536 THEN
                        UPDATE document_embeddings
                        SET embedding_vector = emb_array::vector
                        WHERE id = rec.id;
                    END IF;
                END LOOP;
            END;
            $inner$ LANGUAGE plpgsql
        $func$;

        -- Create vector similarity search function for RAG
        EXECUTE $func$
            CREATE OR REPLACE FUNCTION search_similar_documents(
                p_organization_id UUID,
                p_query_vector vector(1536),
                p_limit INT DEFAULT 10,
                p_min_similarity FLOAT DEFAULT 0.5
            )
            RETURNS TABLE (
                id UUID,
                document_id UUID,
                chunk_index INT,
                chunk_text TEXT,
                metadata JSONB,
                similarity FLOAT
            ) AS $inner$
            BEGIN
                RETURN QUERY
                SELECT
                    de.id,
                    de.document_id,
                    de.chunk_index,
                    de.chunk_text,
                    de.metadata,
                    1 - (de.embedding_vector <=> p_query_vector) AS similarity
                FROM document_embeddings de
                WHERE de.organization_id = p_organization_id
                AND de.embedding_vector IS NOT NULL
                AND 1 - (de.embedding_vector <=> p_query_vector) >= p_min_similarity
                ORDER BY de.embedding_vector <=> p_query_vector
                LIMIT p_limit;
            END;
            $inner$ LANGUAGE plpgsql
        $func$;

        -- Create upsert function for document embeddings with vector
        EXECUTE $func$
            CREATE OR REPLACE FUNCTION upsert_document_embedding(
                p_organization_id UUID,
                p_document_id UUID,
                p_chunk_index INT,
                p_chunk_text TEXT,
                p_embedding_vector vector(1536),
                p_metadata JSONB DEFAULT '{}'::jsonb
            )
            RETURNS UUID AS $inner$
            DECLARE
                v_id UUID;
            BEGIN
                SELECT id INTO v_id
                FROM document_embeddings
                WHERE document_id = p_document_id
                AND chunk_index = p_chunk_index;

                IF v_id IS NOT NULL THEN
                    UPDATE document_embeddings
                    SET
                        chunk_text = p_chunk_text,
                        embedding_vector = p_embedding_vector,
                        embedding = NULL,
                        metadata = p_metadata,
                        updated_at = NOW()
                    WHERE id = v_id;
                ELSE
                    INSERT INTO document_embeddings (
                        organization_id,
                        document_id,
                        chunk_index,
                        chunk_text,
                        embedding_vector,
                        metadata
                    )
                    VALUES (
                        p_organization_id,
                        p_document_id,
                        p_chunk_index,
                        p_chunk_text,
                        p_embedding_vector,
                        p_metadata
                    )
                    RETURNING id INTO v_id;
                END IF;

                RETURN v_id;
            END;
            $inner$ LANGUAGE plpgsql
        $func$;

        RAISE NOTICE 'Created pgvector helper functions';
    END IF;
END $$;

-- Step 6: Create statistics view for RAG system monitoring
-- This view works regardless of whether pgvector is available
CREATE OR REPLACE VIEW v_rag_statistics AS
SELECT
    organization_id,
    COUNT(DISTINCT document_id) AS indexed_documents,
    COUNT(*) AS total_chunks,
    COUNT(*) FILTER (WHERE embedding IS NOT NULL) AS chunks_with_embedding,
    AVG(char_length(chunk_text))::INT AS avg_chunk_length,
    MAX(updated_at) AS last_updated
FROM document_embeddings
GROUP BY organization_id;

-- Grant appropriate permissions
GRANT SELECT ON v_rag_statistics TO PUBLIC;

-- Add comments for documentation
COMMENT ON TABLE document_embeddings IS 'Story 103.5: Document chunks with embeddings for RAG (Retrieval Augmented Generation)';
COMMENT ON VIEW v_rag_statistics IS 'Story 103.5: RAG system statistics per organization';

-- Add comments for functions if they exist
DO $$
BEGIN
    IF EXISTS (SELECT 1 FROM pg_extension WHERE extname = 'vector') THEN
        COMMENT ON FUNCTION search_similar_documents IS 'Story 103.5: Vector similarity search for RAG using pgvector cosine distance';
        COMMENT ON FUNCTION upsert_document_embedding IS 'Story 103.5: Upsert document embedding with vector for RAG indexing';
        COMMENT ON FUNCTION migrate_jsonb_to_vector IS 'Story 103.5: Migrate existing JSONB embeddings to pgvector format';
    END IF;
END $$;

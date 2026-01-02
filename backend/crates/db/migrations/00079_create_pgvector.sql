-- Migration: 00079_create_pgvector.sql
-- Epic 103: Storage & Caching Integration
-- Story 103.5: pgvector migration for RAG (Retrieval Augmented Generation)
--
-- This migration enables vector similarity search for document embeddings.
-- pgvector extension must be available on the PostgreSQL server.

-- Enable pgvector extension (requires superuser or extension privilege)
-- If the extension is not available, this migration will fail gracefully.
-- On managed databases like Supabase, RDS, the extension is pre-installed.
CREATE EXTENSION IF NOT EXISTS vector;

-- Add vector column to document_embeddings table for efficient similarity search
-- Using 1536 dimensions (OpenAI ada-002 embedding size)
-- For other models, adjust the dimension accordingly:
--   - OpenAI text-embedding-3-small: 1536
--   - OpenAI text-embedding-3-large: 3072
--   - Anthropic Claude embeddings: 1024
--   - Cohere embed-english-v3.0: 1024

-- First, check if the embedding_vector column already exists
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM information_schema.columns
        WHERE table_name = 'document_embeddings'
        AND column_name = 'embedding_vector'
    ) THEN
        -- Add the vector column for pgvector
        ALTER TABLE document_embeddings
        ADD COLUMN embedding_vector vector(1536);
    END IF;
END $$;

-- Create an index for cosine similarity search (IVFFlat index)
-- This index significantly speeds up similarity searches
-- The lists parameter (100) is tuned for ~100k documents
-- For larger datasets, increase lists: sqrt(num_rows)
CREATE INDEX IF NOT EXISTS idx_document_embeddings_vector
ON document_embeddings
USING ivfflat (embedding_vector vector_cosine_ops)
WITH (lists = 100);

-- Create a function to migrate existing JSONB embeddings to vector format
CREATE OR REPLACE FUNCTION migrate_jsonb_to_vector()
RETURNS void AS $$
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
        -- Convert JSONB array to PostgreSQL array
        SELECT ARRAY(SELECT (e)::float8 FROM jsonb_array_elements_text(rec.embedding) e)
        INTO emb_array;

        -- Only update if the array has the expected dimension
        IF array_length(emb_array, 1) = 1536 THEN
            UPDATE document_embeddings
            SET embedding_vector = emb_array::vector
            WHERE id = rec.id;
        END IF;
    END LOOP;
END;
$$ LANGUAGE plpgsql;

-- Create a function for vector similarity search (RAG)
-- Returns documents sorted by cosine similarity to query vector
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
) AS $$
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
$$ LANGUAGE plpgsql;

-- Create a function to upsert document embedding with vector
CREATE OR REPLACE FUNCTION upsert_document_embedding(
    p_organization_id UUID,
    p_document_id UUID,
    p_chunk_index INT,
    p_chunk_text TEXT,
    p_embedding_vector vector(1536),
    p_metadata JSONB DEFAULT '{}'::jsonb
)
RETURNS UUID AS $$
DECLARE
    v_id UUID;
BEGIN
    -- Check if embedding already exists for this document chunk
    SELECT id INTO v_id
    FROM document_embeddings
    WHERE document_id = p_document_id
    AND chunk_index = p_chunk_index;

    IF v_id IS NOT NULL THEN
        -- Update existing embedding
        UPDATE document_embeddings
        SET
            chunk_text = p_chunk_text,
            embedding_vector = p_embedding_vector,
            embedding = NULL, -- Clear JSONB embedding when vector is set
            metadata = p_metadata,
            updated_at = NOW()
        WHERE id = v_id;
    ELSE
        -- Insert new embedding
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
$$ LANGUAGE plpgsql;

-- Create statistics view for RAG system monitoring
CREATE OR REPLACE VIEW v_rag_statistics AS
SELECT
    organization_id,
    COUNT(DISTINCT document_id) AS indexed_documents,
    COUNT(*) AS total_chunks,
    COUNT(*) FILTER (WHERE embedding_vector IS NOT NULL) AS chunks_with_vector,
    COUNT(*) FILTER (WHERE embedding IS NOT NULL AND embedding_vector IS NULL) AS chunks_pending_migration,
    AVG(char_length(chunk_text))::INT AS avg_chunk_length,
    MAX(updated_at) AS last_updated
FROM document_embeddings
GROUP BY organization_id;

-- Grant appropriate permissions
GRANT SELECT ON v_rag_statistics TO PUBLIC;

-- Add comment for documentation
COMMENT ON FUNCTION search_similar_documents IS 'Story 103.5: Vector similarity search for RAG using pgvector cosine distance';
COMMENT ON FUNCTION upsert_document_embedding IS 'Story 103.5: Upsert document embedding with vector for RAG indexing';
COMMENT ON FUNCTION migrate_jsonb_to_vector IS 'Story 103.5: Migrate existing JSONB embeddings to pgvector format';
COMMENT ON VIEW v_rag_statistics IS 'Story 103.5: RAG system statistics per organization';

import "../shared/models.tsp";
import "../shared/errors.tsp";
import "../shared/pagination.tsp";
import "../shared/tenant.tsp";

namespace PropertyManagement.Documents;

using TypeSpec.Http;
using PropertyManagement.Shared;

@doc("Document (UC-08)")
model Document {
  id: uuid;
  organizationId: uuid;
  buildingId?: uuid;
  unitId?: uuid;
  title: string;
  description?: string;
  category: DocumentCategory;
  file: Attachment;
  visibility: DocumentVisibility;
  tags?: string[];
  expiresAt?: dateTime;
  version: int32;
  previousVersionId?: uuid;
  ...AuditableEntity;
}

enum DocumentCategory {
  Contract: "contract",
  Invoice: "invoice",
  Receipt: "receipt",
  Report: "report",
  Minutes: "minutes",
  Policy: "policy",
  Manual: "manual",
  Certificate: "certificate",
  Permit: "permit",
  Insurance: "insurance",
  Other: "other",
}

enum DocumentVisibility {
  Public: "public",
  BuildingOnly: "building_only",
  OwnersOnly: "owners_only",
  ManagersOnly: "managers_only",
  Private: "private",
}

@doc("Document folder")
model Folder {
  id: uuid;
  organizationId: uuid;
  parentId?: uuid;
  name: string;
  description?: string;
  visibility: DocumentVisibility;
  documentCount: int32;
  ...AuditableEntity;
}

@route("/api/v1/documents")
@tag("Documents")
interface DocumentsApi {
  @get list(
    @header("Authorization") token: string,
    @header("X-Tenant-ID") tenantId: uuid,
    ...PaginationQuery,
    ...SearchQuery,
    @query buildingId?: uuid,
    @query category?: DocumentCategory,
    @query folderId?: uuid
  ): PaginatedResponse<Document>;

  @post upload(
    @header("Authorization") token: string,
    @header("X-Tenant-ID") tenantId: uuid,
    @body body: {
      title: string;
      description?: string;
      category: DocumentCategory;
      buildingId?: uuid;
      unitId?: uuid;
      folderId?: uuid;
      visibility: DocumentVisibility;
      tags?: string[];
      fileContent: string;
      fileName: string;
      mimeType: string;
    }
  ): { @statusCode statusCode: 201; @body body: Document; };

  @get @route("/{id}") get(
    @header("Authorization") token: string,
    @header("X-Tenant-ID") tenantId: uuid,
    @path id: uuid
  ): Document | NotFoundError;

  @patch @route("/{id}") update(
    @header("Authorization") token: string,
    @header("X-Tenant-ID") tenantId: uuid,
    @path id: uuid,
    @body body: { title?: string; description?: string; category?: DocumentCategory; visibility?: DocumentVisibility; tags?: string[]; }
  ): Document;

  @delete @route("/{id}") delete(
    @header("Authorization") token: string,
    @header("X-Tenant-ID") tenantId: uuid,
    @path id: uuid
  ): { @statusCode statusCode: 204; };

  @get @route("/{id}/download") download(
    @header("Authorization") token: string,
    @header("X-Tenant-ID") tenantId: uuid,
    @path id: uuid
  ): { @header("Content-Type") contentType: string; @body body: bytes; };

  @post @route("/{id}/new-version") uploadNewVersion(
    @header("Authorization") token: string,
    @header("X-Tenant-ID") tenantId: uuid,
    @path id: uuid,
    @body body: { fileContent: string; fileName: string; mimeType: string; }
  ): Document;

  @get @route("/{id}/versions") listVersions(
    @header("Authorization") token: string,
    @header("X-Tenant-ID") tenantId: uuid,
    @path id: uuid
  ): Document[];

  @get @route("/folders") listFolders(
    @header("Authorization") token: string,
    @header("X-Tenant-ID") tenantId: uuid,
    @query parentId?: uuid
  ): Folder[];

  @post @route("/folders") createFolder(
    @header("Authorization") token: string,
    @header("X-Tenant-ID") tenantId: uuid,
    @body body: { name: string; description?: string; parentId?: uuid; visibility: DocumentVisibility; }
  ): { @statusCode statusCode: 201; @body body: Folder; };
}

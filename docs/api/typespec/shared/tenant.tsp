namespace PropertyManagement.Shared;

using TypeSpec.Http;

@doc("Tenant context - required for all multi-tenant operations")
model TenantContext {
  @doc("Tenant/Organization ID")
  @header("X-Tenant-ID")
  tenantId: uuid;
}

@doc("User role within tenant")
enum TenantRole {
  @doc("Super administrator (platform level)")
  SuperAdmin: "super_admin",

  @doc("Organization administrator")
  OrgAdmin: "org_admin",

  @doc("Building manager")
  Manager: "manager",

  @doc("Technical manager")
  TechnicalManager: "technical_manager",

  @doc("Property owner")
  Owner: "owner",

  @doc("Owner's delegate")
  OwnerDelegate: "owner_delegate",

  @doc("Tenant/renter")
  Tenant: "tenant",

  @doc("Resident (no ownership)")
  Resident: "resident",

  @doc("Short-term rental property manager")
  PropertyManager: "property_manager",

  @doc("Real estate agent")
  RealEstateAgent: "real_estate_agent",

  @doc("Guest (temporary access)")
  Guest: "guest",
}

@doc("Permission scope")
enum PermissionScope {
  @doc("Full read access")
  Read: "read",

  @doc("Create new resources")
  Create: "create",

  @doc("Update existing resources")
  Update: "update",

  @doc("Delete resources")
  Delete: "delete",

  @doc("Administrative actions")
  Admin: "admin",
}

@doc("Resource access level")
model ResourceAccess {
  @doc("Resource type (e.g., 'building', 'unit', 'document')")
  resourceType: string;

  @doc("Resource ID (null for all resources of type)")
  resourceId?: uuid;

  @doc("Granted permissions")
  permissions: PermissionScope[];

  @doc("Access expiration (null for permanent)")
  expiresAt?: dateTime;
}

@doc("Delegation of rights")
model Delegation {
  @doc("Unique identifier")
  id: uuid;

  @doc("User granting the delegation")
  fromUserId: uuid;

  @doc("User receiving the delegation")
  toUserId: uuid;

  @doc("Delegated resource accesses")
  accesses: ResourceAccess[];

  @doc("Delegation start date")
  startsAt: dateTime;

  @doc("Delegation end date (null for permanent)")
  endsAt?: dateTime;

  @doc("Whether delegation is currently active")
  isActive: boolean;

  @doc("Reason for delegation")
  reason?: string;
}

@doc("Authenticated user info (from JWT)")
model AuthenticatedUser {
  @doc("User ID")
  userId: uuid;

  @doc("Current tenant ID")
  tenantId: uuid;

  @doc("User's role in current tenant")
  role: TenantRole;

  @doc("User's email")
  email: email;

  @doc("User's display name")
  displayName: string;

  @doc("Active delegations to this user")
  delegations?: Delegation[];
}

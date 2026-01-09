# Database Seeding

This document describes how to seed the database with sample data for development and testing.

## Quick Start

```bash
# Interactive mode (recommended)
cargo run -p api-server --bin ppt-seed

# Non-interactive mode
cargo run -p api-server --bin ppt-seed -- \
  --admin-email admin@example.com \
  --admin-password SecurePass123
```

## CLI Options

| Flag | Short | Description |
|------|-------|-------------|
| `--admin-email` | | Admin email address (prompts if not provided) |
| `--admin-password` | | Admin password (prompts if not provided) |
| `--force` | `-f` | Drop existing seed data before seeding |
| `--minimal` | | Create admin only, skip sample data |
| `--yes` | `-y` | Skip confirmation prompts |

## Password Requirements

The admin password must meet these requirements:
- At least 8 characters
- At least 1 uppercase letter
- At least 1 number

## Sample Data Overview

When running without `--minimal`, the seed creates comprehensive sample data:

### Organization

| Name | Slug | Email |
|------|------|-------|
| Demo Property Management | demo-property | contact@demo-property.test |

### Users (15 users covering all 12 role types)

| Role | Email | Default Password |
|------|-------|------------------|
| Super Admin | (your input) | (your input) |
| Organization Admin | orgadmin@demo-property.test | DemoPass123 |
| Manager | manager@demo-property.test | DemoPass123 |
| Technical Manager | techmanager@demo-property.test | DemoPass123 |
| Owner (x3) | owner1/2/3@demo-property.test | DemoPass123 |
| Owner Delegate | delegate@demo-property.test | DemoPass123 |
| Property Manager | propmgr@demo-property.test | DemoPass123 |
| Real Estate Agent | agent@demo-property.test | DemoPass123 |
| Tenant (x3) | tenant1/2/3@demo-property.test | DemoPass123 |
| Resident (x2) | resident1/2@demo-property.test | DemoPass123 |
| Guest | guest@demo-property.test | DemoPass123 |

### Buildings (3 buildings with 19 units)

#### 1. Sunrise Apartments (Hlavná 123, Bratislava)
- 5 floors, built 2010
- 8 units: 5 apartments, 2 parking, 1 storage

#### 2. Oak Street Residence (Dubová 45, Bratislava)
- 3 floors, built 2015
- 6 units: 5 apartments, 1 storage

#### 3. Central Plaza (Centrálna 1, Bratislava)
- 4 floors, built 2020
- 5 units: 2 commercial, 3 apartments (including penthouse)

### Unit Assignments

Sample users are assigned to units:
- owner1 → Unit 1A + Parking P1 (Sunrise)
- owner2 → Unit 2B (Sunrise, rents to tenant1)
- owner3 → Unit 101 (Oak Street)
- tenant1 → Unit 2B (Sunrise)
- tenant2 → Unit 201 (Oak Street)
- tenant3 → Unit G1 Commercial (Central Plaza)
- resident1 → Unit 1A (family member of owner1)
- resident2 → Unit 201 (subtenant)

## Re-seeding

To re-seed the database (drop existing seed data and create fresh):

```bash
cargo run -p api-server --bin ppt-seed -- --force
```

This will:
1. Delete all users with emails ending in `@demo-property.test`
2. Delete the Demo Property Management organization
3. Delete all associated buildings, units, and memberships
4. Create fresh seed data

## Minimal Seed

For a minimal setup with just the admin user and organization:

```bash
cargo run -p api-server --bin ppt-seed -- --minimal
```

This creates:
- 1 organization (Demo Property Management)
- 1 super admin user (with your credentials)

No buildings, units, or sample users are created.

## SQL Bootstrap Function

For fresh database installs, a SQL function is available:

```sql
-- Bootstrap minimal seed (1 org + 1 admin)
SELECT * FROM seed_bootstrap('admin@example.com', '<argon2-hash>');
```

**Note:** The password hash must be generated using Argon2id. The CLI is recommended for ease of use.

## Security Notes

1. **Password Hashing**: All passwords are hashed using Argon2id before storage
2. **RLS Context**: The seed process uses super admin context to bypass Row-Level Security during data creation
3. **Context Cleanup**: RLS context is automatically cleared after seeding completes
4. **Email Domain**: All sample users use `@demo-property.test` for easy identification and cleanup
5. **No Hardcoded Admin**: The admin password is never stored in code - always provided at runtime

## Environment Variables

| Variable | Required | Description |
|----------|----------|-------------|
| `DATABASE_URL` | Yes | PostgreSQL connection string |

Example:
```bash
DATABASE_URL=postgres://user:pass@localhost:5432/ppt cargo run -p api-server --bin ppt-seed
```

## Troubleshooting

### "Seed data already exists"

Run with `--force` to drop existing seed data:
```bash
cargo run -p api-server --bin ppt-seed -- --force
```

### Password validation errors

Ensure your password meets all requirements:
- At least 8 characters
- At least 1 uppercase letter
- At least 1 number

### Database connection errors

1. Verify `DATABASE_URL` is set correctly
2. Ensure PostgreSQL is running
3. Check network connectivity to the database

### RLS policy errors

The seed process sets super admin context to bypass RLS. If you see RLS-related errors:
1. Check that the `set_request_context` and `clear_request_context` functions exist
2. Verify migration 00006 (RLS policies) has been applied

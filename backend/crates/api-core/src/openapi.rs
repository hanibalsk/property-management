//! OpenAPI documentation setup.

use utoipa::openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme};
use utoipa::{Modify, OpenApi};

/// API documentation modifier to add security schemes.
pub struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "bearer_auth",
                SecurityScheme::Http(
                    HttpBuilder::new()
                        .scheme(HttpAuthScheme::Bearer)
                        .bearer_format("JWT")
                        .build(),
                ),
            );
        }
    }
}

/// Base OpenAPI configuration.
#[derive(OpenApi)]
#[openapi(
    info(
        title = "Property Management API",
        version = "1.0.0",
        description = "API for Property Management System",
        license(name = "MIT"),
        contact(
            name = "API Support",
            url = "https://github.com/hanibalsk/property-management"
        )
    ),
    servers(
        (url = "http://localhost:8080", description = "Development"),
        (url = "https://api.property-management.example.com", description = "Production")
    ),
    modifiers(&SecurityAddon),
    tags(
        (name = "Authentication", description = "User authentication and authorization"),
        (name = "Organizations", description = "Multi-tenancy and organization management"),
        (name = "Buildings", description = "Building management"),
        (name = "Units", description = "Property units and ownership"),
        (name = "Faults", description = "Fault reporting and tracking"),
        (name = "Voting", description = "Voting and polls"),
        (name = "Documents", description = "Document management"),
        (name = "Rentals", description = "Short-term rental management"),
        (name = "Listings", description = "Real estate listings"),
        (name = "Compliance", description = "GDPR and compliance")
    )
)]
pub struct ApiDoc;

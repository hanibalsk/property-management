//! Seed data definitions.
//!
//! Contains predefined sample data for organizations, users, buildings, and units.

/// Seed organization data.
#[derive(Debug, Clone)]
pub struct SeedOrganization {
    pub name: &'static str,
    pub slug: &'static str,
    pub contact_email: &'static str,
}

/// Seed building data.
#[derive(Debug, Clone)]
pub struct SeedBuilding {
    pub name: &'static str,
    pub street: &'static str,
    pub city: &'static str,
    pub postal_code: &'static str,
    pub country: &'static str,
    pub total_floors: i32,
    pub year_built: Option<i32>,
    pub units: Vec<SeedUnit>,
}

/// Seed unit data.
#[derive(Debug, Clone)]
pub struct SeedUnit {
    pub designation: &'static str,
    pub floor: i32,
    pub unit_type: &'static str,
    pub size_sqm: Option<i32>,
    pub rooms: Option<i32>,
}

/// Seed user data.
#[derive(Debug, Clone)]
pub struct SeedUser {
    pub email: &'static str,
    pub name: &'static str,
    pub role_type: &'static str,
    pub phone: Option<&'static str>,
    /// Unit assignments for this user
    pub unit_assignments: Vec<SeedUnitAssignment>,
}

/// Assignment of a user to a unit.
#[derive(Debug, Clone)]
pub struct SeedUnitAssignment {
    /// Index into buildings array
    pub building_index: usize,
    /// Index into building's units array
    pub unit_index: usize,
    /// Resident type: owner, tenant, family_member, subtenant
    pub resident_type: &'static str,
    /// Whether this is the primary resident
    pub is_primary: bool,
}

/// Complete seed data configuration.
#[derive(Debug, Clone)]
pub struct SeedData {
    pub organization: SeedOrganization,
    pub buildings: Vec<SeedBuilding>,
    pub users: Vec<SeedUser>,
    /// Default password for non-admin users
    pub default_password: &'static str,
}

impl Default for SeedData {
    fn default() -> Self {
        Self {
            organization: SeedOrganization {
                name: "Demo Property Management",
                slug: "demo-property",
                contact_email: "contact@demo-property.test",
            },
            buildings: vec![
                // Building 0: Sunrise Apartments (8 units)
                SeedBuilding {
                    name: "Sunrise Apartments",
                    street: "Hlavná 123",
                    city: "Bratislava",
                    postal_code: "81101",
                    country: "SK",
                    total_floors: 5,
                    year_built: Some(2010),
                    units: vec![
                        SeedUnit {
                            designation: "1A",
                            floor: 1,
                            unit_type: "apartment",
                            size_sqm: Some(65),
                            rooms: Some(2),
                        },
                        SeedUnit {
                            designation: "1B",
                            floor: 1,
                            unit_type: "apartment",
                            size_sqm: Some(75),
                            rooms: Some(3),
                        },
                        SeedUnit {
                            designation: "2A",
                            floor: 2,
                            unit_type: "apartment",
                            size_sqm: Some(65),
                            rooms: Some(2),
                        },
                        SeedUnit {
                            designation: "2B",
                            floor: 2,
                            unit_type: "apartment",
                            size_sqm: Some(75),
                            rooms: Some(3),
                        },
                        SeedUnit {
                            designation: "3A",
                            floor: 3,
                            unit_type: "apartment",
                            size_sqm: Some(85),
                            rooms: Some(4),
                        },
                        SeedUnit {
                            designation: "P1",
                            floor: -1,
                            unit_type: "parking",
                            size_sqm: Some(15),
                            rooms: None,
                        },
                        SeedUnit {
                            designation: "P2",
                            floor: -1,
                            unit_type: "parking",
                            size_sqm: Some(15),
                            rooms: None,
                        },
                        SeedUnit {
                            designation: "S1",
                            floor: -1,
                            unit_type: "storage",
                            size_sqm: Some(5),
                            rooms: None,
                        },
                    ],
                },
                // Building 1: Oak Street Residence (6 units)
                SeedBuilding {
                    name: "Oak Street Residence",
                    street: "Dubová 45",
                    city: "Bratislava",
                    postal_code: "82105",
                    country: "SK",
                    total_floors: 3,
                    year_built: Some(2015),
                    units: vec![
                        SeedUnit {
                            designation: "101",
                            floor: 1,
                            unit_type: "apartment",
                            size_sqm: Some(55),
                            rooms: Some(2),
                        },
                        SeedUnit {
                            designation: "102",
                            floor: 1,
                            unit_type: "apartment",
                            size_sqm: Some(55),
                            rooms: Some(2),
                        },
                        SeedUnit {
                            designation: "201",
                            floor: 2,
                            unit_type: "apartment",
                            size_sqm: Some(70),
                            rooms: Some(3),
                        },
                        SeedUnit {
                            designation: "202",
                            floor: 2,
                            unit_type: "apartment",
                            size_sqm: Some(70),
                            rooms: Some(3),
                        },
                        SeedUnit {
                            designation: "301",
                            floor: 3,
                            unit_type: "apartment",
                            size_sqm: Some(90),
                            rooms: Some(4),
                        },
                        SeedUnit {
                            designation: "G01",
                            floor: 0,
                            unit_type: "storage",
                            size_sqm: Some(8),
                            rooms: None,
                        },
                    ],
                },
                // Building 2: Central Plaza (5 units, mixed use)
                SeedBuilding {
                    name: "Central Plaza",
                    street: "Centrálna 1",
                    city: "Bratislava",
                    postal_code: "81102",
                    country: "SK",
                    total_floors: 4,
                    year_built: Some(2020),
                    units: vec![
                        SeedUnit {
                            designation: "G1",
                            floor: 0,
                            unit_type: "commercial",
                            size_sqm: Some(120),
                            rooms: None,
                        },
                        SeedUnit {
                            designation: "G2",
                            floor: 0,
                            unit_type: "commercial",
                            size_sqm: Some(80),
                            rooms: None,
                        },
                        SeedUnit {
                            designation: "A1",
                            floor: 1,
                            unit_type: "apartment",
                            size_sqm: Some(100),
                            rooms: Some(4),
                        },
                        SeedUnit {
                            designation: "A2",
                            floor: 2,
                            unit_type: "apartment",
                            size_sqm: Some(100),
                            rooms: Some(4),
                        },
                        SeedUnit {
                            designation: "PH",
                            floor: 3,
                            unit_type: "apartment",
                            size_sqm: Some(150),
                            rooms: Some(5),
                        },
                    ],
                },
            ],
            users: vec![
                // Organization Admin
                SeedUser {
                    email: "orgadmin@demo-property.test",
                    name: "Organization Admin",
                    role_type: "Organization Admin",
                    phone: Some("+421900111001"),
                    unit_assignments: vec![],
                },
                // Manager
                SeedUser {
                    email: "manager@demo-property.test",
                    name: "Building Manager",
                    role_type: "Manager",
                    phone: Some("+421900111002"),
                    unit_assignments: vec![],
                },
                // Technical Manager
                SeedUser {
                    email: "techmanager@demo-property.test",
                    name: "Technical Manager",
                    role_type: "Technical Manager",
                    phone: Some("+421900111003"),
                    unit_assignments: vec![],
                },
                // Owner 1 - owns unit 1A in Sunrise + parking P1
                SeedUser {
                    email: "owner1@demo-property.test",
                    name: "Jana Nováková",
                    role_type: "Owner",
                    phone: Some("+421900222001"),
                    unit_assignments: vec![
                        SeedUnitAssignment {
                            building_index: 0,
                            unit_index: 0, // 1A
                            resident_type: "owner",
                            is_primary: true,
                        },
                        SeedUnitAssignment {
                            building_index: 0,
                            unit_index: 5, // P1
                            resident_type: "owner",
                            is_primary: true,
                        },
                    ],
                },
                // Owner 2 - owns unit 2B in Sunrise, rents it out
                SeedUser {
                    email: "owner2@demo-property.test",
                    name: "Peter Horváth",
                    role_type: "Owner",
                    phone: Some("+421900222002"),
                    unit_assignments: vec![SeedUnitAssignment {
                        building_index: 0,
                        unit_index: 3, // 2B
                        resident_type: "owner",
                        is_primary: true,
                    }],
                },
                // Owner 3 - owns unit 101 in Oak Street
                SeedUser {
                    email: "owner3@demo-property.test",
                    name: "Mária Kováčová",
                    role_type: "Owner",
                    phone: Some("+421900222003"),
                    unit_assignments: vec![SeedUnitAssignment {
                        building_index: 1,
                        unit_index: 0, // 101
                        resident_type: "owner",
                        is_primary: true,
                    }],
                },
                // Owner Delegate - represents owner2
                SeedUser {
                    email: "delegate@demo-property.test",
                    name: "Martin Delegát",
                    role_type: "Owner Delegate",
                    phone: Some("+421900222004"),
                    unit_assignments: vec![],
                },
                // Property Manager - manages short-term rentals
                SeedUser {
                    email: "propmgr@demo-property.test",
                    name: "Lucia Property",
                    role_type: "Property Manager",
                    phone: Some("+421900333001"),
                    unit_assignments: vec![],
                },
                // Real Estate Agent
                SeedUser {
                    email: "agent@demo-property.test",
                    name: "Tomáš Agent",
                    role_type: "Real Estate Agent",
                    phone: Some("+421900333002"),
                    unit_assignments: vec![],
                },
                // Tenant 1 - rents unit 2B in Sunrise (from owner2)
                SeedUser {
                    email: "tenant1@demo-property.test",
                    name: "Ján Nájomník",
                    role_type: "Tenant",
                    phone: Some("+421900444001"),
                    unit_assignments: vec![SeedUnitAssignment {
                        building_index: 0,
                        unit_index: 3, // 2B
                        resident_type: "tenant",
                        is_primary: true,
                    }],
                },
                // Tenant 2 - rents unit 201 in Oak Street
                SeedUser {
                    email: "tenant2@demo-property.test",
                    name: "Eva Prenajímateľka",
                    role_type: "Tenant",
                    phone: Some("+421900444002"),
                    unit_assignments: vec![SeedUnitAssignment {
                        building_index: 1,
                        unit_index: 2, // 201
                        resident_type: "tenant",
                        is_primary: true,
                    }],
                },
                // Tenant 3 - rents commercial unit G1 in Central Plaza
                SeedUser {
                    email: "tenant3@demo-property.test",
                    name: "Firma s.r.o.",
                    role_type: "Tenant",
                    phone: Some("+421900444003"),
                    unit_assignments: vec![SeedUnitAssignment {
                        building_index: 2,
                        unit_index: 0, // G1
                        resident_type: "tenant",
                        is_primary: true,
                    }],
                },
                // Resident 1 - family member in unit 1A
                SeedUser {
                    email: "resident1@demo-property.test",
                    name: "Michal Novák",
                    role_type: "Resident",
                    phone: Some("+421900555001"),
                    unit_assignments: vec![SeedUnitAssignment {
                        building_index: 0,
                        unit_index: 0, // 1A
                        resident_type: "family_member",
                        is_primary: false,
                    }],
                },
                // Resident 2 - subtenant in unit 201
                SeedUser {
                    email: "resident2@demo-property.test",
                    name: "Anna Rezidentka",
                    role_type: "Resident",
                    phone: Some("+421900555002"),
                    unit_assignments: vec![SeedUnitAssignment {
                        building_index: 1,
                        unit_index: 2, // 201
                        resident_type: "subtenant",
                        is_primary: false,
                    }],
                },
                // Guest - temporary access
                SeedUser {
                    email: "guest@demo-property.test",
                    name: "Guest User",
                    role_type: "Guest",
                    phone: None,
                    unit_assignments: vec![],
                },
            ],
            default_password: "DemoPass123",
        }
    }
}

impl SeedData {
    /// Create new seed data with custom configuration.
    pub fn new(organization: SeedOrganization) -> Self {
        Self {
            organization,
            buildings: Vec::new(),
            users: Vec::new(),
            default_password: "DemoPass123",
        }
    }

    /// Get total number of units across all buildings.
    pub fn total_units(&self) -> usize {
        self.buildings.iter().map(|b| b.units.len()).sum()
    }
}

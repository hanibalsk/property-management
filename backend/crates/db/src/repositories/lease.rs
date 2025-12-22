//! Lease repository (Epic 19: Lease Management & Tenant Screening).

use crate::models::lease::{
    lease_status, screening_status, ApplicationListQuery, ApplicationSummary, CreateAmendment,
    CreateApplication, CreateLease, CreateLeaseTemplate, CreateReminder, ExpirationOverview,
    InitiateScreening, Lease, LeaseAmendment, LeaseListQuery, LeasePayment, LeaseReminder,
    LeaseStatistics, LeaseSummary, LeaseTemplate, LeaseWithDetails, PaymentSummary, RecordPayment,
    RenewLease, ReviewApplication, ScreeningConsent, ScreeningSummary, SubmitApplication,
    TenantApplication, TenantScreening, TerminateLease, UpdateApplication, UpdateLease,
    UpdateLeaseTemplate, UpdateScreeningResult,
};
use crate::DbPool;
use chrono::{Datelike, Duration, NaiveDate, Utc};
use rust_decimal::Decimal;
use sqlx::Error as SqlxError;
use uuid::Uuid;

/// Repository for lease operations.
#[derive(Clone)]
pub struct LeaseRepository {
    pool: DbPool,
}

impl LeaseRepository {
    /// Create a new LeaseRepository.
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    // ========================================================================
    // Applications (Story 19.1)
    // ========================================================================

    /// Create tenant application.
    pub async fn create_application(
        &self,
        org_id: Uuid,
        data: CreateApplication,
    ) -> Result<TenantApplication, SqlxError> {
        let app = sqlx::query_as::<_, TenantApplication>(
            r#"
            INSERT INTO tenant_applications (
                organization_id, unit_id, applicant_name, applicant_email, applicant_phone,
                date_of_birth, national_id, current_address, current_landlord_name,
                current_landlord_phone, current_rent_amount, current_tenancy_start,
                employer_name, employer_phone, job_title, employment_start, monthly_income,
                desired_move_in, desired_lease_term_months, proposed_rent, co_applicants,
                source, referral_code, status
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22, $23, 'draft')
            RETURNING *
            "#,
        )
        .bind(org_id)
        .bind(data.unit_id)
        .bind(&data.applicant_name)
        .bind(&data.applicant_email)
        .bind(&data.applicant_phone)
        .bind(data.date_of_birth)
        .bind(&data.national_id)
        .bind(&data.current_address)
        .bind(&data.current_landlord_name)
        .bind(&data.current_landlord_phone)
        .bind(data.current_rent_amount)
        .bind(data.current_tenancy_start)
        .bind(&data.employer_name)
        .bind(&data.employer_phone)
        .bind(&data.job_title)
        .bind(data.employment_start)
        .bind(data.monthly_income)
        .bind(data.desired_move_in)
        .bind(data.desired_lease_term_months)
        .bind(data.proposed_rent)
        .bind(&data.co_applicants)
        .bind(&data.source)
        .bind(&data.referral_code)
        .fetch_one(&self.pool)
        .await?;

        Ok(app)
    }

    /// Find application by ID.
    pub async fn find_application_by_id(
        &self,
        id: Uuid,
    ) -> Result<Option<TenantApplication>, SqlxError> {
        let app = sqlx::query_as::<_, TenantApplication>(
            r#"SELECT * FROM tenant_applications WHERE id = $1"#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(app)
    }

    /// Update application.
    pub async fn update_application(
        &self,
        id: Uuid,
        data: UpdateApplication,
    ) -> Result<TenantApplication, SqlxError> {
        let app = sqlx::query_as::<_, TenantApplication>(
            r#"
            UPDATE tenant_applications SET
                applicant_name = COALESCE($2, applicant_name),
                applicant_email = COALESCE($3, applicant_email),
                applicant_phone = COALESCE($4, applicant_phone),
                date_of_birth = COALESCE($5, date_of_birth),
                national_id = COALESCE($6, national_id),
                current_address = COALESCE($7, current_address),
                current_landlord_name = COALESCE($8, current_landlord_name),
                current_landlord_phone = COALESCE($9, current_landlord_phone),
                current_rent_amount = COALESCE($10, current_rent_amount),
                current_tenancy_start = COALESCE($11, current_tenancy_start),
                employer_name = COALESCE($12, employer_name),
                employer_phone = COALESCE($13, employer_phone),
                job_title = COALESCE($14, job_title),
                employment_start = COALESCE($15, employment_start),
                monthly_income = COALESCE($16, monthly_income),
                desired_move_in = COALESCE($17, desired_move_in),
                desired_lease_term_months = COALESCE($18, desired_lease_term_months),
                proposed_rent = COALESCE($19, proposed_rent),
                co_applicants = COALESCE($20, co_applicants),
                updated_at = NOW()
            WHERE id = $1
            RETURNING *
            "#,
        )
        .bind(id)
        .bind(&data.applicant_name)
        .bind(&data.applicant_email)
        .bind(&data.applicant_phone)
        .bind(data.date_of_birth)
        .bind(&data.national_id)
        .bind(&data.current_address)
        .bind(&data.current_landlord_name)
        .bind(&data.current_landlord_phone)
        .bind(data.current_rent_amount)
        .bind(data.current_tenancy_start)
        .bind(&data.employer_name)
        .bind(&data.employer_phone)
        .bind(&data.job_title)
        .bind(data.employment_start)
        .bind(data.monthly_income)
        .bind(data.desired_move_in)
        .bind(data.desired_lease_term_months)
        .bind(data.proposed_rent)
        .bind(&data.co_applicants)
        .fetch_one(&self.pool)
        .await?;

        Ok(app)
    }

    /// Submit application.
    pub async fn submit_application(
        &self,
        id: Uuid,
        data: SubmitApplication,
    ) -> Result<TenantApplication, SqlxError> {
        let app = sqlx::query_as::<_, TenantApplication>(
            r#"
            UPDATE tenant_applications SET
                status = 'submitted',
                documents = COALESCE($2, documents),
                submitted_at = NOW(),
                updated_at = NOW()
            WHERE id = $1 AND status = 'draft'
            RETURNING *
            "#,
        )
        .bind(id)
        .bind(&data.documents)
        .fetch_one(&self.pool)
        .await?;

        Ok(app)
    }

    /// Review application.
    pub async fn review_application(
        &self,
        id: Uuid,
        reviewer_id: Uuid,
        data: ReviewApplication,
    ) -> Result<TenantApplication, SqlxError> {
        let app = sqlx::query_as::<_, TenantApplication>(
            r#"
            UPDATE tenant_applications SET
                status = $2::tenant_application_status,
                reviewed_by = $3,
                reviewed_at = NOW(),
                decision_notes = $4,
                updated_at = NOW()
            WHERE id = $1
            RETURNING *
            "#,
        )
        .bind(id)
        .bind(&data.status)
        .bind(reviewer_id)
        .bind(&data.decision_notes)
        .fetch_one(&self.pool)
        .await?;

        Ok(app)
    }

    /// Delete application.
    pub async fn delete_application(&self, id: Uuid) -> Result<bool, SqlxError> {
        let result = sqlx::query(r#"DELETE FROM tenant_applications WHERE id = $1"#)
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }

    /// List applications for organization.
    pub async fn list_applications(
        &self,
        org_id: Uuid,
        query: ApplicationListQuery,
    ) -> Result<(Vec<ApplicationSummary>, i64), SqlxError> {
        let limit = query.limit.unwrap_or(20);
        let offset = query.offset.unwrap_or(0);

        // Get total count
        let (total,): (i64,) = sqlx::query_as(
            r#"
            SELECT COUNT(*) FROM tenant_applications
            WHERE organization_id = $1
                AND ($2::uuid IS NULL OR unit_id = $2)
                AND ($3::text IS NULL OR status = $3::tenant_application_status)
            "#,
        )
        .bind(org_id)
        .bind(query.unit_id)
        .bind(&query.status)
        .fetch_one(&self.pool)
        .await?;

        // Get applications with summaries
        let apps = sqlx::query_as::<_, (Uuid, Uuid, String, String, String, String, String, Option<chrono::DateTime<Utc>>, Option<Decimal>, Option<NaiveDate>, i64, bool)>(
            r#"
            SELECT
                a.id, a.unit_id, u.name, b.name,
                a.applicant_name, a.applicant_email, a.status,
                a.submitted_at, a.monthly_income, a.desired_move_in,
                (SELECT COUNT(*) FROM tenant_screenings WHERE application_id = a.id) as screening_count,
                COALESCE((SELECT bool_and(passed) FROM tenant_screenings WHERE application_id = a.id AND status = 'completed'), false) as screening_passed
            FROM tenant_applications a
            JOIN units u ON u.id = a.unit_id
            JOIN buildings b ON b.id = u.building_id
            WHERE a.organization_id = $1
                AND ($2::uuid IS NULL OR a.unit_id = $2)
                AND ($3::text IS NULL OR a.status = $3::tenant_application_status)
            ORDER BY a.submitted_at DESC NULLS LAST, a.created_at DESC
            LIMIT $4 OFFSET $5
            "#,
        )
        .bind(org_id)
        .bind(query.unit_id)
        .bind(&query.status)
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await?;

        let summaries = apps
            .into_iter()
            .map(
                |(
                    id,
                    unit_id,
                    unit_name,
                    building_name,
                    applicant_name,
                    applicant_email,
                    status,
                    submitted_at,
                    monthly_income,
                    desired_move_in,
                    screening_count,
                    screening_passed,
                )| {
                    ApplicationSummary {
                        id,
                        unit_id,
                        unit_name,
                        building_name,
                        applicant_name,
                        applicant_email,
                        status,
                        submitted_at,
                        monthly_income,
                        desired_move_in,
                        screening_count,
                        screening_passed,
                    }
                },
            )
            .collect();

        Ok((summaries, total))
    }

    // ========================================================================
    // Screening (Story 19.2)
    // ========================================================================

    /// Initiate screening for application.
    pub async fn initiate_screening(
        &self,
        application_id: Uuid,
        org_id: Uuid,
        data: InitiateScreening,
    ) -> Result<Vec<TenantScreening>, SqlxError> {
        let mut screenings = Vec::new();

        for screening_type in &data.screening_types {
            let screening = sqlx::query_as::<_, TenantScreening>(
                r#"
                INSERT INTO tenant_screenings (
                    application_id, organization_id, screening_type, provider, status, consent_requested_at
                )
                VALUES ($1, $2, $3::screening_type, $4, 'pending_consent', NOW())
                RETURNING *
                "#,
            )
            .bind(application_id)
            .bind(org_id)
            .bind(screening_type)
            .bind(&data.provider)
            .fetch_one(&self.pool)
            .await?;

            screenings.push(screening);
        }

        // Update application status
        sqlx::query(r#"UPDATE tenant_applications SET status = 'screening_pending' WHERE id = $1"#)
            .bind(application_id)
            .execute(&self.pool)
            .await?;

        Ok(screenings)
    }

    /// Submit screening consent.
    pub async fn submit_screening_consent(
        &self,
        id: Uuid,
        data: ScreeningConsent,
    ) -> Result<TenantScreening, SqlxError> {
        let screening = sqlx::query_as::<_, TenantScreening>(
            r#"
            UPDATE tenant_screenings SET
                status = 'consent_received',
                consent_received_at = NOW(),
                consent_document_url = $2,
                updated_at = NOW()
            WHERE id = $1
            RETURNING *
            "#,
        )
        .bind(id)
        .bind(&data.consent_document_url)
        .fetch_one(&self.pool)
        .await?;

        Ok(screening)
    }

    /// Start screening process.
    pub async fn start_screening(&self, id: Uuid) -> Result<TenantScreening, SqlxError> {
        let screening = sqlx::query_as::<_, TenantScreening>(
            r#"
            UPDATE tenant_screenings SET
                status = 'in_progress',
                started_at = NOW(),
                updated_at = NOW()
            WHERE id = $1
            RETURNING *
            "#,
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await?;

        Ok(screening)
    }

    /// Update screening result.
    pub async fn update_screening_result(
        &self,
        id: Uuid,
        data: UpdateScreeningResult,
    ) -> Result<TenantScreening, SqlxError> {
        let status = if data.passed.unwrap_or(false) {
            screening_status::COMPLETED
        } else if data.passed == Some(false) {
            screening_status::FAILED
        } else {
            screening_status::COMPLETED
        };

        let screening = sqlx::query_as::<_, TenantScreening>(
            r#"
            UPDATE tenant_screenings SET
                status = $2::screening_status,
                result_summary = $3,
                risk_score = $4,
                passed = $5,
                flags = $6,
                completed_at = NOW(),
                expires_at = NOW() + INTERVAL '90 days',
                updated_at = NOW()
            WHERE id = $1
            RETURNING *
            "#,
        )
        .bind(id)
        .bind(status)
        .bind(&data.result_summary)
        .bind(data.risk_score)
        .bind(data.passed)
        .bind(&data.flags)
        .fetch_one(&self.pool)
        .await?;

        // Check if all screenings complete and update application
        let (app_id,): (Uuid,) =
            sqlx::query_as(r#"SELECT application_id FROM tenant_screenings WHERE id = $1"#)
                .bind(id)
                .fetch_one(&self.pool)
                .await?;

        let (pending,): (i64,) = sqlx::query_as(
            r#"SELECT COUNT(*) FROM tenant_screenings WHERE application_id = $1 AND status NOT IN ('completed', 'failed', 'expired')"#,
        )
        .bind(app_id)
        .fetch_one(&self.pool)
        .await?;

        if pending == 0 {
            sqlx::query(
                r#"UPDATE tenant_applications SET status = 'screening_complete' WHERE id = $1"#,
            )
            .bind(app_id)
            .execute(&self.pool)
            .await?;
        }

        Ok(screening)
    }

    /// Get screenings for application.
    pub async fn get_screenings_for_application(
        &self,
        application_id: Uuid,
    ) -> Result<Vec<ScreeningSummary>, SqlxError> {
        let screenings = sqlx::query_as::<
            _,
            (
                Uuid,
                String,
                Option<String>,
                String,
                Option<i32>,
                Option<bool>,
                Option<chrono::DateTime<Utc>>,
            ),
        >(
            r#"
            SELECT id, screening_type, provider, status, risk_score, passed, completed_at
            FROM tenant_screenings
            WHERE application_id = $1
            ORDER BY created_at
            "#,
        )
        .bind(application_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(screenings
            .into_iter()
            .map(
                |(id, screening_type, provider, status, risk_score, passed, completed_at)| {
                    ScreeningSummary {
                        id,
                        screening_type,
                        provider,
                        status,
                        risk_score,
                        passed,
                        completed_at,
                    }
                },
            )
            .collect())
    }

    // ========================================================================
    // Lease Templates (Story 19.3)
    // ========================================================================

    /// Create lease template.
    pub async fn create_template(
        &self,
        org_id: Uuid,
        user_id: Uuid,
        data: CreateLeaseTemplate,
    ) -> Result<LeaseTemplate, SqlxError> {
        // If marking as default, unset other defaults first
        if data.is_default == Some(true) {
            sqlx::query(
                r#"UPDATE lease_templates SET is_default = false WHERE organization_id = $1"#,
            )
            .bind(org_id)
            .execute(&self.pool)
            .await?;
        }

        let template = sqlx::query_as::<_, LeaseTemplate>(
            r#"
            INSERT INTO lease_templates (
                organization_id, name, description, content_html, content_variables,
                default_term_months, default_security_deposit_months, default_notice_period_days,
                clauses, is_default, created_by
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            RETURNING *
            "#,
        )
        .bind(org_id)
        .bind(&data.name)
        .bind(&data.description)
        .bind(&data.content_html)
        .bind(&data.content_variables)
        .bind(data.default_term_months)
        .bind(data.default_security_deposit_months)
        .bind(data.default_notice_period_days)
        .bind(&data.clauses)
        .bind(data.is_default.unwrap_or(false))
        .bind(user_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(template)
    }

    /// Find template by ID.
    pub async fn find_template_by_id(&self, id: Uuid) -> Result<Option<LeaseTemplate>, SqlxError> {
        let template =
            sqlx::query_as::<_, LeaseTemplate>(r#"SELECT * FROM lease_templates WHERE id = $1"#)
                .bind(id)
                .fetch_optional(&self.pool)
                .await?;

        Ok(template)
    }

    /// Update template.
    pub async fn update_template(
        &self,
        id: Uuid,
        org_id: Uuid,
        data: UpdateLeaseTemplate,
    ) -> Result<LeaseTemplate, SqlxError> {
        // If marking as default, unset other defaults first
        if data.is_default == Some(true) {
            sqlx::query(
                r#"UPDATE lease_templates SET is_default = false WHERE organization_id = $1 AND id != $2"#,
            )
            .bind(org_id)
            .bind(id)
            .execute(&self.pool)
            .await?;
        }

        let template = sqlx::query_as::<_, LeaseTemplate>(
            r#"
            UPDATE lease_templates SET
                name = COALESCE($2, name),
                description = COALESCE($3, description),
                content_html = COALESCE($4, content_html),
                content_variables = COALESCE($5, content_variables),
                default_term_months = COALESCE($6, default_term_months),
                default_security_deposit_months = COALESCE($7, default_security_deposit_months),
                default_notice_period_days = COALESCE($8, default_notice_period_days),
                clauses = COALESCE($9, clauses),
                is_default = COALESCE($10, is_default),
                is_active = COALESCE($11, is_active),
                version = version + 1,
                updated_at = NOW()
            WHERE id = $1
            RETURNING *
            "#,
        )
        .bind(id)
        .bind(&data.name)
        .bind(&data.description)
        .bind(&data.content_html)
        .bind(&data.content_variables)
        .bind(data.default_term_months)
        .bind(data.default_security_deposit_months)
        .bind(data.default_notice_period_days)
        .bind(&data.clauses)
        .bind(data.is_default)
        .bind(data.is_active)
        .fetch_one(&self.pool)
        .await?;

        Ok(template)
    }

    /// List templates for organization.
    pub async fn list_templates(&self, org_id: Uuid) -> Result<Vec<LeaseTemplate>, SqlxError> {
        let templates = sqlx::query_as::<_, LeaseTemplate>(
            r#"
            SELECT * FROM lease_templates
            WHERE organization_id = $1 AND is_active = true
            ORDER BY is_default DESC, name ASC
            "#,
        )
        .bind(org_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(templates)
    }

    // ========================================================================
    // Leases (Story 19.3, 19.4)
    // ========================================================================

    /// Create lease.
    pub async fn create_lease(
        &self,
        org_id: Uuid,
        user_id: Uuid,
        data: CreateLease,
    ) -> Result<Lease, SqlxError> {
        let lease = sqlx::query_as::<_, Lease>(
            r#"
            INSERT INTO leases (
                organization_id, unit_id, application_id, template_id,
                landlord_user_id, landlord_name, landlord_address,
                tenant_user_id, tenant_name, tenant_email, tenant_phone, occupants,
                start_date, end_date, term_months, is_fixed_term,
                monthly_rent, security_deposit, deposit_held_by, rent_due_day,
                late_fee_amount, late_fee_grace_days,
                utilities_included, parking_spaces, storage_units,
                pets_allowed, pet_deposit, max_occupants, smoking_allowed,
                notes, status, created_by
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22, $23, $24, $25, $26, $27, $28, $29, $30, 'draft', $31)
            RETURNING *
            "#,
        )
        .bind(org_id)
        .bind(data.unit_id)
        .bind(data.application_id)
        .bind(data.template_id)
        .bind(data.landlord_user_id)
        .bind(&data.landlord_name)
        .bind(&data.landlord_address)
        .bind(data.tenant_user_id)
        .bind(&data.tenant_name)
        .bind(&data.tenant_email)
        .bind(&data.tenant_phone)
        .bind(&data.occupants)
        .bind(data.start_date)
        .bind(data.end_date)
        .bind(data.term_months)
        .bind(data.is_fixed_term.unwrap_or(true))
        .bind(data.monthly_rent)
        .bind(data.security_deposit)
        .bind(&data.deposit_held_by)
        .bind(data.rent_due_day.unwrap_or(1))
        .bind(data.late_fee_amount)
        .bind(data.late_fee_grace_days)
        .bind(&data.utilities_included)
        .bind(data.parking_spaces.unwrap_or(0))
        .bind(data.storage_units.unwrap_or(0))
        .bind(data.pets_allowed.unwrap_or(false))
        .bind(data.pet_deposit)
        .bind(data.max_occupants)
        .bind(data.smoking_allowed.unwrap_or(false))
        .bind(&data.notes)
        .bind(user_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(lease)
    }

    /// Find lease by ID.
    pub async fn find_lease_by_id(&self, id: Uuid) -> Result<Option<Lease>, SqlxError> {
        let lease = sqlx::query_as::<_, Lease>(r#"SELECT * FROM leases WHERE id = $1"#)
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(lease)
    }

    /// Update lease.
    pub async fn update_lease(&self, id: Uuid, data: UpdateLease) -> Result<Lease, SqlxError> {
        let lease = sqlx::query_as::<_, Lease>(
            r#"
            UPDATE leases SET
                landlord_address = COALESCE($2, landlord_address),
                tenant_phone = COALESCE($3, tenant_phone),
                occupants = COALESCE($4, occupants),
                utilities_included = COALESCE($5, utilities_included),
                parking_spaces = COALESCE($6, parking_spaces),
                storage_units = COALESCE($7, storage_units),
                pets_allowed = COALESCE($8, pets_allowed),
                pet_deposit = COALESCE($9, pet_deposit),
                max_occupants = COALESCE($10, max_occupants),
                smoking_allowed = COALESCE($11, smoking_allowed),
                notes = COALESCE($12, notes),
                updated_at = NOW()
            WHERE id = $1
            RETURNING *
            "#,
        )
        .bind(id)
        .bind(&data.landlord_address)
        .bind(&data.tenant_phone)
        .bind(&data.occupants)
        .bind(&data.utilities_included)
        .bind(data.parking_spaces)
        .bind(data.storage_units)
        .bind(data.pets_allowed)
        .bind(data.pet_deposit)
        .bind(data.max_occupants)
        .bind(data.smoking_allowed)
        .bind(&data.notes)
        .fetch_one(&self.pool)
        .await?;

        Ok(lease)
    }

    /// Send lease for signature.
    pub async fn send_for_signature(&self, id: Uuid) -> Result<Lease, SqlxError> {
        let lease = sqlx::query_as::<_, Lease>(
            r#"
            UPDATE leases SET
                status = 'pending_signature',
                updated_at = NOW()
            WHERE id = $1 AND status = 'draft'
            RETURNING *
            "#,
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await?;

        Ok(lease)
    }

    /// Record landlord signature.
    pub async fn record_landlord_signature(&self, id: Uuid) -> Result<Lease, SqlxError> {
        let lease = sqlx::query_as::<_, Lease>(
            r#"
            UPDATE leases SET
                landlord_signed_at = NOW(),
                status = CASE
                    WHEN tenant_signed_at IS NOT NULL THEN 'active'::lease_status
                    ELSE status
                END,
                updated_at = NOW()
            WHERE id = $1
            RETURNING *
            "#,
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await?;

        Ok(lease)
    }

    /// Record tenant signature.
    pub async fn record_tenant_signature(&self, id: Uuid) -> Result<Lease, SqlxError> {
        let lease = sqlx::query_as::<_, Lease>(
            r#"
            UPDATE leases SET
                tenant_signed_at = NOW(),
                status = CASE
                    WHEN landlord_signed_at IS NOT NULL THEN 'active'::lease_status
                    ELSE status
                END,
                updated_at = NOW()
            WHERE id = $1
            RETURNING *
            "#,
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await?;

        Ok(lease)
    }

    /// Terminate lease.
    pub async fn terminate_lease(
        &self,
        id: Uuid,
        user_id: Uuid,
        data: TerminateLease,
    ) -> Result<Lease, SqlxError> {
        let lease = sqlx::query_as::<_, Lease>(
            r#"
            UPDATE leases SET
                status = 'terminated',
                terminated_at = COALESCE($4, NOW()),
                termination_reason = $2::termination_reason,
                termination_notes = $3,
                termination_initiated_by = $5,
                updated_at = NOW()
            WHERE id = $1
            RETURNING *
            "#,
        )
        .bind(id)
        .bind(&data.termination_reason)
        .bind(&data.termination_notes)
        .bind(data.effective_date)
        .bind(user_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(lease)
    }

    /// Renew lease.
    pub async fn renew_lease(
        &self,
        id: Uuid,
        user_id: Uuid,
        data: RenewLease,
    ) -> Result<Lease, SqlxError> {
        // Get existing lease (verify it exists)
        let _old_lease = self
            .find_lease_by_id(id)
            .await?
            .ok_or(SqlxError::RowNotFound)?;

        // Create new lease based on old one
        let new_lease = sqlx::query_as::<_, Lease>(
            r#"
            INSERT INTO leases (
                organization_id, unit_id, template_id,
                landlord_user_id, landlord_name, landlord_address,
                tenant_user_id, tenant_name, tenant_email, tenant_phone, occupants,
                start_date, end_date, term_months, is_fixed_term,
                monthly_rent, security_deposit, deposit_held_by, rent_due_day,
                late_fee_amount, late_fee_grace_days,
                utilities_included, parking_spaces, storage_units,
                pets_allowed, pet_deposit, max_occupants, smoking_allowed,
                notes, status, previous_lease_id, created_by
            )
            SELECT
                organization_id, unit_id, template_id,
                landlord_user_id, landlord_name, landlord_address,
                tenant_user_id, tenant_name, tenant_email, tenant_phone, occupants,
                end_date + INTERVAL '1 day', $2, $3, is_fixed_term,
                COALESCE($4, monthly_rent), COALESCE($5, security_deposit), deposit_held_by, rent_due_day,
                late_fee_amount, late_fee_grace_days,
                utilities_included, parking_spaces, storage_units,
                pets_allowed, pet_deposit, max_occupants, smoking_allowed,
                $6, 'draft', id, $7
            FROM leases WHERE id = $1
            RETURNING *
            "#,
        )
        .bind(id)
        .bind(data.new_end_date)
        .bind(data.term_months)
        .bind(data.new_monthly_rent)
        .bind(data.new_security_deposit)
        .bind(&data.notes)
        .bind(user_id)
        .fetch_one(&self.pool)
        .await?;

        // Update old lease
        sqlx::query(
            r#"UPDATE leases SET status = 'renewed', renewed_to_lease_id = $2, updated_at = NOW() WHERE id = $1"#,
        )
        .bind(id)
        .bind(new_lease.id)
        .execute(&self.pool)
        .await?;

        Ok(new_lease)
    }

    /// List leases for organization.
    pub async fn list_leases(
        &self,
        org_id: Uuid,
        query: LeaseListQuery,
    ) -> Result<(Vec<LeaseSummary>, i64), SqlxError> {
        let limit = query.limit.unwrap_or(20);
        let offset = query.offset.unwrap_or(0);
        let today = Utc::now().date_naive();

        // Get total
        let (total,): (i64,) = sqlx::query_as(
            r#"
            SELECT COUNT(*) FROM leases l
            WHERE l.organization_id = $1
                AND ($2::uuid IS NULL OR l.unit_id = $2)
                AND ($3::uuid IS NULL OR l.tenant_user_id = $3)
                AND ($4::text IS NULL OR l.status = $4::lease_status)
                AND ($5::int IS NULL OR l.end_date <= $6::date + ($5 || ' days')::interval)
            "#,
        )
        .bind(org_id)
        .bind(query.unit_id)
        .bind(query.tenant_id)
        .bind(&query.status)
        .bind(query.expiring_within_days)
        .bind(today)
        .fetch_one(&self.pool)
        .await?;

        // Get leases
        let leases = sqlx::query_as::<
            _,
            (
                Uuid,
                Uuid,
                String,
                String,
                String,
                String,
                NaiveDate,
                NaiveDate,
                Decimal,
                String,
                i64,
            ),
        >(
            r#"
            SELECT
                l.id, l.unit_id, u.name, b.name,
                l.tenant_name, l.tenant_email,
                l.start_date, l.end_date, l.monthly_rent, l.status,
                (l.end_date - $6::date)::int8 as days_until_expiry
            FROM leases l
            JOIN units u ON u.id = l.unit_id
            JOIN buildings b ON b.id = u.building_id
            WHERE l.organization_id = $1
                AND ($2::uuid IS NULL OR l.unit_id = $2)
                AND ($3::uuid IS NULL OR l.tenant_user_id = $3)
                AND ($4::text IS NULL OR l.status = $4::lease_status)
                AND ($5::int IS NULL OR l.end_date <= $6::date + ($5 || ' days')::interval)
            ORDER BY l.end_date ASC
            LIMIT $7 OFFSET $8
            "#,
        )
        .bind(org_id)
        .bind(query.unit_id)
        .bind(query.tenant_id)
        .bind(&query.status)
        .bind(query.expiring_within_days)
        .bind(today)
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await?;

        let summaries = leases
            .into_iter()
            .map(
                |(
                    id,
                    unit_id,
                    unit_name,
                    building_name,
                    tenant_name,
                    tenant_email,
                    start_date,
                    end_date,
                    monthly_rent,
                    status,
                    days_until_expiry,
                )| {
                    LeaseSummary {
                        id,
                        unit_id,
                        unit_name,
                        building_name,
                        tenant_name,
                        tenant_email,
                        start_date,
                        end_date,
                        monthly_rent,
                        status,
                        days_until_expiry,
                    }
                },
            )
            .collect();

        Ok((summaries, total))
    }

    /// Get lease with full details.
    pub async fn get_lease_with_details(
        &self,
        id: Uuid,
    ) -> Result<Option<LeaseWithDetails>, SqlxError> {
        let lease = match self.find_lease_by_id(id).await? {
            Some(l) => l,
            None => return Ok(None),
        };

        // Get unit and building names
        let (unit_name, building_name): (String, String) = sqlx::query_as(
            r#"SELECT u.name, b.name FROM units u JOIN buildings b ON b.id = u.building_id WHERE u.id = $1"#,
        )
        .bind(lease.unit_id)
        .fetch_one(&self.pool)
        .await?;

        // Get amendments
        let amendments = sqlx::query_as::<_, LeaseAmendment>(
            r#"SELECT * FROM lease_amendments WHERE lease_id = $1 ORDER BY amendment_number"#,
        )
        .bind(id)
        .fetch_all(&self.pool)
        .await?;

        // Get upcoming payments
        let today = Utc::now().date_naive();
        let upcoming_payments = sqlx::query_as::<_, LeasePayment>(
            r#"SELECT * FROM lease_payments WHERE lease_id = $1 AND due_date >= $2 ORDER BY due_date LIMIT 5"#,
        )
        .bind(id)
        .bind(today)
        .fetch_all(&self.pool)
        .await?;

        // Get reminders
        let reminders = sqlx::query_as::<_, LeaseReminder>(
            r#"SELECT * FROM lease_reminders WHERE lease_id = $1 AND sent_at IS NULL ORDER BY trigger_date"#,
        )
        .bind(id)
        .fetch_all(&self.pool)
        .await?;

        Ok(Some(LeaseWithDetails {
            lease,
            unit_name,
            building_name,
            amendments,
            upcoming_payments,
            reminders,
        }))
    }

    // ========================================================================
    // Amendments
    // ========================================================================

    /// Create lease amendment.
    pub async fn create_amendment(
        &self,
        lease_id: Uuid,
        user_id: Uuid,
        data: CreateAmendment,
    ) -> Result<LeaseAmendment, SqlxError> {
        // Get next amendment number
        let (next_num,): (i64,) = sqlx::query_as(
            r#"SELECT COALESCE(MAX(amendment_number), 0) + 1 FROM lease_amendments WHERE lease_id = $1"#,
        )
        .bind(lease_id)
        .fetch_one(&self.pool)
        .await?;

        let amendment = sqlx::query_as::<_, LeaseAmendment>(
            r#"
            INSERT INTO lease_amendments (
                lease_id, amendment_number, title, description, changes, effective_date, created_by
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING *
            "#,
        )
        .bind(lease_id)
        .bind(next_num as i32)
        .bind(&data.title)
        .bind(&data.description)
        .bind(&data.changes)
        .bind(data.effective_date)
        .bind(user_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(amendment)
    }

    // ========================================================================
    // Payments
    // ========================================================================

    /// Generate payment schedule for lease.
    pub async fn generate_payment_schedule(
        &self,
        lease_id: Uuid,
    ) -> Result<Vec<LeasePayment>, SqlxError> {
        let lease = self
            .find_lease_by_id(lease_id)
            .await?
            .ok_or_else(|| SqlxError::RowNotFound)?;

        let mut payments = Vec::new();
        let mut current_date = lease.start_date;

        // Generate monthly rent payments
        while current_date < lease.end_date {
            let due_date = NaiveDate::from_ymd_opt(
                current_date.year(),
                current_date.month(),
                lease.rent_due_day as u32,
            )
            .unwrap_or(current_date);

            let payment = sqlx::query_as::<_, LeasePayment>(
                r#"
                INSERT INTO lease_payments (lease_id, organization_id, due_date, amount, payment_type, description)
                VALUES ($1, $2, $3, $4, 'rent', 'Monthly rent')
                ON CONFLICT DO NOTHING
                RETURNING *
                "#,
            )
            .bind(lease_id)
            .bind(lease.organization_id)
            .bind(due_date)
            .bind(lease.monthly_rent)
            .fetch_optional(&self.pool)
            .await?;

            if let Some(p) = payment {
                payments.push(p);
            }

            // Move to next month
            current_date += Duration::days(32);
            current_date = NaiveDate::from_ymd_opt(current_date.year(), current_date.month(), 1)
                .unwrap_or(current_date);
        }

        Ok(payments)
    }

    /// Record payment.
    pub async fn record_payment(
        &self,
        id: Uuid,
        data: RecordPayment,
    ) -> Result<LeasePayment, SqlxError> {
        let payment = sqlx::query_as::<_, LeasePayment>(
            r#"
            UPDATE lease_payments SET
                paid_at = COALESCE($2, NOW()),
                paid_amount = $3,
                payment_method = $4,
                payment_reference = $5,
                updated_at = NOW()
            WHERE id = $1
            RETURNING *
            "#,
        )
        .bind(id)
        .bind(data.paid_at)
        .bind(data.paid_amount)
        .bind(&data.payment_method)
        .bind(&data.payment_reference)
        .fetch_one(&self.pool)
        .await?;

        Ok(payment)
    }

    /// Get overdue payments.
    pub async fn get_overdue_payments(
        &self,
        org_id: Uuid,
    ) -> Result<Vec<PaymentSummary>, SqlxError> {
        let today = Utc::now().date_naive();

        let payments = sqlx::query_as::<_, (Uuid, NaiveDate, Decimal, String, Option<chrono::DateTime<Utc>>, Option<Decimal>, bool, Option<Decimal>)>(
            r#"
            SELECT id, due_date, amount, payment_type, paid_at, paid_amount, is_late, late_fee_applied
            FROM lease_payments
            WHERE organization_id = $1 AND due_date < $2 AND paid_at IS NULL
            ORDER BY due_date
            "#,
        )
        .bind(org_id)
        .bind(today)
        .fetch_all(&self.pool)
        .await?;

        Ok(payments
            .into_iter()
            .map(
                |(
                    id,
                    due_date,
                    amount,
                    payment_type,
                    paid_at,
                    paid_amount,
                    is_late,
                    late_fee_applied,
                )| {
                    PaymentSummary {
                        id,
                        due_date,
                        amount,
                        payment_type,
                        paid_at,
                        paid_amount,
                        is_late,
                        late_fee_applied,
                    }
                },
            )
            .collect())
    }

    // ========================================================================
    // Reminders (Story 19.5)
    // ========================================================================

    /// Create reminder.
    pub async fn create_reminder(
        &self,
        lease_id: Uuid,
        data: CreateReminder,
    ) -> Result<LeaseReminder, SqlxError> {
        let reminder = sqlx::query_as::<_, LeaseReminder>(
            r#"
            INSERT INTO lease_reminders (
                lease_id, reminder_type, trigger_date, days_before_event,
                subject, message, recipients, is_recurring, recurrence_pattern
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            RETURNING *
            "#,
        )
        .bind(lease_id)
        .bind(&data.reminder_type)
        .bind(data.trigger_date)
        .bind(data.days_before_event)
        .bind(&data.subject)
        .bind(&data.message)
        .bind(&data.recipients)
        .bind(data.is_recurring.unwrap_or(false))
        .bind(&data.recurrence_pattern)
        .fetch_one(&self.pool)
        .await?;

        Ok(reminder)
    }

    /// Get expiration overview.
    pub async fn get_expiration_overview(
        &self,
        org_id: Uuid,
    ) -> Result<ExpirationOverview, SqlxError> {
        let today = Utc::now().date_naive();
        let day_30 = today + Duration::days(30);
        let day_60 = today + Duration::days(60);
        let day_90 = today + Duration::days(90);

        // Get leases expiring in each period
        let get_expiring = |start: NaiveDate, end: NaiveDate| async move {
            self.list_leases(
                org_id,
                LeaseListQuery {
                    status: Some(lease_status::ACTIVE.to_string()),
                    expiring_within_days: Some((end - today).num_days() as i32),
                    ..Default::default()
                },
            )
            .await
            .map(|(leases, _)| {
                leases
                    .into_iter()
                    .filter(|l| l.end_date >= start && l.end_date <= end)
                    .collect::<Vec<_>>()
            })
        };

        let expiring_30_days = get_expiring(today, day_30).await?;
        let expiring_60_days = get_expiring(day_30, day_60).await?;
        let expiring_90_days = get_expiring(day_60, day_90).await?;

        let (total_active_leases,): (i64,) = sqlx::query_as(
            r#"SELECT COUNT(*) FROM leases WHERE organization_id = $1 AND status = 'active'"#,
        )
        .bind(org_id)
        .fetch_one(&self.pool)
        .await?;

        let total_expiring_soon =
            (expiring_30_days.len() + expiring_60_days.len() + expiring_90_days.len()) as i64;

        Ok(ExpirationOverview {
            expiring_30_days,
            expiring_60_days,
            expiring_90_days,
            total_active_leases,
            total_expiring_soon,
        })
    }

    // ========================================================================
    // Statistics
    // ========================================================================

    /// Get lease statistics.
    pub async fn get_statistics(&self, org_id: Uuid) -> Result<LeaseStatistics, SqlxError> {
        let today = Utc::now().date_naive();
        let day_90 = today + Duration::days(90);

        let (total_leases,): (i64,) =
            sqlx::query_as(r#"SELECT COUNT(*) FROM leases WHERE organization_id = $1"#)
                .bind(org_id)
                .fetch_one(&self.pool)
                .await?;

        let (active_leases,): (i64,) = sqlx::query_as(
            r#"SELECT COUNT(*) FROM leases WHERE organization_id = $1 AND status = 'active'"#,
        )
        .bind(org_id)
        .fetch_one(&self.pool)
        .await?;

        let (pending_signatures,): (i64,) = sqlx::query_as(
            r#"SELECT COUNT(*) FROM leases WHERE organization_id = $1 AND status = 'pending_signature'"#,
        )
        .bind(org_id)
        .fetch_one(&self.pool)
        .await?;

        let (expiring_soon,): (i64,) = sqlx::query_as(
            r#"SELECT COUNT(*) FROM leases WHERE organization_id = $1 AND status = 'active' AND end_date <= $2"#,
        )
        .bind(org_id)
        .bind(day_90)
        .fetch_one(&self.pool)
        .await?;

        let (total_applications,): (i64,) = sqlx::query_as(
            r#"SELECT COUNT(*) FROM tenant_applications WHERE organization_id = $1"#,
        )
        .bind(org_id)
        .fetch_one(&self.pool)
        .await?;

        let (pending_applications,): (i64,) = sqlx::query_as(
            r#"SELECT COUNT(*) FROM tenant_applications WHERE organization_id = $1 AND status IN ('submitted', 'under_review', 'screening_pending')"#,
        )
        .bind(org_id)
        .fetch_one(&self.pool)
        .await?;

        let (total_monthly_rent,): (Option<Decimal>,) = sqlx::query_as(
            r#"SELECT COALESCE(SUM(monthly_rent), 0) FROM leases WHERE organization_id = $1 AND status = 'active'"#,
        )
        .bind(org_id)
        .fetch_one(&self.pool)
        .await?;

        // Calculate occupancy rate
        let (total_units,): (i64,) =
            sqlx::query_as(r#"SELECT COUNT(*) FROM units WHERE organization_id = $1"#)
                .bind(org_id)
                .fetch_one(&self.pool)
                .await?;

        let (occupied_units,): (i64,) = sqlx::query_as(
            r#"SELECT COUNT(DISTINCT unit_id) FROM leases WHERE organization_id = $1 AND status = 'active'"#,
        )
        .bind(org_id)
        .fetch_one(&self.pool)
        .await?;

        let occupancy_rate = if total_units > 0 {
            (occupied_units as f64 / total_units as f64) * 100.0
        } else {
            0.0
        };

        Ok(LeaseStatistics {
            total_leases,
            active_leases,
            pending_signatures,
            expiring_soon,
            total_applications,
            pending_applications,
            total_monthly_rent: total_monthly_rent.unwrap_or_default(),
            occupancy_rate,
        })
    }
}

impl Default for LeaseListQuery {
    fn default() -> Self {
        Self {
            unit_id: None,
            tenant_id: None,
            status: None,
            expiring_within_days: None,
            limit: Some(20),
            offset: Some(0),
        }
    }
}

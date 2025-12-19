package com.propertymanagement.models

import kotlinx.serialization.Serializable

/**
 * Shared models for Property Management.
 *
 * These will be generated from OpenAPI spec.
 */

@Serializable
data class User(
    val id: String,
    val email: String,
    val displayName: String,
    val avatarUrl: String? = null
)

@Serializable
data class TenantContext(
    val tenantId: String,
    val tenantName: String,
    val role: String
)

@Serializable
data class LoginRequest(
    val email: String,
    val password: String,
    val twoFactorCode: String? = null
)

@Serializable
data class LoginResponse(
    val accessToken: String,
    val refreshToken: String,
    val expiresIn: Int,
    val user: User,
    val tenants: List<TenantMembership>
)

@Serializable
data class TenantMembership(
    val tenantId: String,
    val tenantName: String,
    val role: String
)

@Serializable
data class ErrorResponse(
    val code: String,
    val message: String,
    val requestId: String? = null,
    val timestamp: String
)

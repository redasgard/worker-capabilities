//! Type definitions for worker capabilities

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::constants::*;

/// Capability attestation for cryptographic verification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityAttestation {
    /// Hash of the capability claims
    pub capability_hash: String,
    /// Digital signature of the hash
    pub signature: String,
    /// Public key used for verification
    pub public_key: String,
    /// Timestamp when attested
    pub timestamp: u64,
    /// Attestation algorithm used
    pub algorithm: String,
    /// Attester identity
    pub attester: String,
}

/// Capability permissions and boundaries
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityPermissions {
    /// Can access filesystem
    pub filesystem_access: bool,
    /// Can make network calls
    pub network_access: bool,
    /// Can spawn processes
    pub process_spawn: bool,
    /// Can access environment variables
    pub env_access: bool,
    /// Can access system resources
    pub system_access: bool,
    /// Maximum memory usage in MB
    pub memory_limit_mb: u64,
    /// Maximum CPU usage percentage
    pub cpu_limit_percent: u8,
    /// Maximum execution time in seconds
    pub timeout_seconds: u64,
}

impl Default for CapabilityPermissions {
    fn default() -> Self {
        Self {
            filesystem_access: false,
            network_access: false,
            process_spawn: false,
            env_access: false,
            system_access: false,
            memory_limit_mb: DEFAULT_MEMORY_LIMIT_MB,
            cpu_limit_percent: DEFAULT_CPU_LIMIT_PERCENT,
            timeout_seconds: DEFAULT_TIMEOUT_SECONDS,
        }
    }
}

/// Capability expiration and revocation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityExpiration {
    /// Expiration timestamp
    pub expires_at: u64,
    /// Whether capability is revoked
    pub revoked: bool,
    /// Revocation reason (if revoked)
    pub revocation_reason: Option<String>,
    /// Revocation timestamp (if revoked)
    pub revoked_at: Option<u64>,
    /// Revoker identity (if revoked)
    pub revoked_by: Option<String>,
}

impl Default for CapabilityExpiration {
    fn default() -> Self {
        Self {
            expires_at: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs() + DEFAULT_EXPIRATION_HOURS * 60 * 60, // 24 hours from now
            revoked: false,
            revocation_reason: None,
            revoked_at: None,
            revoked_by: None,
        }
    }
}

/// Tool capability definition with security features
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCapability {
    /// Name of the tool
    pub tool_name: String,
    /// Whether this tool is required (vs optional)
    pub required: bool,
    /// Alternative tools that can substitute
    pub alternatives: Vec<String>,
    /// Capability attestation
    pub attestation: Option<CapabilityAttestation>,
    /// Capability permissions
    pub permissions: CapabilityPermissions,
    /// Capability expiration
    pub expiration: CapabilityExpiration,
    /// Whether capability is verified
    pub verified: bool,
}

impl ToolCapability {
    /// Create a new tool capability
    pub fn new(tool_name: impl Into<String>, required: bool) -> Self {
        Self {
            tool_name: tool_name.into(),
            required,
            alternatives: Vec::new(),
            attestation: None,
            permissions: CapabilityPermissions::default(),
            expiration: CapabilityExpiration::default(),
            verified: false,
        }
    }

    /// Create a new secure tool capability
    pub fn new_secure(
        tool_name: impl Into<String>,
        required: bool,
        permissions: CapabilityPermissions,
        expiration: CapabilityExpiration,
    ) -> Self {
        Self {
            tool_name: tool_name.into(),
            required,
            alternatives: Vec::new(),
            attestation: None,
            permissions,
            expiration,
            verified: false,
        }
    }

    /// Add alternative tools
    pub fn with_alternatives(mut self, alternatives: Vec<String>) -> Self {
        self.alternatives = alternatives;
        self
    }

    /// Add attestation to capability
    pub fn with_attestation(mut self, attestation: CapabilityAttestation) -> Self {
        self.attestation = Some(attestation);
        self.verified = true;
        self
    }

    /// Set permissions
    pub fn with_permissions(mut self, permissions: CapabilityPermissions) -> Self {
        self.permissions = permissions;
        self
    }

    /// Set expiration
    pub fn with_expiration(mut self, expiration: CapabilityExpiration) -> Self {
        self.expiration = expiration;
        self
    }

    /// Check if this capability is satisfied
    pub fn is_satisfied(&self, tool_checker: &dyn Fn(&str) -> bool) -> bool {
        // Check if capability is expired
        if self.is_expired() {
            return false;
        }

        // Check if capability is revoked
        if self.is_revoked() {
            return false;
        }

        // Check primary tool
        if tool_checker(&self.tool_name) {
            return true;
        }

        // Check alternatives
        self.alternatives.iter().any(|alt| tool_checker(alt))
    }

    /// Check if capability is expired
    pub fn is_expired(&self) -> bool {
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        current_time > self.expiration.expires_at
    }

    /// Check if capability is revoked
    pub fn is_revoked(&self) -> bool {
        self.expiration.revoked
    }

    /// Verify capability attestation
    pub fn verify_attestation(&self) -> bool {
        match &self.attestation {
            Some(attestation) => {
                // Check if attestation is not expired
                let current_time = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs();
                
                if current_time - attestation.timestamp > ATTESTATION_EXPIRY_DAYS * 24 * 60 * 60 {
                    return false; // Attestation expired
                }

                // Verify attestation algorithm
                if attestation.algorithm != DEFAULT_ATTESTATION_ALGORITHM {
                    return false;
                }

                // In a real implementation, verify the actual signature
                // For now, just check that attestation exists and is not empty
                !attestation.signature.is_empty() && !attestation.public_key.is_empty()
            }
            None => false, // No attestation means not verified
        }
    }

    /// Check if capability has required permissions
    pub fn has_permission(&self, permission: &str) -> bool {
        match permission {
            PERMISSION_FILESYSTEM_ACCESS => self.permissions.filesystem_access,
            PERMISSION_NETWORK_ACCESS => self.permissions.network_access,
            PERMISSION_PROCESS_SPAWN => self.permissions.process_spawn,
            PERMISSION_ENV_ACCESS => self.permissions.env_access,
            PERMISSION_SYSTEM_ACCESS => self.permissions.system_access,
            _ => false,
        }
    }

    /// Revoke capability
    pub fn revoke(&mut self, reason: String, revoked_by: String) {
        self.expiration.revoked = true;
        self.expiration.revocation_reason = Some(reason);
        self.expiration.revoked_at = Some(
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs()
        );
        self.expiration.revoked_by = Some(revoked_by);
    }
}

/// Security report for a capability
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilitySecurityReport {
    pub tool_name: String,
    pub has_attestation: bool,
    pub attestation_verified: bool,
    pub is_expired: bool,
    pub is_revoked: bool,
    pub permissions: CapabilityPermissions,
    pub expiration: CapabilityExpiration,
}

//! Capability attestation and verification functionality

use sha2::{Sha256, Digest};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::constants::*;
use crate::types::{ToolCapability, CapabilityAttestation};

impl ToolCapability {
    /// Generate capability hash for attestation
    pub fn generate_capability_hash(&self) -> String {
        let mut hasher = Sha256::new();
        hasher.update(self.tool_name.as_bytes());
        hasher.update(self.required.to_string().as_bytes());
        hasher.update(self.alternatives.join(",").as_bytes());
        hasher.update(self.permissions.filesystem_access.to_string().as_bytes());
        hasher.update(self.permissions.network_access.to_string().as_bytes());
        hasher.update(self.permissions.process_spawn.to_string().as_bytes());
        hasher.update(self.permissions.env_access.to_string().as_bytes());
        hasher.update(self.permissions.system_access.to_string().as_bytes());
        hasher.update(self.permissions.memory_limit_mb.to_string().as_bytes());
        hasher.update(self.permissions.cpu_limit_percent.to_string().as_bytes());
        hasher.update(self.permissions.timeout_seconds.to_string().as_bytes());
        hasher.update(self.expiration.expires_at.to_string().as_bytes());
        
        format!("{:x}", hasher.finalize())
    }

    /// Create attestation for this capability
    pub fn create_attestation(&self, signer_private_key: &str, attester: String) -> CapabilityAttestation {
        let capability_hash = self.generate_capability_hash();
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        // In a real implementation, use proper cryptographic signing
        // For now, create a mock signature
        let signature = format!("signature_{}_{}", capability_hash, timestamp);
        let public_key = format!("pubkey_{}", signer_private_key);

        CapabilityAttestation {
            capability_hash,
            signature,
            public_key,
            timestamp,
            algorithm: DEFAULT_ATTESTATION_ALGORITHM.to_string(),
            attester,
        }
    }

    /// Verify the capability hash matches the attestation
    pub fn verify_capability_hash(&self) -> bool {
        if let Some(attestation) = &self.attestation {
            let current_hash = self.generate_capability_hash();
            current_hash == attestation.capability_hash
        } else {
            false
        }
    }

    /// Check if attestation is valid and not tampered with
    pub fn verify_attestation_integrity(&self) -> bool {
        if let Some(attestation) = &self.attestation {
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

            // Verify capability hash matches
            if !self.verify_capability_hash() {
                return false;
            }

            // In a real implementation, verify the actual signature
            // For now, just check that attestation exists and is not empty
            !attestation.signature.is_empty() && !attestation.public_key.is_empty()
        } else {
            false // No attestation means not verified
        }
    }
}

/// Attestation manager for handling multiple attestations
pub struct AttestationManager {
    /// Map of tool names to their attestations
    attestations: std::collections::HashMap<String, CapabilityAttestation>,
}

impl AttestationManager {
    /// Create a new attestation manager
    pub fn new() -> Self {
        Self {
            attestations: std::collections::HashMap::new(),
        }
    }

    /// Add an attestation for a tool
    pub fn add_attestation(&mut self, tool_name: String, attestation: CapabilityAttestation) {
        self.attestations.insert(tool_name, attestation);
    }

    /// Get attestation for a tool
    pub fn get_attestation(&self, tool_name: &str) -> Option<&CapabilityAttestation> {
        self.attestations.get(tool_name)
    }

    /// Verify all attestations are valid
    pub fn verify_all_attestations(&self) -> bool {
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        for attestation in self.attestations.values() {
            // Check if attestation is not expired
            if current_time - attestation.timestamp > ATTESTATION_EXPIRY_DAYS * 24 * 60 * 60 {
                return false;
            }

            // Verify attestation algorithm
            if attestation.algorithm != DEFAULT_ATTESTATION_ALGORITHM {
                return false;
            }

            // Check that signature and public key are not empty
            if attestation.signature.is_empty() || attestation.public_key.is_empty() {
                return false;
            }
        }

        true
    }

    /// Get expired attestations
    pub fn get_expired_attestations(&self) -> Vec<String> {
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let mut expired = Vec::new();
        for (tool_name, attestation) in &self.attestations {
            if current_time - attestation.timestamp > ATTESTATION_EXPIRY_DAYS * 24 * 60 * 60 {
                expired.push(tool_name.clone());
            }
        }

        expired
    }

    /// Remove attestation for a tool
    pub fn remove_attestation(&mut self, tool_name: &str) -> Option<CapabilityAttestation> {
        self.attestations.remove(tool_name)
    }

    /// Clear all attestations
    pub fn clear_all(&mut self) {
        self.attestations.clear();
    }

    /// Get count of attestations
    pub fn count(&self) -> usize {
        self.attestations.len()
    }
}

impl Default for AttestationManager {
    fn default() -> Self {
        Self::new()
    }
}

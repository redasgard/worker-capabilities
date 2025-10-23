//! Registry for managing multiple capability sets

use std::collections::HashMap;

use crate::constants::*;
use crate::types::{CapabilitySecurityReport};
use crate::capabilities::Capabilities;

/// Registry for managing multiple capability sets
#[derive(Debug, Default)]
pub struct CapabilityRegistry {
    capabilities: HashMap<String, Capabilities>,
}

impl CapabilityRegistry {
    /// Create a new empty registry
    pub fn new() -> Self {
        Self {
            capabilities: HashMap::new(),
        }
    }

    /// Register a capability set
    pub fn register(&mut self, caps: Capabilities) {
        self.capabilities.insert(caps.id.clone(), caps);
    }

    /// Get capabilities by ID
    pub fn get(&self, id: &str) -> Option<&Capabilities> {
        self.capabilities.get(id)
    }

    /// Get mutable capabilities by ID
    pub fn get_mut(&mut self, id: &str) -> Option<&mut Capabilities> {
        self.capabilities.get_mut(id)
    }

    /// List all registered capability IDs
    pub fn list_ids(&self) -> Vec<String> {
        self.capabilities.keys().cloned().collect()
    }

    /// Find workers with a specific capability
    pub fn find_with_capability(
        &self,
        capability_type: &str,
        tool_checker: &dyn Fn(&str) -> bool,
    ) -> Vec<&Capabilities> {
        self.capabilities
            .values()
            .filter(|caps| caps.has_capability(capability_type, tool_checker))
            .collect()
    }

    /// Find workers with verified capabilities
    pub fn find_verified_workers(&self) -> Vec<&Capabilities> {
        self.capabilities
            .values()
            .filter(|caps| caps.verify_all_capabilities())
            .collect()
    }

    /// Revoke capabilities for a specific worker
    pub fn revoke_worker_capabilities(
        &mut self,
        worker_id: &str,
        reason: String,
        revoked_by: String,
    ) -> bool {
        if let Some(caps) = self.capabilities.get_mut(worker_id) {
            caps.revoke_all_capabilities(reason, revoked_by);
            true
        } else {
            false
        }
    }

    /// Get security report for all workers
    pub fn get_security_report(&self) -> HashMap<String, HashMap<String, CapabilitySecurityReport>> {
        let mut report = HashMap::new();
        
        for (worker_id, capabilities) in &self.capabilities {
            report.insert(worker_id.clone(), capabilities.get_security_report());
        }
        
        report
    }

    /// Verify all workers have valid capabilities
    pub fn verify_all_workers(&self) -> HashMap<String, bool> {
        let mut results = HashMap::new();
        
        for (worker_id, capabilities) in &self.capabilities {
            results.insert(worker_id.clone(), capabilities.verify_all_capabilities());
        }
        
        results
    }

    /// Find workers with specific permissions
    pub fn find_workers_with_permissions(
        &self,
        capability_type: &str,
        required_permission: &str,
    ) -> Vec<&Capabilities> {
        self.capabilities
            .values()
            .filter(|caps| caps.has_required_permissions(capability_type, required_permission))
            .collect()
    }

    /// Get registry statistics
    pub fn get_statistics(&self) -> RegistryStatistics {
        let total_workers = self.capabilities.len();
        let verified_workers = self.find_verified_workers().len();
        
        let mut total_tools = 0;
        let mut total_required_tools = 0;
        let mut total_verified_tools = 0;
        
        for capabilities in self.capabilities.values() {
            let stats = capabilities.get_statistics();
            total_tools += stats.total_tools;
            total_required_tools += stats.required_tools;
            total_verified_tools += stats.verified_tools;
        }

        RegistryStatistics {
            total_workers,
            verified_workers,
            total_tools,
            total_required_tools,
            total_verified_tools,
        }
    }

    /// Remove a worker from the registry
    pub fn remove_worker(&mut self, worker_id: &str) -> Option<Capabilities> {
        self.capabilities.remove(worker_id)
    }

    /// Clear all workers from the registry
    pub fn clear_all(&mut self) {
        self.capabilities.clear();
    }

    /// Check if a worker is registered
    pub fn contains_worker(&self, worker_id: &str) -> bool {
        self.capabilities.contains_key(worker_id)
    }

    /// Get all workers with a specific flag
    pub fn find_workers_with_flag(&self, flag: &str) -> Vec<&Capabilities> {
        self.capabilities
            .values()
            .filter(|caps| caps.has_flag(flag))
            .collect()
    }

    /// Get workers with metadata matching a key-value pair
    pub fn find_workers_with_metadata(&self, key: &str, value: &str) -> Vec<&Capabilities> {
        self.capabilities
            .values()
            .filter(|caps| caps.get_metadata(key) == Some(&value.to_string()))
            .collect()
    }

    /// Get all unique tool names across all workers
    pub fn get_all_tool_names(&self) -> std::collections::HashSet<String> {
        let mut tool_names = std::collections::HashSet::new();
        
        for capabilities in self.capabilities.values() {
            for tool_name in capabilities.all_tools() {
                tool_names.insert(tool_name);
            }
        }
        
        tool_names
    }

    /// Find workers that have all required tools available
    pub fn find_workers_with_all_required_tools(&self, tool_checker: &dyn Fn(&str) -> bool) -> Vec<&Capabilities> {
        self.capabilities
            .values()
            .filter(|caps| caps.has_all_required_tools(tool_checker))
            .collect()
    }
}

/// Statistics about the registry
#[derive(Debug, Clone)]
pub struct RegistryStatistics {
    pub total_workers: usize,
    pub verified_workers: usize,
    pub total_tools: usize,
    pub total_required_tools: usize,
    pub total_verified_tools: usize,
}

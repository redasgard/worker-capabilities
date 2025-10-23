//! Capabilities management for workers

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::constants::*;
use crate::types::{ToolCapability, CapabilityPermissions, CapabilityExpiration, CapabilitySecurityReport};

/// Capabilities for a worker or component
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Capabilities {
    /// Identifier for this capability set
    pub id: String,

    /// Static analysis tools
    pub static_analysis_tools: Vec<ToolCapability>,

    /// Security scanning tools
    pub security_scanning_tools: Vec<ToolCapability>,

    /// Dynamic analysis tools
    pub dynamic_analysis_tools: Vec<ToolCapability>,

    /// Fuzzing tools
    pub fuzzing_tools: Vec<ToolCapability>,

    /// Test framework tools
    pub test_framework_tools: Vec<ToolCapability>,

    /// Additional capability flags
    pub flags: HashMap<String, bool>,

    /// Custom metadata
    pub metadata: HashMap<String, String>,
}

impl Capabilities {
    /// Create a new capability set
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            static_analysis_tools: Vec::new(),
            security_scanning_tools: Vec::new(),
            dynamic_analysis_tools: Vec::new(),
            fuzzing_tools: Vec::new(),
            test_framework_tools: Vec::new(),
            flags: HashMap::new(),
            metadata: HashMap::new(),
        }
    }

    /// Add a static analysis tool
    pub fn with_static_analysis(mut self, tool: impl Into<String>, required: bool) -> Self {
        self.static_analysis_tools
            .push(ToolCapability::new(tool, required));
        self
    }

    /// Add a security scanning tool
    pub fn with_security_tool(mut self, tool: impl Into<String>, required: bool) -> Self {
        self.security_scanning_tools
            .push(ToolCapability::new(tool, required));
        self
    }

    /// Add a dynamic analysis tool
    pub fn with_dynamic_tool(mut self, tool: impl Into<String>, required: bool) -> Self {
        self.dynamic_analysis_tools
            .push(ToolCapability::new(tool, required));
        self
    }

    /// Add a fuzzing tool
    pub fn with_fuzzing_tool(mut self, tool: impl Into<String>, required: bool) -> Self {
        self.fuzzing_tools
            .push(ToolCapability::new(tool, required));
        self
    }

    /// Add a test framework tool
    pub fn with_test_framework(mut self, tool: impl Into<String>, required: bool) -> Self {
        self.test_framework_tools
            .push(ToolCapability::new(tool, required));
        self
    }

    /// Add a generic tool to any category
    pub fn with_tool(mut self, tool: impl Into<String>, required: bool) -> Self {
        self.static_analysis_tools
            .push(ToolCapability::new(tool, required));
        self
    }

    /// Add a tool with alternatives
    pub fn with_alternative(
        mut self,
        tool: impl Into<String>,
        alternatives: Vec<impl Into<String>>,
    ) -> Self {
        self.static_analysis_tools.push(ToolCapability {
            tool_name: tool.into(),
            required: false,
            alternatives: alternatives.into_iter().map(|a| a.into()).collect(),
            attestation: None,
            permissions: CapabilityPermissions::default(),
            expiration: CapabilityExpiration::default(),
            verified: false,
        });
        self
    }

    /// Add a capability flag
    pub fn with_flag(mut self, flag: impl Into<String>) -> Self {
        self.flags.insert(flag.into(), true);
        self
    }

    /// Add metadata
    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }

    /// Check if a capability is available
    pub fn has_capability(&self, capability_type: &str, tool_checker: &dyn Fn(&str) -> bool) -> bool {
        let tools = match capability_type {
            CAPABILITY_STATIC_ANALYSIS => &self.static_analysis_tools,
            CAPABILITY_SECURITY_SCANNING => &self.security_scanning_tools,
            CAPABILITY_DYNAMIC_ANALYSIS => &self.dynamic_analysis_tools,
            CAPABILITY_FUZZING => &self.fuzzing_tools,
            CAPABILITY_TEST_FRAMEWORK => &self.test_framework_tools,
            _ => return false,
        };

        if tools.is_empty() {
            return false;
        }

        // At least one tool must be satisfied
        tools.iter().any(|cap| cap.is_satisfied(tool_checker))
    }

    /// Check if all required tools are available
    pub fn has_all_required_tools(&self, tool_checker: &dyn Fn(&str) -> bool) -> bool {
        let all_tools = self
            .static_analysis_tools
            .iter()
            .chain(&self.security_scanning_tools)
            .chain(&self.dynamic_analysis_tools)
            .chain(&self.fuzzing_tools)
            .chain(&self.test_framework_tools);

        for tool in all_tools {
            if tool.required && !tool.is_satisfied(tool_checker) {
                return false;
            }
        }

        true
    }

    /// Get all tool names (including alternatives)
    pub fn all_tools(&self) -> Vec<String> {
        let mut tools = Vec::new();

        for cap in self
            .static_analysis_tools
            .iter()
            .chain(&self.security_scanning_tools)
            .chain(&self.dynamic_analysis_tools)
            .chain(&self.fuzzing_tools)
            .chain(&self.test_framework_tools)
        {
            tools.push(cap.tool_name.clone());
            tools.extend(cap.alternatives.clone());
        }

        tools
    }

    /// Check if a specific flag is set
    pub fn has_flag(&self, flag: &str) -> bool {
        self.flags.get(flag).copied().unwrap_or(false)
    }

    /// Get metadata value
    pub fn get_metadata(&self, key: &str) -> Option<&String> {
        self.metadata.get(key)
    }

    /// Verify all capabilities are attested and not expired/revoked
    pub fn verify_all_capabilities(&self) -> bool {
        let all_tools = self
            .static_analysis_tools
            .iter()
            .chain(&self.security_scanning_tools)
            .chain(&self.dynamic_analysis_tools)
            .chain(&self.fuzzing_tools)
            .chain(&self.test_framework_tools);

        for tool in all_tools {
            // Check if capability is expired
            if tool.is_expired() {
                return false;
            }

            // Check if capability is revoked
            if tool.is_revoked() {
                return false;
            }

            // Check if capability is attested (if attestation is required)
            if !tool.verify_attestation() {
                return false;
            }
        }

        true
    }

    /// Check if worker has required permissions for a capability
    pub fn has_required_permissions(&self, capability_type: &str, required_permission: &str) -> bool {
        let tools = match capability_type {
            CAPABILITY_STATIC_ANALYSIS => &self.static_analysis_tools,
            CAPABILITY_SECURITY_SCANNING => &self.security_scanning_tools,
            CAPABILITY_DYNAMIC_ANALYSIS => &self.dynamic_analysis_tools,
            CAPABILITY_FUZZING => &self.fuzzing_tools,
            CAPABILITY_TEST_FRAMEWORK => &self.test_framework_tools,
            _ => return false,
        };

        // At least one tool must have the required permission
        tools.iter().any(|tool| tool.has_permission(required_permission))
    }

    /// Revoke all capabilities
    pub fn revoke_all_capabilities(&mut self, reason: String, revoked_by: String) {
        for tool in self
            .static_analysis_tools
            .iter_mut()
            .chain(&mut self.security_scanning_tools)
            .chain(&mut self.dynamic_analysis_tools)
            .chain(&mut self.fuzzing_tools)
            .chain(&mut self.test_framework_tools)
        {
            tool.revoke(reason.clone(), revoked_by.clone());
        }
    }

    /// Get security report for all capabilities
    pub fn get_security_report(&self) -> HashMap<String, CapabilitySecurityReport> {
        let mut report = HashMap::new();
        let all_tools = self
            .static_analysis_tools
            .iter()
            .chain(&self.security_scanning_tools)
            .chain(&self.dynamic_analysis_tools)
            .chain(&self.fuzzing_tools)
            .chain(&self.test_framework_tools);

        for tool in all_tools {
            let security_report = CapabilitySecurityReport {
                tool_name: tool.tool_name.clone(),
                has_attestation: tool.attestation.is_some(),
                attestation_verified: tool.verify_attestation(),
                is_expired: tool.is_expired(),
                is_revoked: tool.is_revoked(),
                permissions: tool.permissions.clone(),
                expiration: tool.expiration.clone(),
            };
            report.insert(tool.tool_name.clone(), security_report);
        }

        report
    }

    /// Get capability statistics
    pub fn get_statistics(&self) -> CapabilityStatistics {
        let total_tools = self.static_analysis_tools.len()
            + self.security_scanning_tools.len()
            + self.dynamic_analysis_tools.len()
            + self.fuzzing_tools.len()
            + self.test_framework_tools.len();

        let required_tools = self
            .static_analysis_tools
            .iter()
            .chain(&self.security_scanning_tools)
            .chain(&self.dynamic_analysis_tools)
            .chain(&self.fuzzing_tools)
            .chain(&self.test_framework_tools)
            .filter(|tool| tool.required)
            .count();

        let verified_tools = self
            .static_analysis_tools
            .iter()
            .chain(&self.security_scanning_tools)
            .chain(&self.dynamic_analysis_tools)
            .chain(&self.fuzzing_tools)
            .chain(&self.test_framework_tools)
            .filter(|tool| tool.verify_attestation())
            .count();

        CapabilityStatistics {
            total_tools,
            required_tools,
            verified_tools,
            flags_count: self.flags.len(),
            metadata_count: self.metadata.len(),
        }
    }
}

/// Statistics about capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityStatistics {
    pub total_tools: usize,
    pub required_tools: usize,
    pub verified_tools: usize,
    pub flags_count: usize,
    pub metadata_count: usize,
}

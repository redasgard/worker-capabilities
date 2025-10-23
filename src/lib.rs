//! # Worker Capabilities
//!
//! A capability-aware workflow system for distributed workers with dynamic tool availability checking.
//!
//! ## Features
//!
//! - **Capability Registration**: Workers declare what they can do
//! - **Tool Availability**: Dynamic checking of tool availability
//! - **Builder Pattern**: Ergonomic API for defining capabilities
//! - **Alternative Tools**: Specify fallback tools
//! - **Generic Design**: Works with any tool/language/framework
//! - **Type-Safe**: Strongly typed capability definitions
//!
//! ## Quick Start
//!
//! ```rust
//! use worker_capabilities::{Capabilities, ToolCapability};
//!
//! # fn main() {
//! // Define capabilities for a worker
//! let caps = Capabilities::new("rust-analyzer")
//!     .with_tool("clippy", true)          // Required tool
//!     .with_tool("cargo-audit", false)    // Optional tool
//!     .with_alternative("rustfmt", vec!["rustfmt", "cargo-fmt"])
//!     .with_flag("ast_support")
//!     .with_flag("llm_support");
//!
//! // Check if worker has required capabilities
//! let tool_checker = |tool: &str| {
//!     // Your logic to check if tool is installed
//!     tool == "clippy" || tool == "cargo-audit"
//! };
//!
//! if caps.has_capability("static_analysis", &tool_checker) {
//!     println!("Worker can perform static analysis");
//! }
//! # }
//! ```

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Tool capability definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCapability {
    /// Name of the tool
    pub tool_name: String,
    /// Whether this tool is required (vs optional)
    pub required: bool,
    /// Alternative tools that can substitute
    pub alternatives: Vec<String>,
}

impl ToolCapability {
    /// Create a new tool capability
    pub fn new(tool_name: impl Into<String>, required: bool) -> Self {
        Self {
            tool_name: tool_name.into(),
            required,
            alternatives: Vec::new(),
        }
    }

    /// Add alternative tools
    pub fn with_alternatives(mut self, alternatives: Vec<String>) -> Self {
        self.alternatives = alternatives;
        self
    }

    /// Check if this capability is satisfied
    pub fn is_satisfied(&self, tool_checker: &dyn Fn(&str) -> bool) -> bool {
        // Check primary tool
        if tool_checker(&self.tool_name) {
            return true;
        }

        // Check alternatives
        self.alternatives.iter().any(|alt| tool_checker(alt))
    }
}

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
            "static_analysis" => &self.static_analysis_tools,
            "security_scanning" => &self.security_scanning_tools,
            "dynamic_analysis" => &self.dynamic_analysis_tools,
            "fuzzing" => &self.fuzzing_tools,
            "test_framework" => &self.test_framework_tools,
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
}

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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tool_capability_creation() {
        let cap = ToolCapability::new("clippy", true);
        assert_eq!(cap.tool_name, "clippy");
        assert!(cap.required);
        assert!(cap.alternatives.is_empty());
    }

    #[test]
    fn test_tool_capability_with_alternatives() {
        let cap = ToolCapability::new("rustfmt", false)
            .with_alternatives(vec!["cargo-fmt".to_string(), "rustfmt-nightly".to_string()]);

        assert_eq!(cap.alternatives.len(), 2);
    }

    #[test]
    fn test_tool_capability_is_satisfied() {
        let cap = ToolCapability::new("clippy", true)
            .with_alternatives(vec!["cargo-clippy".to_string()]);

        // Primary tool available
        assert!(cap.is_satisfied(&|tool| tool == "clippy"));

        // Alternative available
        assert!(cap.is_satisfied(&|tool| tool == "cargo-clippy"));

        // Neither available
        assert!(!cap.is_satisfied(&|_tool| false));
    }

    #[test]
    fn test_capabilities_builder() {
        let caps = Capabilities::new("rust-worker")
            .with_static_analysis("clippy", true)
            .with_security_tool("cargo-audit", false)
            .with_dynamic_tool("cargo-test", true)
            .with_flag("ast_support")
            .with_metadata("version", "1.0.0");

        assert_eq!(caps.id, "rust-worker");
        assert_eq!(caps.static_analysis_tools.len(), 1);
        assert_eq!(caps.security_scanning_tools.len(), 1);
        assert_eq!(caps.dynamic_analysis_tools.len(), 1);
        assert!(caps.has_flag("ast_support"));
        assert_eq!(caps.get_metadata("version"), Some(&"1.0.0".to_string()));
    }

    #[test]
    fn test_has_capability() {
        let caps = Capabilities::new("test")
            .with_static_analysis("clippy", false)
            .with_security_tool("audit", false);

        let tool_checker = |tool: &str| tool == "clippy";

        assert!(caps.has_capability("static_analysis", &tool_checker));
        assert!(!caps.has_capability("security_scanning", &tool_checker));
    }

    #[test]
    fn test_has_all_required_tools() {
        let caps = Capabilities::new("test")
            .with_static_analysis("required-tool", true)
            .with_security_tool("optional-tool", false);

        // Required tool available
        assert!(caps.has_all_required_tools(&|tool| tool == "required-tool"));

        // Required tool missing
        assert!(!caps.has_all_required_tools(&|_| false));

        // Optional tool missing is OK
        assert!(caps.has_all_required_tools(&|tool| tool == "required-tool"));
    }

    #[test]
    fn test_all_tools() {
        let caps = Capabilities::new("test")
            .with_tool("tool1", false)
            .with_alternative("tool2", vec!["alt1", "alt2"]);

        let all = caps.all_tools();
        assert!(all.contains(&"tool1".to_string()));
        assert!(all.contains(&"tool2".to_string()));
        assert!(all.contains(&"alt1".to_string()));
        assert!(all.contains(&"alt2".to_string()));
    }

    #[test]
    fn test_registry_operations() {
        let mut registry = CapabilityRegistry::new();

        let caps1 = Capabilities::new("worker1").with_tool("tool1", false);
        let caps2 = Capabilities::new("worker2").with_tool("tool2", false);

        registry.register(caps1);
        registry.register(caps2);

        assert_eq!(registry.list_ids().len(), 2);
        assert!(registry.get("worker1").is_some());
        assert!(registry.get("worker2").is_some());
        assert!(registry.get("worker3").is_none());
    }

    #[test]
    fn test_find_with_capability() {
        let mut registry = CapabilityRegistry::new();

        let caps1 = Capabilities::new("worker1").with_static_analysis("clippy", false);
        let caps2 = Capabilities::new("worker2").with_security_tool("audit", false);
        let caps3 = Capabilities::new("worker3").with_static_analysis("eslint", false);

        registry.register(caps1);
        registry.register(caps2);
        registry.register(caps3);

        let tool_checker = |tool: &str| tool == "clippy" || tool == "eslint";
        let workers = registry.find_with_capability("static_analysis", &tool_checker);

        assert_eq!(workers.len(), 2); // worker1 and worker3
    }

    #[test]
    fn test_serialization() {
        let caps = Capabilities::new("test")
            .with_tool("clippy", true)
            .with_flag("ast_support");

        let json = serde_json::to_string(&caps).unwrap();
        let deserialized: Capabilities = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.id, "test");
        assert_eq!(deserialized.static_analysis_tools.len(), 1);
        assert!(deserialized.has_flag("ast_support"));
    }
}


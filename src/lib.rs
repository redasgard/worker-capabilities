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
//! - **Security Features**: Attestation, expiration, revocation, permissions
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

pub mod attestation;
pub mod capabilities;
pub mod constants;
pub mod registry;
pub mod types;

// Re-export main types and functions
pub use attestation::*;
pub use capabilities::*;
pub use constants::*;
pub use registry::*;
pub use types::*;

// Re-export commonly used types for convenience
pub use capabilities::{Capabilities, CapabilityStatistics};
pub use registry::{CapabilityRegistry, RegistryStatistics};
pub use types::{
    ToolCapability, CapabilityAttestation, CapabilityPermissions, 
    CapabilityExpiration, CapabilitySecurityReport
};
# Worker Capabilities

[![Crates.io](https://img.shields.io/crates/v/worker-capabilities.svg)](https://crates.io/crates/worker-capabilities)
[![Documentation](https://docs.rs/worker-capabilities/badge.svg)](https://docs.rs/worker-capabilities)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE-MIT)

Capability-aware workflow system for distributed workers with dynamic tool availability checking.

## Features

- **Capability Registration**: Workers declare what they can do
- **Tool Availability**: Dynamic checking of tool availability  
- **Builder Pattern**: Ergonomic API for defining capabilities
- **Alternative Tools**: Specify fallback tools
- **Serializable**: JSON support for capability exchange
- **Type-Safe**: Strongly typed capability definitions
- **Zero Dependencies**: Only serde for serialization

## Installation

```toml
[dependencies]
worker-capabilities = "0.1"
```

## Quick Start

```rust
use worker_capabilities::Capabilities;

// Define capabilities for a Rust worker
let rust_worker = Capabilities::new("rust-analyzer")
    .with_static_analysis("clippy", true)       // Required
    .with_security_tool("cargo-audit", false)   // Optional
    .with_fuzzing_tool("cargo-fuzz", false)
    .with_flag("ast_support")
    .with_flag("llm_support");

// Check if worker can perform static analysis
let tool_checker = |tool: &str| {
    // Your logic to check if tool is installed
    matches!(tool, "clippy" | "cargo-audit")
};

if rust_worker.has_capability("static_analysis", &tool_checker) {
    println!("Worker can perform static analysis!");
}

// Check all required tools are available
if rust_worker.has_all_required_tools(&tool_checker) {
    println!("Worker is fully operational");
}
```

## Use Cases

### Distributed Task Assignment

Assign tasks to workers based on their capabilities:

```rust
use worker_capabilities::{Capabilities, CapabilityRegistry};

let mut registry = CapabilityRegistry::new();

// Register workers with different capabilities
registry.register(
    Capabilities::new("rust-worker")
        .with_static_analysis("clippy", true)
        .with_security_tool("cargo-audit", true)
);

registry.register(
    Capabilities::new("solidity-worker")
        .with_static_analysis("slither", true)
        .with_security_tool("mythril", false)
);

// Find workers that can do security scanning
let tool_checker = |tool: &str| tool == "cargo-audit" || tool == "slither";
let security_workers = registry.find_with_capability("security_scanning", &tool_checker);

for worker in security_workers {
    println!("Worker {} can perform security scanning", worker.id);
}
```

### Tool Alternatives

Specify alternative tools for flexibility:

```rust
let caps = Capabilities::new("formatter")
    .with_alternative(
        "rustfmt",
        vec!["rustfmt", "cargo-fmt", "rustfmt-nightly"]
    );

// Will accept any of the alternatives
let satisfied = caps.has_capability("static_analysis", &|tool| {
    tool == "cargo-fmt" // Using alternative
});
```

### Capability Negotiation

Workers and coordinators can negotiate capabilities:

```rust
let worker_caps = Capabilities::new("worker1")
    .with_static_analysis("tool-a", true)
    .with_dynamic_tool("tool-b", false);

// Coordinator checks if worker meets requirements
let required_checker = |tool: &str| tool == "tool-a";

if worker_caps.has_all_required_tools(&required_checker) {
    // Assign task to worker
    println!("Worker meets requirements");
}
```

### Metadata Tracking

Store additional information about capabilities:

```rust
let caps = Capabilities::new("worker")
    .with_tool("clippy", true)
    .with_metadata("version", "0.1.0")
    .with_metadata("platform", "linux")
    .with_metadata("max_concurrent", "4");

if let Some(platform) = caps.get_metadata("platform") {
    println!("Worker runs on: {}", platform);
}
```

## API Reference

### `Capabilities`

Main capability definition.

**Constructor**:
- `new(id)` - Create new capability set

**Builder Methods**:
- `with_static_analysis(tool, required)` - Add static analysis tool
- `with_security_tool(tool, required)` - Add security tool
- `with_dynamic_tool(tool, required)` - Add dynamic analysis tool
- `with_fuzzing_tool(tool, required)` - Add fuzzing tool
- `with_test_framework(tool, required)` - Add test framework
- `with_tool(tool, required)` - Add generic tool
- `with_alternative(tool, alternatives)` - Add tool with fallbacks
- `with_flag(flag)` - Add capability flag
- `with_metadata(key, value)` - Add metadata

**Query Methods**:
- `has_capability(type, checker)` - Check if capability is available
- `has_all_required_tools(checker)` - Verify all required tools
- `all_tools()` - List all tools (including alternatives)
- `has_flag(flag)` - Check if flag is set
- `get_metadata(key)` - Get metadata value

### `CapabilityRegistry`

Registry for managing multiple capability sets.

**Methods**:
- `new()` - Create empty registry
- `register(caps)` - Register capability set
- `get(id)` - Get capabilities by ID
- `list_ids()` - List all registered IDs
- `find_with_capability(type, checker)` - Find workers with capability

### `ToolCapability`

Individual tool capability.

**Fields**:
- `tool_name` - Primary tool name
- `required` - Whether tool is required
- `alternatives` - Alternative tool names

**Methods**:
- `new(name, required)` - Create tool capability
- `with_alternatives(alts)` - Add alternative tools
- `is_satisfied(checker)` - Check if tool is available

## Examples

### CI/CD Pipeline

```rust
// Define capabilities for different build agents
let linux_agent = Capabilities::new("linux-agent")
    .with_tool("docker", true)
    .with_tool("cargo", true)
    .with_metadata("os", "linux")
    .with_metadata("arch", "x86_64");

let macos_agent = Capabilities::new("macos-agent")
    .with_tool("xcodebuild", true)
    .with_tool("cargo", true)
    .with_metadata("os", "macos")
    .with_metadata("arch", "aarch64");

// Assign tasks based on capabilities and metadata
```

### Job Queue System

```rust
// Workers register their capabilities
let worker = Capabilities::new("worker-1")
    .with_static_analysis("analyzer", true)
    .with_security_tool("scanner", false)
    .with_metadata("max_jobs", "5")
    .with_metadata("priority", "high");

// Job queue matches tasks to capable workers
if worker.has_capability("static_analysis", &tool_checker) {
    // Assign static analysis job
}
```

## Testing

```bash
# Run tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_capabilities_builder
```

## Origin

Extracted from [Valkra](https://github.com/asgardtech/valkra), a blockchain security auditing platform where it manages capability negotiation between distributed security analysis workers.

## License

Licensed under MIT License. See [LICENSE-MIT](LICENSE-MIT) for details.

## Contributing

Contributions welcome! Areas of interest:
- Additional capability types
- Performance optimizations
- Documentation improvements
- Real-world examples

## Contact

- **Author**: Red Asgard
- **Email**: hello@redasgard.com
- **GitHub**: https://github.com/redasgard


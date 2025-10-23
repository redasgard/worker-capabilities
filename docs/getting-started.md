# Getting Started

## Installation

Add Worker Capabilities to your `Cargo.toml`:

```toml
[dependencies]
worker-capabilities = "0.1"
```

## First Steps

### 1. Define Worker Capabilities

Create a capability set for your worker:

```rust
use worker_capabilities::Capabilities;

fn main() {
    let worker = Capabilities::new("my-worker")
        .with_tool("clippy", true);      // Required tool
    
    println!("Worker ID: {}", worker.id);
}
```

### 2. Add Multiple Tools

Build comprehensive capability sets:

```rust
let rust_worker = Capabilities::new("rust-analyzer")
    .with_static_analysis("clippy", true)       // Required
    .with_static_analysis("rustc", true)        // Required
    .with_security_tool("cargo-audit", false)   // Optional
    .with_fuzzing_tool("cargo-fuzz", false)     // Optional
    .with_test_framework("cargo-test", true);   // Required
```

### 3. Add Capability Flags

Declare additional capabilities:

```rust
let worker = Capabilities::new("advanced-worker")
    .with_tool("clippy", true)
    .with_flag("ast_support")           // Can parse AST
    .with_flag("llm_support")           // Has LLM integration
    .with_flag("parallel_execution");   // Can run in parallel
```

### 4. Add Metadata

Store additional information:

```rust
let worker = Capabilities::new("worker-1")
    .with_tool("analyzer", true)
    .with_metadata("version", "1.0.0")
    .with_metadata("platform", "linux")
    .with_metadata("region", "us-east")
    .with_metadata("max_concurrent_tasks", "5");
```

### 5. Check Capabilities

Verify if worker can perform a task:

```rust
// Define tool checker (checks if tool is installed/available)
let tool_checker = |tool: &str| {
    matches!(tool, "clippy" | "rustc" | "cargo-audit")
};

// Check if worker can do static analysis
if worker.has_capability("static_analysis", &tool_checker) {
    println!("✓ Worker can perform static analysis");
}

// Check if all required tools are available
if worker.has_all_required_tools(&tool_checker) {
    println!("✓ Worker is fully operational");
} else {
    println!("✗ Worker missing required tools");
}
```

### 6. Use Registry

Manage multiple workers:

```rust
use worker_capabilities::CapabilityRegistry;

let mut registry = CapabilityRegistry::new();

// Register workers
let worker1 = Capabilities::new("rust-worker")
    .with_static_analysis("clippy", true);

let worker2 = Capabilities::new("js-worker")
    .with_static_analysis("eslint", true);

registry.register(worker1);
registry.register(worker2);

// Find capable workers
let tool_checker = |tool: &str| tool == "clippy" || tool == "eslint";
let capable = registry.find_with_capability("static_analysis", &tool_checker);

println!("Found {} capable workers", capable.len());
```

## Complete Example

```rust
use worker_capabilities::{Capabilities, CapabilityRegistry};

fn main() {
    println!("=== Worker Capabilities Example ===\n");
    
    // 1. Create capabilities for different workers
    let rust_worker = Capabilities::new("rust-worker")
        .with_static_analysis("clippy", true)
        .with_security_tool("cargo-audit", true)
        .with_metadata("language", "rust")
        .with_metadata("max_jobs", "10");
    
    let solidity_worker = Capabilities::new("solidity-worker")
        .with_static_analysis("slither", true)
        .with_security_tool("mythril", false)
        .with_metadata("language", "solidity")
        .with_metadata("max_jobs", "5");
    
    let python_worker = Capabilities::new("python-worker")
        .with_static_analysis("pylint", true)
        .with_security_tool("bandit", true)
        .with_test_framework("pytest", true)
        .with_metadata("language", "python");
    
    // 2. Create registry
    let mut registry = CapabilityRegistry::new();
    registry.register(rust_worker);
    registry.register(solidity_worker);
    registry.register(python_worker);
    
    println!("Registered {} workers\n", registry.list_ids().len());
    
    // 3. Find workers for static analysis
    let tool_checker = |tool: &str| {
        matches!(tool, "clippy" | "slither" | "pylint")
    };
    
    let static_analysis_workers = registry.find_with_capability(
        "static_analysis",
        &tool_checker
    );
    
    println!("Workers with static analysis:");
    for worker in static_analysis_workers {
        println!("  - {} ({})", 
            worker.id, 
            worker.get_metadata("language").unwrap_or(&"unknown".to_string())
        );
    }
    
    // 4. Find workers for security scanning
    let security_checker = |tool: &str| {
        matches!(tool, "cargo-audit" | "mythril" | "bandit")
    };
    
    let security_workers = registry.find_with_capability(
        "security_scanning",
        &security_checker
    );
    
    println!("\nWorkers with security scanning:");
    for worker in security_workers {
        println!("  - {}", worker.id);
    }
    
    // 5. Check specific worker capabilities
    if let Some(rust_worker) = registry.get("rust-worker") {
        println!("\nRust worker capabilities:");
        println!("  Has static analysis: {}", 
            rust_worker.has_capability("static_analysis", &tool_checker));
        println!("  Has all required tools: {}", 
            rust_worker.has_all_required_tools(&tool_checker));
        println!("  Max jobs: {}", 
            rust_worker.get_metadata("max_jobs").unwrap_or(&"?".to_string()));
    }
}
```

## Tool Alternatives

Handle different tool versions/variants:

```rust
let worker = Capabilities::new("formatter")
    .with_alternative("rustfmt", vec![
        "rustfmt",
        "cargo-fmt",
        "rustfmt-nightly"
    ]);

// Worker accepts any variant
let checker1 = |tool: &str| tool == "rustfmt";
assert!(worker.has_capability("static_analysis", &checker1));

let checker2 = |tool: &str| tool == "cargo-fmt";
assert!(worker.has_capability("static_analysis", &checker2));

let checker3 = |tool: &str| tool == "rustfmt-nightly";
assert!(worker.has_capability("static_analysis", &checker3));
```

## Dynamic Tool Checking

Implement tool checkers for your environment:

```rust
// Check if command exists
fn command_exists(tool: &str) -> bool {
    std::process::Command::new("which")
        .arg(tool)
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

// Check against installed list
fn check_installed_tools(tool: &str) -> bool {
    let installed = vec!["clippy", "rustc", "cargo"];
    installed.contains(&tool)
}

// Use in capability check
let worker = Capabilities::new("worker");
if worker.has_all_required_tools(&command_exists) {
    println!("Worker has all required tools installed");
}
```

## Serialization

Share capabilities over network:

```rust
use serde_json;

// Worker side: Serialize and send
let capabilities = Capabilities::new("worker1")
    .with_tool("clippy", true);

let json = serde_json::to_string(&capabilities)?;
send_to_coordinator(&json)?;

// Coordinator side: Receive and deserialize
let json = receive_from_worker()?;
let capabilities: Capabilities = serde_json::from_str(&json)?;

registry.register(capabilities);
```

## Next Steps

- Read [Use Cases](./use-cases.md) for practical examples
- Check [API Reference](./api-reference.md) for detailed documentation
- See [Integration Guide](./integration-guide.md) for system integration
- Review [Best Practices](./best-practices.md) for recommended patterns

## Troubleshooting

### Capabilities not found in registry

Check:
- Worker registered successfully
- Using correct worker ID
- Registry not cleared

### Tool checker always returns false

Check:
- Tool names match exactly
- Tool checker logic is correct
- Tools are actually installed

### has_all_required_tools() returns false

Check:
- All required tools are marked correctly
- Tool checker can find all required tools
- No typos in tool names

## Getting Help

- **Documentation**: See `/docs/` directory
- **Examples**: Check `examples/` directory
- **Issues**: https://github.com/redasgard/worker-capabilities/issues
- **Email**: hello@redasgard.com


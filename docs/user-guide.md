# User Guide

Comprehensive guide for using Worker Capabilities.

## Building Capabilities

### Basic Capability Set

```rust
use worker_capabilities::Capabilities;

let worker = Capabilities::new("my-worker")
    .with_tool("analyzer", true);
```

### Comprehensive Capability Set

```rust
let worker = Capabilities::new("full-featured-worker")
    // Static analysis
    .with_static_analysis("clippy", true)
    .with_static_analysis("rustc", true)
    
    // Security scanning
    .with_security_tool("cargo-audit", true)
    .with_security_tool("cargo-geiger", false)
    
    // Dynamic analysis
    .with_dynamic_tool("debugger", false)
    .with_dynamic_tool("profiler", false)
    
    // Fuzzing
    .with_fuzzing_tool("cargo-fuzz", false)
    .with_fuzzing_tool("afl", false)
    
    // Test frameworks
    .with_test_framework("cargo-test", true)
    .with_test_framework("cargo-nextest", false)
    
    // Feature flags
    .with_flag("ast_support")
    .with_flag("llm_support")
    .with_flag("parallel_execution")
    
    // Metadata
    .with_metadata("version", "1.0.0")
    .with_metadata("platform", "linux")
    .with_metadata("region", "us-east")
    .with_metadata("max_concurrent", "10");
```

## Tool Alternatives

### Single Alternative

```rust
let worker = Capabilities::new("formatter")
    .with_alternative("rustfmt", vec!["rustfmt", "cargo-fmt"]);
```

### Multiple Alternatives

```rust
let worker = Capabilities::new("analyzer")
    .with_alternative("linter", vec![
        "clippy",
        "cargo-clippy",
        "clippy-preview"
    ])
    .with_alternative("formatter", vec![
        "rustfmt",
        "cargo-fmt",
        "rustfmt-nightly"
    ]);
```

### Platform-Specific Alternatives

```rust
let compiler = Capabilities::new("compiler")
    .with_alternative("cc", vec![
        "gcc",          // Linux
        "clang",        // macOS
        "cl.exe",       // Windows
        "cc"            // Generic
    ]);
```

## Checking Capabilities

### Simple Check

```rust
let tool_checker = |tool: &str| tool == "clippy";

if worker.has_capability("static_analysis", &tool_checker) {
    println!("Worker can do static analysis");
}
```

### Advanced Tool Checker

```rust
// Check if tool actually exists on system
fn system_has_tool(tool: &str) -> bool {
    std::process::Command::new("which")
        .arg(tool)
        .output()
        .map(|out| out.status.success())
        .unwrap_or(false)
}

// Use in capability check
if worker.has_all_required_tools(&system_has_tool) {
    println!("All required tools are installed");
}
```

### Custom Tool Checker

```rust
struct ToolChecker {
    available_tools: Vec<String>,
}

impl ToolChecker {
    fn new() -> Self {
        Self {
            available_tools: vec![
                "clippy".to_string(),
                "cargo-audit".to_string(),
                "rustfmt".to_string(),
            ],
        }
    }
    
    fn check(&self, tool: &str) -> bool {
        self.available_tools.contains(&tool.to_string())
    }
}

// Use
let checker = ToolChecker::new();
if worker.has_capability("static_analysis", &|t| checker.check(t)) {
    println!("Worker capable");
}
```

## Using Registry

### Register Workers

```rust
use worker_capabilities::CapabilityRegistry;

let mut registry = CapabilityRegistry::new();

// Register multiple workers
for worker_id in &["worker-1", "worker-2", "worker-3"] {
    let caps = build_capabilities(worker_id);
    registry.register(caps);
}

println!("Registered {} workers", registry.list_ids().len());
```

### Find Capable Workers

```rust
// Find workers with static analysis
let tool_checker = |tool: &str| matches!(tool, "clippy" | "eslint" | "pylint");

let workers = registry.find_with_capability("static_analysis", &tool_checker);

for worker in workers {
    println!("Capable worker: {}", worker.id);
}
```

### Filter by Metadata

```rust
// Find Linux workers with clippy
let workers = registry.find_with_capability("static_analysis", &has_clippy);

let linux_workers: Vec<_> = workers.iter()
    .filter(|w| w.get_metadata("platform") == Some(&"linux".to_string()))
    .collect();
```

### Select by Criteria

```rust
// Find least loaded worker
let workers = registry.find_with_capability("analysis", &tool_checker);

let best_worker = workers.iter()
    .min_by_key(|w| {
        w.get_metadata("current_load")
            .and_then(|l| l.parse::<usize>().ok())
            .unwrap_or(0)
    });

if let Some(worker) = best_worker {
    println!("Selected worker: {}", worker.id);
}
```

## Flags and Metadata

### Feature Flags

```rust
let worker = Capabilities::new("advanced-worker")
    .with_tool("analyzer", true)
    .with_flag("gpu_acceleration")
    .with_flag("ml_support")
    .with_flag("distributed_execution");

// Check flags
if worker.has_flag("gpu_acceleration") {
    // Use GPU-accelerated analysis
}

if worker.has_flag("ml_support") {
    // Use ML-based features
}
```

### Metadata

```rust
let worker = Capabilities::new("worker")
    .with_tool("tool", true)
    .with_metadata("version", "2.1.0")
    .with_metadata("max_memory", "8192")
    .with_metadata("cost_per_hour", "0.50")
    .with_metadata("uptime", "99.9");

// Query metadata
if let Some(version) = worker.get_metadata("version") {
    println!("Worker version: {}", version);
}

// Use for selection
let affordable_workers = registry.find_with_capability("analysis", &tool_checker)
    .into_iter()
    .filter(|w| {
        w.get_metadata("cost_per_hour")
            .and_then(|c| c.parse::<f64>().ok())
            .map_or(false, |cost| cost < 1.0)
    })
    .collect::<Vec<_>>();
```

## Serialization

### Send Over Network

```rust
// Worker side
let capabilities = Capabilities::new("worker-1")
    .with_tool("clippy", true);

let json = serde_json::to_string(&capabilities)?;

// Send to coordinator
send_to_coordinator(&json)?;

// Coordinator side
let received = receive_from_worker()?;
let capabilities: Capabilities = serde_json::from_str(&received)?;

registry.register(capabilities);
```

### Save to File

```rust
// Save capabilities
let json = serde_json::to_string_pretty(&capabilities)?;
std::fs::write("worker-capabilities.json", json)?;

// Load capabilities
let json = std::fs::read_to_string("worker-capabilities.json")?;
let capabilities: Capabilities = serde_json::from_str(&json)?;
```

## Pattern: Dynamic Worker Pool

```rust
use worker_capabilities::{Capabilities, CapabilityRegistry};
use std::sync::{Arc, Mutex};

struct WorkerPool {
    registry: Arc<Mutex<CapabilityRegistry>>,
}

impl WorkerPool {
    fn new() -> Self {
        Self {
            registry: Arc::new(Mutex::new(CapabilityRegistry::new())),
        }
    }
    
    fn add_worker(&self, worker_id: &str, tools: Vec<&str>) {
        let mut caps = Capabilities::new(worker_id);
        
        for tool in tools {
            caps = caps.with_tool(tool, true);
        }
        
        self.registry.lock().unwrap().register(caps);
    }
    
    fn remove_worker(&self, worker_id: &str) {
        // Note: CapabilityRegistry doesn't have remove() method
        // You'd need to implement this or recreate registry
    }
    
    fn assign_task(&self, task_type: &str, required_tools: Vec<&str>) -> Option<String> {
        let checker = move |tool: &str| required_tools.contains(&tool);
        
        let registry = self.registry.lock().unwrap();
        let workers = registry.find_with_capability(task_type, &checker);
        
        workers.first().map(|w| w.id.clone())
    }
}
```

## Pattern: Capability Negotiation

```rust
// Worker advertises capabilities
let worker_caps = Capabilities::new("worker")
    .with_static_analysis("analyzer-v2", true)
    .with_flag("beta_features");

// Coordinator has requirements
struct TaskRequirements {
    min_version: String,
    allow_beta: bool,
}

fn can_handle_task(caps: &Capabilities, req: &TaskRequirements) -> bool {
    // Check version
    if let Some(version) = caps.get_metadata("version") {
        if version < &req.min_version {
            return false;
        }
    }
    
    // Check beta flag
    if req.allow_beta && !caps.has_flag("beta_features") {
        return false;
    }
    
    true
}
```

## Best Practices

### 1. Always Check Required Tools

```rust
// ✅ Good
if worker.has_all_required_tools(&tool_checker) {
    assign_task(worker.id);
} else {
    log_error("Worker missing required tools");
}

// ❌ Bad
assign_task(worker.id);  // Hope it works!
```

### 2. Use Alternatives for Flexibility

```rust
// ✅ Good
let caps = Capabilities::new("worker")
    .with_alternative("formatter", vec!["rustfmt", "cargo-fmt"]);

// ❌ Bad
let caps = Capabilities::new("worker")
    .with_tool("rustfmt", true);  // Only accepts exact match
```

### 3. Include Metadata for Selection

```rust
// ✅ Good
let caps = Capabilities::new("worker")
    .with_tool("analyzer", true)
    .with_metadata("max_jobs", "5")
    .with_metadata("priority", "high")
    .with_metadata("region", "us-east");

// ❌ Bad
let caps = Capabilities::new("worker")
    .with_tool("analyzer", true);
    // No way to differentiate workers
```

## Next Steps

- Explore [Use Cases](./use-cases.md) for practical examples
- Review [API Reference](./api-reference.md) for complete API
- Check [Integration Guide](./integration-guide.md) for system integration
- See [Best Practices](./best-practices.md) for optimal usage


# Examples

Practical examples of using Worker Capabilities.

## Example 1: Simple Worker Registration

```rust
use worker_capabilities::Capabilities;

fn main() {
    // Create capabilities for a Rust worker
    let worker = Capabilities::new("rust-worker-1")
        .with_static_analysis("clippy", true)
        .with_security_tool("cargo-audit", false);
    
    println!("Worker: {}", worker.id);
    println!("Tools: {:?}", worker.all_tools());
}
```

## Example 2: Tool Availability Checking

```rust
use worker_capabilities::Capabilities;

fn main() {
    let worker = Capabilities::new("my-worker")
        .with_tool("clippy", true)
        .with_tool("cargo-audit", false);
    
    // Define tool checker
    let tool_checker = |tool: &str| {
        match tool {
            "clippy" => true,
            "cargo-audit" => false,
            _ => false,
        }
    };
    
    // Check capabilities
    if worker.has_capability("static_analysis", &tool_checker) {
        println!("✓ Worker can perform static analysis");
    }
    
    if worker.has_all_required_tools(&tool_checker) {
        println!("✓ Worker has all required tools");
    }
}
```

## Example 3: Multi-Worker Registry

```rust
use worker_capabilities::{Capabilities, CapabilityRegistry};

fn main() {
    let mut registry = CapabilityRegistry::new();
    
    // Register Rust workers
    registry.register(
        Capabilities::new("rust-worker-1")
            .with_static_analysis("clippy", true)
            .with_security_tool("cargo-audit", true)
    );
    
    registry.register(
        Capabilities::new("rust-worker-2")
            .with_static_analysis("clippy", true)
            .with_fuzzing_tool("cargo-fuzz", true)
    );
    
    // Register Solidity worker
    registry.register(
        Capabilities::new("solidity-worker-1")
            .with_static_analysis("slither", true)
            .with_security_tool("mythril", true)
    );
    
    println!("Registry has {} workers", registry.list_ids().len());
    
    // Find workers with security scanning
    let checker = |tool: &str| {
        matches!(tool, "cargo-audit" | "mythril")
    };
    
    let security_workers = registry.find_with_capability("security_scanning", &checker);
    
    println!("\nSecurity workers:");
    for worker in security_workers {
        println!("  - {}", worker.id);
    }
}
```

## Example 4: Tool Alternatives

```rust
use worker_capabilities::Capabilities;

fn main() {
    // Worker accepts multiple formatter variants
    let worker = Capabilities::new("formatter")
        .with_alternative("rustfmt", vec![
            "rustfmt",
            "cargo-fmt",
            "rustfmt-nightly"
        ]);
    
    // Test different tool checkers
    let checkers = vec![
        ("rustfmt", |tool: &str| tool == "rustfmt"),
        ("cargo-fmt", |tool: &str| tool == "cargo-fmt"),
        ("rustfmt-nightly", |tool: &str| tool == "rustfmt-nightly"),
    ];
    
    for (name, checker) in checkers {
        if worker.has_capability("static_analysis", &checker) {
            println!("✓ Worker accepts {}", name);
        }
    }
}
```

## Example 5: Metadata-Based Selection

```rust
use worker_capabilities::{Capabilities, CapabilityRegistry};

fn main() {
    let mut registry = CapabilityRegistry::new();
    
    // Register workers with metadata
    registry.register(
        Capabilities::new("worker-1")
            .with_tool("analyzer", true)
            .with_metadata("region", "us-east")
            .with_metadata("cost", "0.50")
            .with_metadata("load", "3")
    );
    
    registry.register(
        Capabilities::new("worker-2")
            .with_tool("analyzer", true)
            .with_metadata("region", "eu-west")
            .with_metadata("cost", "0.60")
            .with_metadata("load", "1")
    );
    
    // Find workers by region
    let checker = |_| true;  // All workers have analyzer
    let workers = registry.find_with_capability("static_analysis", &checker);
    
    let us_workers: Vec<_> = workers.iter()
        .filter(|w| w.get_metadata("region") == Some(&"us-east".to_string()))
        .collect();
    
    println!("US workers: {}", us_workers.len());
    
    // Find cheapest worker
    let cheapest = workers.iter()
        .min_by_key(|w| {
            w.get_metadata("cost")
                .and_then(|c| c.parse::<f64>().ok())
                .unwrap_or(f64::MAX) as u64
        });
    
    if let Some(worker) = cheapest {
        println!("Cheapest worker: {} (${}/hr)", 
            worker.id,
            worker.get_metadata("cost").unwrap()
        );
    }
    
    // Find least loaded worker
    let least_loaded = workers.iter()
        .min_by_key(|w| {
            w.get_metadata("load")
                .and_then(|l| l.parse::<usize>().ok())
                .unwrap_or(usize::MAX)
        });
    
    if let Some(worker) = least_loaded {
        println!("Least loaded: {} ({} tasks)", 
            worker.id,
            worker.get_metadata("load").unwrap()
        );
    }
}
```

## Example 6: Dynamic Tool Detection

```rust
use worker_capabilities::Capabilities;
use std::process::Command;

fn detect_capabilities(worker_id: &str) -> Capabilities {
    let mut caps = Capabilities::new(worker_id);
    
    // Detect Rust tools
    if command_exists("clippy") {
        caps = caps.with_static_analysis("clippy", true);
    }
    
    if command_exists("cargo-audit") {
        caps = caps.with_security_tool("cargo-audit", false);
    }
    
    if command_exists("cargo-fuzz") {
        caps = caps.with_fuzzing_tool("cargo-fuzz", false);
    }
    
    // Add metadata
    caps = caps.with_metadata("auto_detected", "true");
    caps = caps.with_metadata("platform", std::env::consts::OS);
    
    caps
}

fn command_exists(cmd: &str) -> bool {
    Command::new("which")
        .arg(cmd)
        .output()
        .map(|out| out.status.success())
        .unwrap_or(false)
}

fn main() {
    let caps = detect_capabilities("auto-worker");
    println!("Detected {} tools", caps.all_tools().len());
}
```

## Example 7: Serialization and Network Transport

```rust
use worker_capabilities::Capabilities;
use serde_json;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create capabilities
    let caps = Capabilities::new("remote-worker")
        .with_tool("analyzer", true)
        .with_flag("remote_execution")
        .with_metadata("endpoint", "https://worker.example.com");
    
    // Serialize to JSON
    let json = serde_json::to_string_pretty(&caps)?;
    println!("Serialized:\n{}", json);
    
    // Send over network (simulated)
    send_to_coordinator(&json);
    
    // Receive and deserialize
    let received = receive_from_network();
    let deserialized: Capabilities = serde_json::from_str(&received)?;
    
    println!("\nDeserialized worker: {}", deserialized.id);
    
    Ok(())
}

fn send_to_coordinator(json: &str) {
    println!("\n[Sending to coordinator...]");
}

fn receive_from_network() -> String {
    // Simulated
    r#"{"id":"remote-worker","static_analysis_tools":[{"tool_name":"analyzer","required":true,"alternatives":[]}],"security_scanning_tools":[],"dynamic_analysis_tools":[],"fuzzing_tools":[],"test_framework_tools":[],"flags":{"remote_execution":true},"metadata":{"endpoint":"https://worker.example.com"}}"#.to_string()
}
```

## Example 8: Task Assignment Logic

```rust
use worker_capabilities::{Capabilities, CapabilityRegistry};

struct Task {
    id: String,
    task_type: String,
    required_tools: Vec<String>,
    preferred_region: Option<String>,
}

fn assign_task(registry: &CapabilityRegistry, task: &Task) -> Option<String> {
    let tool_checker = |tool: &str| {
        task.required_tools.contains(&tool.to_string())
    };
    
    // Find capable workers
    let mut workers = registry.find_with_capability(&task.task_type, &tool_checker);
    
    // Prefer workers in same region
    if let Some(region) = &task.preferred_region {
        let regional = workers.iter()
            .find(|w| w.get_metadata("region") == Some(region));
        
        if let Some(worker) = regional {
            return Some(worker.id.clone());
        }
    }
    
    // Fallback to any capable worker
    workers.first().map(|w| w.id.clone())
}

fn main() {
    let mut registry = CapabilityRegistry::new();
    
    registry.register(
        Capabilities::new("us-worker")
            .with_static_analysis("analyzer", true)
            .with_metadata("region", "us-east")
    );
    
    registry.register(
        Capabilities::new("eu-worker")
            .with_static_analysis("analyzer", true)
            .with_metadata("region", "eu-west")
    );
    
    let task = Task {
        id: "task-1".to_string(),
        task_type: "static_analysis".to_string(),
        required_tools: vec!["analyzer".to_string()],
        preferred_region: Some("us-east".to_string()),
    };
    
    if let Some(worker_id) = assign_task(&registry, &task) {
        println!("Assigned task to: {}", worker_id);
    }
}
```

## Example 9: Health Monitoring

```rust
use worker_capabilities::{Capabilities, CapabilityRegistry};
use std::time::{Duration, Instant};
use std::collections::HashMap;

struct HealthMonitor {
    registry: CapabilityRegistry,
    last_heartbeat: HashMap<String, Instant>,
}

impl HealthMonitor {
    fn new() -> Self {
        Self {
            registry: CapabilityRegistry::new(),
            last_heartbeat: HashMap::new(),
        }
    }
    
    fn register_worker(&mut self, caps: Capabilities) {
        let worker_id = caps.id.clone();
        self.registry.register(caps);
        self.last_heartbeat.insert(worker_id, Instant::now());
    }
    
    fn heartbeat(&mut self, worker_id: &str) {
        self.last_heartbeat.insert(worker_id.to_string(), Instant::now());
    }
    
    fn check_health(&self, timeout: Duration) -> Vec<String> {
        let now = Instant::now();
        let mut unhealthy = Vec::new();
        
        for worker_id in self.registry.list_ids() {
            if let Some(last_seen) = self.last_heartbeat.get(&worker_id) {
                if now.duration_since(*last_seen) > timeout {
                    unhealthy.push(worker_id);
                }
            }
        }
        
        unhealthy
    }
}

fn main() {
    let mut monitor = HealthMonitor::new();
    
    monitor.register_worker(
        Capabilities::new("worker-1")
            .with_tool("analyzer", true)
    );
    
    // Simulate heartbeats
    std::thread::sleep(Duration::from_secs(2));
    monitor.heartbeat("worker-1");
    
    // Check health
    let unhealthy = monitor.check_health(Duration::from_secs(5));
    println!("Unhealthy workers: {}", unhealthy.len());
}
```

## Example 10: Load Balancing

```rust
use worker_capabilities::{Capabilities, CapabilityRegistry};

fn main() {
    let mut registry = CapabilityRegistry::new();
    
    // Register workers with current load
    for i in 1..=5 {
        registry.register(
            Capabilities::new(&format!("worker-{}", i))
                .with_tool("analyzer", true)
                .with_metadata("current_tasks", &i.to_string())
        );
    }
    
    // Find least loaded worker
    let checker = |_| true;
    let workers = registry.find_with_capability("static_analysis", &checker);
    
    let least_loaded = workers.iter()
        .min_by_key(|w| {
            w.get_metadata("current_tasks")
                .and_then(|t| t.parse::<usize>().ok())
                .unwrap_or(usize::MAX)
        });
    
    if let Some(worker) = least_loaded {
        println!("Selected least loaded worker: {} ({} tasks)",
            worker.id,
            worker.get_metadata("current_tasks").unwrap()
        );
    }
}
```

## More Examples

See the `examples/` directory in the repository for additional examples:

- `examples/basic_usage.rs` - Basic capability definition
- `examples/registry_usage.rs` - Registry operations
- `examples/tool_alternatives.rs` - Alternative tool handling
- `examples/metadata_filtering.rs` - Metadata-based selection
- `examples/distributed_system.rs` - Complete distributed system

## Next Steps

- Review [User Guide](./user-guide.md) for comprehensive usage
- Check [Integration Guide](./integration-guide.md) for system integration
- See [Best Practices](./best-practices.md) for optimal patterns


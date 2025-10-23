# Best Practices

Best practices for using Worker Capabilities effectively.

## Capability Definition

### Use Descriptive IDs

```rust
// ✅ Good: Descriptive
let caps = Capabilities::new("rust-analyzer-worker-us-east-1");

// ❌ Bad: Generic
let caps = Capabilities::new("worker1");
```

### Mark Required vs Optional Correctly

```rust
// ✅ Good: Clear requirements
let caps = Capabilities::new("worker")
    .with_tool("clippy", true)          // MUST have
    .with_tool("cargo-audit", false);   // NICE to have

// ❌ Bad: Everything required
let caps = Capabilities::new("worker")
    .with_tool("clippy", true)
    .with_tool("cargo-audit", true)     // Too restrictive!
    .with_tool("cargo-geiger", true);   // Few workers will match
```

### Use Alternatives for Flexibility

```rust
// ✅ Good: Accept multiple variants
let caps = Capabilities::new("formatter")
    .with_alternative("rustfmt", vec![
        "rustfmt",
        "cargo-fmt",
        "rustfmt-nightly",
        "rustfmt-stable"
    ]);

// ❌ Bad: Only one option
let caps = Capabilities::new("formatter")
    .with_tool("rustfmt-nightly", true);  // Too specific
```

### Include Useful Metadata

```rust
// ✅ Good: Rich metadata
let caps = Capabilities::new("worker")
    .with_tool("analyzer", true)
    .with_metadata("version", "1.2.3")
    .with_metadata("platform", "linux")
    .with_metadata("arch", "x86_64")
    .with_metadata("region", "us-east-1")
    .with_metadata("max_concurrent_tasks", "5")
    .with_metadata("cost_per_hour", "0.50");

// ❌ Bad: No metadata
let caps = Capabilities::new("worker")
    .with_tool("analyzer", true);
    // No way to differentiate or select
```

## Tool Checking

### Implement Robust Tool Checker

```rust
// ✅ Good: Comprehensive checking
fn robust_tool_checker(tool: &str) -> bool {
    // 1. Check in-memory cache
    if TOOL_CACHE.contains(tool) {
        return true;
    }
    
    // 2. Check filesystem
    if let Ok(output) = std::process::Command::new("which").arg(tool).output() {
        if output.status.success() {
            TOOL_CACHE.insert(tool.to_string());
            return true;
        }
    }
    
    // 3. Check known locations
    for path in &["/usr/bin", "/usr/local/bin", "~/.cargo/bin"] {
        if std::path::Path::new(path).join(tool).exists() {
            TOOL_CACHE.insert(tool.to_string());
            return true;
        }
    }
    
    false
}

// ❌ Bad: Simple but unreliable
fn simple_checker(tool: &str) -> bool {
    tool == "clippy"  // Only checks one tool!
}
```

### Cache Tool Availability

```rust
use std::sync::{Arc, Mutex};
use std::collections::HashSet;

struct CachedToolChecker {
    cache: Arc<Mutex<HashSet<String>>>,
}

impl CachedToolChecker {
    fn new() -> Self {
        Self {
            cache: Arc::new(Mutex::new(HashSet::new())),
        }
    }
    
    fn check(&self, tool: &str) -> bool {
        // Check cache first
        if self.cache.lock().unwrap().contains(tool) {
            return true;
        }
        
        // Check system
        if system_has_tool(tool) {
            self.cache.lock().unwrap().insert(tool.to_string());
            true
        } else {
            false
        }
    }
}

fn system_has_tool(tool: &str) -> bool {
    std::process::Command::new("which")
        .arg(tool)
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}
```

## Registry Management

### Thread-Safe Registry

```rust
// ✅ Good: Thread-safe
use std::sync::{Arc, RwLock};

struct SafeRegistry {
    inner: Arc<RwLock<CapabilityRegistry>>,
}

impl SafeRegistry {
    fn new() -> Self {
        Self {
            inner: Arc::new(RwLock::new(CapabilityRegistry::new())),
        }
    }
    
    fn register(&self, caps: Capabilities) {
        self.inner.write().unwrap().register(caps);
    }
    
    fn find(&self, capability_type: &str, checker: impl Fn(&str) -> bool) -> Vec<Capabilities> {
        let registry = self.inner.read().unwrap();
        registry.find_with_capability(capability_type, &checker)
            .into_iter()
            .cloned()
            .collect()
    }
}
```

### Registry Cleanup

```rust
// ✅ Good: Remove stale workers
struct ManagedRegistry {
    registry: CapabilityRegistry,
    last_seen: HashMap<String, std::time::Instant>,
}

impl ManagedRegistry {
    fn cleanup_stale(&mut self, timeout: Duration) {
        let now = std::time::Instant::now();
        let stale: Vec<_> = self.last_seen.iter()
            .filter(|(_, last_seen)| now.duration_since(**last_seen) > timeout)
            .map(|(id, _)| id.clone())
            .collect();
        
        for id in stale {
            println!("Removing stale worker: {}", id);
            // Note: CapabilityRegistry doesn't have remove()
            // Need to rebuild registry or add remove() method
        }
    }
    
    fn heartbeat(&mut self, worker_id: &str) {
        self.last_seen.insert(worker_id.to_string(), std::time::Instant::now());
    }
}
```

## Error Handling

### Validate Capabilities Before Registration

```rust
// ✅ Good: Validate first
fn register_worker(registry: &mut CapabilityRegistry, caps: Capabilities) -> Result<(), String> {
    // Validate ID
    if caps.id.is_empty() {
        return Err("Worker ID cannot be empty".to_string());
    }
    
    // Validate has at least one tool
    if caps.all_tools().is_empty() {
        return Err("Worker must have at least one tool".to_string());
    }
    
    // Validate required tools are available
    if !caps.has_all_required_tools(&system_has_tool) {
        return Err(format!("Worker {} missing required tools", caps.id));
    }
    
    registry.register(caps);
    Ok(())
}
```

### Handle Missing Workers

```rust
// ✅ Good: Graceful handling
fn assign_task(registry: &CapabilityRegistry, task: &Task) -> Result<String, String> {
    let workers = registry.find_with_capability(&task.capability_type, &task.tool_checker);
    
    workers.first()
        .map(|w| w.id.clone())
        .ok_or_else(|| format!("No capable workers found for task type: {}", task.capability_type))
}
```

## Performance

### Minimize Tool Checking

```rust
// ✅ Good: Check once, cache result
let tool_cache: HashMap<String, bool> = HashMap::new();

fn cached_checker(tool: &str) -> bool {
    *tool_cache.entry(tool.to_string())
        .or_insert_with(|| system_has_tool(tool))
}

// ❌ Bad: Check every time
fn uncached_checker(tool: &str) -> bool {
    system_has_tool(tool)  // Slow system call every time!
}
```

### Batch Operations

```rust
// ✅ Good: Batch registration
fn register_workers_batch(registry: &mut CapabilityRegistry, workers: Vec<Capabilities>) {
    for caps in workers {
        registry.register(caps);
    }
}

// ❌ Bad: One at a time with overhead
fn register_workers_slow(registry: &mut CapabilityRegistry, workers: Vec<Capabilities>) {
    for caps in workers {
        check_network();  // Unnecessary overhead
        registry.register(caps);
        sync_to_db();     // Too frequent
    }
}
```

## Security

### Validate Worker Identity

```rust
// ✅ Good: Authenticate workers
fn register_authenticated_worker(
    registry: &mut CapabilityRegistry,
    caps: Capabilities,
    auth_token: &str,
) -> Result<(), String> {
    if !verify_worker_token(&caps.id, auth_token) {
        return Err("Invalid authentication token".to_string());
    }
    
    registry.register(caps);
    Ok(())
}
```

### Sanitize Metadata

```rust
// ✅ Good: Sanitize inputs
fn sanitize_capabilities(mut caps: Capabilities) -> Capabilities {
    // Limit metadata size
    caps.metadata = caps.metadata.into_iter()
        .take(20)  // Max 20 metadata entries
        .filter(|(k, v)| k.len() < 100 && v.len() < 500)  // Size limits
        .collect();
    
    caps
}
```

## Testing

### Mock Tool Checker

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    fn mock_checker(tool: &str) -> bool {
        matches!(tool, "clippy" | "rustfmt" | "cargo-audit")
    }
    
    #[test]
    fn test_capabilities() {
        let caps = Capabilities::new("test")
            .with_tool("clippy", true);
        
        assert!(caps.has_all_required_tools(&mock_checker));
    }
}
```

### Test All Capability Types

```rust
#[test]
fn test_all_capability_types() {
    let caps = Capabilities::new("comprehensive")
        .with_static_analysis("sa", true)
        .with_security_tool("sec", true)
        .with_dynamic_tool("dyn", true)
        .with_fuzzing_tool("fuzz", true)
        .with_test_framework("test", true);
    
    let checker = |_| true;
    
    assert!(caps.has_capability("static_analysis", &checker));
    assert!(caps.has_capability("security_scanning", &checker));
    assert!(caps.has_capability("dynamic_analysis", &checker));
    assert!(caps.has_capability("fuzzing", &checker));
    assert!(caps.has_capability("test_framework", &checker));
}
```

## Common Pitfalls

### ❌ Not Checking Tool Availability

```rust
// Bad: Assume tools are available
let caps = Capabilities::new("worker")
    .with_tool("rare-tool", true);

registry.register(caps);
// Worker registered but tool may not exist!

// Good: Verify first
if system_has_tool("rare-tool") {
    let caps = Capabilities::new("worker")
        .with_tool("rare-tool", true);
    registry.register(caps);
}
```

### ❌ Forgetting to Update Capabilities

```rust
// Bad: Static capabilities
let caps = Capabilities::new("worker")
    .with_tool("tool-v1", true);
registry.register(caps);
// Later: tool-v1 uninstalled, tool-v2 installed
// Capabilities are outdated!

// Good: Periodic refresh
async fn refresh_capabilities(registry: &mut CapabilityRegistry, worker_id: &str) {
    let updated_caps = detect_current_capabilities(worker_id);
    registry.register(updated_caps);  // Overwrites old
}
```

### ❌ Not Using Alternatives

```rust
// Bad: Exact match only
let caps = Capabilities::new("worker")
    .with_tool("rustfmt-nightly", true);
// Won't match workers with "rustfmt" or "cargo-fmt"

// Good: Accept alternatives
let caps = Capabilities::new("worker")
    .with_alternative("rustfmt", vec![
        "rustfmt",
        "cargo-fmt",
        "rustfmt-nightly"
    ]);
```

## Checklist

### For Defining Capabilities

- [ ] Use descriptive worker ID
- [ ] Mark tools as required/optional correctly
- [ ] Include alternative tools
- [ ] Add relevant flags
- [ ] Include useful metadata
- [ ] Verify tools before registering

### For Using Registry

- [ ] Use thread-safe wrapper for concurrent access
- [ ] Implement tool checker correctly
- [ ] Handle empty results gracefully
- [ ] Clean up stale entries periodically
- [ ] Cache tool availability checks

### For Production

- [ ] Authenticate worker registration
- [ ] Validate capability data
- [ ] Implement heartbeat mechanism
- [ ] Monitor registry health
- [ ] Log capability changes
- [ ] Implement backup/restore

## Conclusion

Following these best practices ensures:
- Reliable capability matching
- Efficient operation
- Robust error handling
- Production-ready system

See [User Guide](./user-guide.md) for usage patterns.


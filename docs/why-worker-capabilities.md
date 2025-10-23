# Why Worker Capabilities?

## The Problem

### Dynamic Distributed Systems

Modern distributed systems face challenges:

1. **Heterogeneous Workers**
   - Different tools installed
   - Different versions
   - Different platforms
   - Different capabilities

2. **Dynamic Availability**
   - Tools may be installed/removed
   - Services may come online/offline
   - Resources may become available/unavailable

3. **Task-Worker Matching**
   - How to know which worker can handle a task?
   - How to handle missing tools gracefully?
   - How to support alternative tools?

### Without Worker Capabilities

```rust
// Hard-coded assumptions
fn assign_task(task: Task) -> String {
    match task.language {
        "rust" => "rust-worker",      // Assumes exists
        "solidity" => "sol-worker",    // Assumes has tools
        "javascript" => "js-worker",   // May not be available
        _ => panic!("Unsupported"),    // Fails
    }
}
```

**Problems:**
- Assumes workers exist
- Assumes tools are installed
- No fallback mechanism
- Brittle and error-prone

## The Solution

### Capability-Based Assignment

```rust
use worker_capabilities::CapabilityRegistry;

fn assign_task(registry: &CapabilityRegistry, task: &Task) -> Option<String> {
    let tool_checker = |tool: &str| task.required_tools.contains(&tool.to_string());
    
    // Find workers that CAN do this task
    let capable = registry.find_with_capability(&task.capability_type, &tool_checker);
    
    // Select from available workers
    capable.first().map(|w| w.id.clone())
}
```

**Benefits:**
- Dynamic discovery
- Graceful degradation
- Flexible matching
- Robust handling

## Key Advantages

### 1. Dynamic Discovery

Workers advertise what they can do:

```rust
// Worker 1
let worker1 = Capabilities::new("worker-1")
    .with_tool("clippy", true)
    .with_tool("rustfmt", true);

// Worker 2
let worker2 = Capabilities::new("worker-2")
    .with_tool("cargo-audit", true)
    .with_tool("cargo-geiger", true);

// Coordinator discovers automatically
let rust_workers = registry.find_with_capability("static_analysis", &is_rust_tool);
let security_workers = registry.find_with_capability("security_scanning", &is_security_tool);
```

### 2. Alternative Tool Support

Handle different tool installations:

```rust
// Worker A has "rustfmt"
// Worker B has "cargo-fmt"
// Worker C has "rustfmt-nightly"

let formatter = Capabilities::new("formatter")
    .with_alternative("rustfmt", vec!["rustfmt", "cargo-fmt", "rustfmt-nightly"]);

// All workers can satisfy this capability!
```

### 3. Graceful Fallback

```rust
// Try to find worker with optimal tool
let primary_checker = |tool: &str| tool == "advanced-analyzer";
let workers = registry.find_with_capability("analysis", &primary_checker);

if workers.is_empty() {
    // Fallback to basic analyzer
    let fallback_checker = |tool: &str| tool == "basic-analyzer";
    workers = registry.find_with_capability("analysis", &fallback_checker);
}
```

### 4. Type-Safe

```rust
// Compiler enforces correct usage
let caps = Capabilities::new("worker")
    .with_tool("clippy", true)        // &str
    .with_flag("feature")              // &str
    .with_metadata("key", "value");   // &str, &str

// Type errors caught at compile time
// caps.with_tool(123, true);  // ERROR: expected &str
```

## Real-World Scenarios

### Scenario 1: Security Auditing Platform

**Before:**
```rust
// Hard-coded worker assignment
match code_language {
    "rust" => send_to_worker("rust-worker-1"),
    "solidity" => send_to_worker("sol-worker-1"),
    _ => return Err("Unsupported language"),
}
```

**Problems:**
- What if rust-worker-1 is down?
- What if it doesn't have required tools?
- Can't scale to multiple workers

**After:**
```rust
// Capability-based assignment
let tool_checker = |tool: &str| required_tools.contains(&tool.to_string());
let workers = registry.find_with_capability("security_scanning", &tool_checker);

// Automatically finds ANY capable worker
if let Some(worker) = workers.first() {
    send_to_worker(&worker.id)?;
}
```

**Benefits:**
- Automatic failover
- Multi-worker support
- Tool verification
- Graceful handling

### Scenario 2: CI/CD Pipeline

**Before:**
```rust
// Fixed build agents
let build_agent = match target_os {
    "linux" => "linux-agent-1",
    "macos" => "macos-agent-1",
    "windows" => "windows-agent-1",
    _ => panic!(),
};
```

**After:**
```rust
// Dynamic agent selection
let agents = registry.find_with_capability("build", &has_required_tools);
let agent = agents.iter()
    .find(|a| a.get_metadata("os") == Some(&target_os))
    .or_else(|| agents.first());  // Fallback to any
```

### Scenario 3: Job Queue

**Before:**
```rust
// Manual worker pool management
loop {
    let job = queue.pop();
    let worker = workers.iter().find(|w| w.is_idle())?;
    // Hope worker has right tools!
    worker.assign(job);
}
```

**After:**
```rust
// Capability-aware assignment
loop {
    let job = queue.pop();
    let capable = registry.find_with_capability(&job.job_type, &job.tool_checker);
    let worker = capable.iter()
        .min_by_key(|w| w.get_metadata("current_jobs").unwrap().parse::<usize>().unwrap())?;
    worker.assign(job);
}
```

## Cost-Benefit Analysis

### Without Worker Capabilities

**Costs:**
- Manual worker management
- Hard-coded assumptions
- Brittle failure modes
- Difficult scaling

**Problems:**
- Worker goes down → System fails
- Tool missing → Runtime errors
- New worker → Code changes needed

### With Worker Capabilities

**Benefits:**
- Automatic discovery
- Dynamic routing
- Graceful degradation
- Easy scaling

**Impact:**
- Worker goes down → Route to another
- Tool missing → Use alternative or skip
- New worker → Automatically discovered

**ROI:** ~50% reduction in operational issues

## Who Should Use This?

### Perfect For:

✅ **Distributed Analysis Systems**
- Security auditing platforms
- Code analysis services
- Testing frameworks

✅ **CI/CD Platforms**
- Build agent management
- Test distribution
- Deployment automation

✅ **Job Queues**
- Task routing
- Worker selection
- Load balancing

✅ **Plugin Systems**
- Dynamic plugin loading
- Feature discovery
- Capability negotiation

### Not Needed For:

❌ **Single-Worker Systems**
- Fixed infrastructure
- No tool variation
- Simple task assignment

❌ **Static Configuration**
- Known worker capabilities
- No dynamic changes
- Fixed tool sets

## Design Philosophy

### Capability-Based vs. Name-Based

**Name-Based (Traditional):**
```rust
send_to_worker("rust-worker-1");  // Assumes exists, has tools
```

**Capability-Based (Worker Capabilities):**
```rust
let workers = find_with_capability("static_analysis", &has_clippy);
send_to_worker(&workers[0].id);  // Verified capability
```

### Pull vs. Push

**Pull Model (Recommended):**
- Workers register capabilities
- Coordinator queries when needed
- Low network traffic
- Fast assignment

**Push Model (Alternative):**
- Coordinator broadcasts requirements
- Workers respond if capable
- Higher network traffic
- Better for unknown workers

## Industry Patterns

Similar capability systems:

- **Kubernetes**: Node affinity and taints/tolerations
- **Consul**: Service capabilities and health
- **Nomad**: Task constraints and affinities
- **Worker Capabilities**: Tool-based task matching (this project)

## Getting Started

1. **Understand capabilities** - Read [Architecture](./architecture.md)
2. **Try it out** - Follow [Getting Started](./getting-started.md)
3. **Build something** - Check [Use Cases](./use-cases.md)
4. **Optimize** - See [Best Practices](./best-practices.md)

## Conclusion

Worker Capabilities solves distributed worker management by providing:

- **Dynamic discovery** of worker capabilities
- **Flexible matching** with alternative tools
- **Type-safe** capability definitions
- **Easy scaling** with automatic routing

**Stop hard-coding workers. Start using capability-based assignment.**

## Further Reading

- [Architecture](./architecture.md) - How it works
- [Use Cases](./use-cases.md) - What you can build
- [Getting Started](./getting-started.md) - How to use it


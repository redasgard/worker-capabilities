# Architecture

## System Overview

Worker Capabilities implements a **capability-based task assignment system** that enables distributed workers to advertise their capabilities and allows coordinators to make intelligent routing decisions.

```
┌─────────────────────────────────────────────────────────────┐
│                    Task Coordinator                          │
│              (Assigns Tasks to Workers)                      │
└───────────────────┬──────────────────────────────────────────┘
                    │
                    │ Query: "Who can do static_analysis?"
                    ▼
┌─────────────────────────────────────────────────────────────┐
│                CapabilityRegistry                            │
│           (Stores Worker Capabilities)                       │
├─────────────────────────────────────────────────────────────┤
│  HashMap<WorkerID, Capabilities>                             │
│                                                               │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐       │
│  │ Worker 1     │  │ Worker 2     │  │ Worker N     │       │
│  ├──────────────┤  ├──────────────┤  ├──────────────┤       │
│  │ Capabilities │  │ Capabilities │  │ Capabilities │       │
│  │ - Tools      │  │ - Tools      │  │ - Tools      │       │
│  │ - Flags      │  │ - Flags      │  │ - Flags      │       │
│  │ - Metadata   │  │ - Metadata   │  │ - Metadata   │       │
│  └──────────────┘  └──────────────┘  └──────────────┘       │
└─────────────────────────────────────────────────────────────┘
                    │
                    │ Return: [Worker1, Worker3, ...]
                    ▼
┌─────────────────────────────────────────────────────────────┐
│               Task Assignment Logic                          │
│          (Route Task to Capable Workers)                     │
└─────────────────────────────────────────────────────────────┘
                    │
                    ▼
              Selected Workers
```

## Core Components

### 1. Capabilities

Container for all capabilities of a single worker.

**Structure:**
```rust
pub struct Capabilities {
    pub id: String,                              // Worker identifier
    pub static_analysis_tools: Vec<ToolCapability>,
    pub security_scanning_tools: Vec<ToolCapability>,
    pub dynamic_analysis_tools: Vec<ToolCapability>,
    pub fuzzing_tools: Vec<ToolCapability>,
    pub test_framework_tools: Vec<ToolCapability>,
    pub flags: HashMap<String, bool>,           // Feature flags
    pub metadata: HashMap<String, String>,      // Custom metadata
}
```

**Location:** `src/lib.rs`

### 2. ToolCapability

Individual tool capability with alternatives.

**Structure:**
```rust
pub struct ToolCapability {
    pub tool_name: String,                      // Primary tool
    pub required: bool,                         // Required vs optional
    pub alternatives: Vec<String>,              // Fallback tools
}
```

**Evaluation:**
```
Tool is satisfied if:
  (primary tool available) OR (any alternative available)
```

### 3. CapabilityRegistry

Global registry for managing multiple capability sets.

**Structure:**
```rust
pub struct CapabilityRegistry {
    capabilities: HashMap<String, Capabilities>,
}
```

**Operations:**
- `register(caps)` - Register a capability set
- `get(id)` - Retrieve by ID
- `list_ids()` - List all workers
- `find_with_capability(type, checker)` - Find capable workers

**Location:** `src/lib.rs`

## Capability Categories

### 1. Static Analysis Tools

```rust
capabilities.with_static_analysis("clippy", true)
```

**Examples:**
- Rust: clippy, rustc
- JavaScript: ESLint, TSLint
- Solidity: Slither, Solhint
- Python: Pylint, Mypy

### 2. Security Scanning Tools

```rust
capabilities.with_security_tool("cargo-audit", true)
```

**Examples:**
- Rust: cargo-audit, cargo-deny
- JavaScript: npm audit, Snyk
- Solidity: Mythril, Securify
- Python: Bandit, Safety

### 3. Dynamic Analysis Tools

```rust
capabilities.with_dynamic_tool("debugger", false)
```

**Examples:**
- Debuggers (gdb, lldb)
- Profilers (perf, valgrind)
- Tracers (strace, dtrace)
- Runtime analyzers

### 4. Fuzzing Tools

```rust
capabilities.with_fuzzing_tool("cargo-fuzz", false)
```

**Examples:**
- cargo-fuzz (Rust)
- AFL, LibFuzzer (C/C++)
- Echidna (Solidity)
- Hypothesis (Python)

### 5. Test Frameworks

```rust
capabilities.with_test_framework("pytest", true)
```

**Examples:**
- Rust: cargo test
- JavaScript: Jest, Mocha
- Python: pytest, unittest
- Solidity: Hardhat, Foundry

## Tool Satisfaction Algorithm

### Basic Check

```
is_satisfied(tool_checker):
    if tool_checker(primary_tool):
        return true
    
    for alternative in alternatives:
        if tool_checker(alternative):
            return true
    
    return false
```

### Capability Check

```
has_capability(capability_type, tool_checker):
    tools = get_tools_for_type(capability_type)
    
    if tools.is_empty():
        return false
    
    # At least one tool must be satisfied
    return any(tool.is_satisfied(tool_checker) for tool in tools)
```

### All Required Tools Check

```
has_all_required_tools(tool_checker):
    all_tools = static_analysis_tools 
              + security_scanning_tools
              + dynamic_analysis_tools
              + fuzzing_tools
              + test_framework_tools
    
    for tool in all_tools:
        if tool.required and not tool.is_satisfied(tool_checker):
            return false
    
    return true
```

## Data Flow

### Capability Registration

```
Worker Startup
    │
    ├─ Detect installed tools
    │   └─> run("which clippy")
    │
    ├─ Build Capabilities
    │   └─> Capabilities::new("worker1")
    │       .with_tool("clippy", true)
    │
    ├─ Register with Coordinator
    │   └─> registry.register(capabilities)
    │
    └─ Ready for tasks
```

### Task Assignment

```
Task Arrives: "Run static analysis on Rust code"
    │
    ├─ Extract requirements:
    │   └─> capability_type = "static_analysis"
    │
    ├─ Query registry:
    │   └─> registry.find_with_capability("static_analysis", tool_checker)
    │
    ├─ Filter by tool availability:
    │   └─> workers where tool_checker(worker_tools) = true
    │
    ├─ Select worker:
    │   └─> Based on load, priority, or round-robin
    │
    └─ Assign task to worker
```

## Builder Pattern Flow

```rust
Capabilities::new("worker")                 // Create with ID
  .with_static_analysis("clippy", true)    // Add required tool
  .with_security_tool("audit", false)      // Add optional tool
  .with_alternative("fmt", vec!["rustfmt", "cargo-fmt"]) // Add alternatives
  .with_flag("ast_support")                // Add feature flag
  .with_metadata("version", "1.0.0")       // Add metadata
```

Each method returns `Self`, enabling fluent chaining.

## Serialization

### JSON Format

```json
{
  "id": "rust-worker",
  "static_analysis_tools": [
    {
      "tool_name": "clippy",
      "required": true,
      "alternatives": []
    }
  ],
  "security_scanning_tools": [
    {
      "tool_name": "cargo-audit",
      "required": false,
      "alternatives": ["cargo-deny"]
    }
  ],
  "dynamic_analysis_tools": [],
  "fuzzing_tools": [],
  "test_framework_tools": [],
  "flags": {
    "ast_support": true,
    "llm_support": true
  },
  "metadata": {
    "version": "1.0.0",
    "platform": "linux"
  }
}
```

### Usage

```rust
// Serialize
let json = serde_json::to_string(&capabilities)?;

// Deserialize
let capabilities: Capabilities = serde_json::from_str(&json)?;

// Send over network
send_to_coordinator(json)?;
```

## Alternative Tools System

### Purpose

Handle different tool installations across workers:

```
Worker 1: Has "rustfmt"
Worker 2: Has "cargo-fmt"
Worker 3: Has "rustfmt-nightly"

All can satisfy "formatting" capability!
```

### Implementation

```rust
let caps = Capabilities::new("formatter")
    .with_alternative("rustfmt", vec![
        "rustfmt",
        "cargo-fmt",
        "rustfmt-nightly"
    ]);

// Worker 1
let worker1_checker = |tool: &str| tool == "rustfmt";
assert!(caps.has_capability("static_analysis", &worker1_checker));

// Worker 2
let worker2_checker = |tool: &str| tool == "cargo-fmt";
assert!(caps.has_capability("static_analysis", &worker2_checker));
```

## Performance Characteristics

### Memory Usage

```
Capabilities struct: ~200-500 bytes
CapabilityRegistry: ~200 bytes + (N × capability_size)

Example: 100 workers with 10 tools each:
  100 × 500 bytes = 50KB
```

### Lookup Performance

```
Operation                    | Time Complexity | Typical Time
-----------------------------|-----------------|-------------
register()                   | O(1)            | <1µs
get(id)                      | O(1)            | <1µs
has_capability()             | O(n)            | <10µs
find_with_capability()       | O(N×n)          | <100µs
```

Where:
- N = number of workers
- n = number of tools per worker

### Thread Safety

Not thread-safe by default. Use `Arc<Mutex<CapabilityRegistry>>` for concurrent access:

```rust
use std::sync::{Arc, Mutex};

let registry = Arc::new(Mutex::new(CapabilityRegistry::new()));

// In thread 1
{
    let mut reg = registry.lock().unwrap();
    reg.register(capabilities);
}

// In thread 2
{
    let reg = registry.lock().unwrap();
    let caps = reg.get("worker1");
}
```

## Design Patterns

### Pattern 1: Pull Model

Workers advertise capabilities, coordinator pulls:

```
Worker → Register capabilities → Registry
Coordinator → Query registry → Find capable worker → Assign task
```

### Pattern 2: Push Model

Coordinator broadcasts requirements, workers respond:

```
Coordinator → Broadcast task requirements
Workers → Check own capabilities → Respond if capable
Coordinator → Select from responses
```

### Pattern 3: Hybrid Model

Combine both for efficiency:

```
Registration Phase:
  Workers → Register static capabilities → Registry

Runtime Phase:
  Coordinator → Query registry (fast)
  Selected Workers → Dynamic availability check (slow)
```

## Future Enhancements

### v0.2
- Thread-safe registry (built-in Arc<RwLock>)
- Capability versioning
- Capability dependencies

### v0.3
- Network-aware capabilities (latency, bandwidth)
- Cost-based selection (prefer cheaper workers)
- Load balancing support

### v0.4
- Machine learning-based task assignment
- Predictive capability modeling
- Auto-scaling integration


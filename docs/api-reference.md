# API Reference

Complete API documentation for Worker Capabilities.

## Core Types

### Capabilities

Main capability definition for a worker.

```rust
pub struct Capabilities {
    pub id: String,
    pub static_analysis_tools: Vec<ToolCapability>,
    pub security_scanning_tools: Vec<ToolCapability>,
    pub dynamic_analysis_tools: Vec<ToolCapability>,
    pub fuzzing_tools: Vec<ToolCapability>,
    pub test_framework_tools: Vec<ToolCapability>,
    pub flags: HashMap<String, bool>,
    pub metadata: HashMap<String, String>,
}
```

#### Constructor

##### `new()`

```rust
pub fn new(id: impl Into<String>) -> Self
```

Create a new capability set with the given identifier.

**Parameters:**
- `id` - Unique worker identifier

**Returns:** New `Capabilities` instance

**Example:**
```rust
let caps = Capabilities::new("worker-1");
```

---

#### Builder Methods

##### `with_static_analysis()`

```rust
pub fn with_static_analysis(self, tool: impl Into<String>, required: bool) -> Self
```

Add a static analysis tool.

**Parameters:**
- `tool` - Tool name (e.g., "clippy", "eslint")
- `required` - Whether tool is required

**Example:**
```rust
let caps = Capabilities::new("worker")
    .with_static_analysis("clippy", true);
```

##### `with_security_tool()`

```rust
pub fn with_security_tool(self, tool: impl Into<String>, required: bool) -> Self
```

Add a security scanning tool.

**Example:**
```rust
let caps = Capabilities::new("worker")
    .with_security_tool("cargo-audit", true);
```

##### `with_dynamic_tool()`

```rust
pub fn with_dynamic_tool(self, tool: impl Into<String>, required: bool) -> Self
```

Add a dynamic analysis tool.

##### `with_fuzzing_tool()`

```rust
pub fn with_fuzzing_tool(self, tool: impl Into<String>, required: bool) -> Self
```

Add a fuzzing tool.

##### `with_test_framework()`

```rust
pub fn with_test_framework(self, tool: impl Into<String>, required: bool) -> Self
```

Add a test framework.

##### `with_tool()`

```rust
pub fn with_tool(self, tool: impl Into<String>, required: bool) -> Self
```

Add a generic tool (goes to static_analysis_tools).

##### `with_alternative()`

```rust
pub fn with_alternative(
    self,
    tool: impl Into<String>,
    alternatives: Vec<impl Into<String>>,
) -> Self
```

Add a tool with alternative options.

**Example:**
```rust
let caps = Capabilities::new("worker")
    .with_alternative("rustfmt", vec!["rustfmt", "cargo-fmt", "rustfmt-nightly"]);
```

##### `with_flag()`

```rust
pub fn with_flag(self, flag: impl Into<String>) -> Self
```

Add a capability flag.

**Example:**
```rust
let caps = Capabilities::new("worker")
    .with_flag("ast_support")
    .with_flag("llm_support");
```

##### `with_metadata()`

```rust
pub fn with_metadata(self, key: impl Into<String>, value: impl Into<String>) -> Self
```

Add metadata key-value pair.

**Example:**
```rust
let caps = Capabilities::new("worker")
    .with_metadata("version", "1.0.0")
    .with_metadata("platform", "linux");
```

---

#### Query Methods

##### `has_capability()`

```rust
pub fn has_capability(&self, capability_type: &str, tool_checker: &dyn Fn(&str) -> bool) -> bool
```

Check if worker has a specific capability type.

**Parameters:**
- `capability_type` - One of: "static_analysis", "security_scanning", "dynamic_analysis", "fuzzing", "test_framework"
- `tool_checker` - Function that checks if a tool is available

**Returns:** `bool` - true if at least one tool of this type is available

**Example:**
```rust
let checker = |tool: &str| tool == "clippy";
if caps.has_capability("static_analysis", &checker) {
    println!("Can do static analysis");
}
```

##### `has_all_required_tools()`

```rust
pub fn has_all_required_tools(&self, tool_checker: &dyn Fn(&str) -> bool) -> bool
```

Check if all required tools are available.

**Returns:** `bool` - true if all required tools are satisfied

**Example:**
```rust
let checker = |tool: &str| matches!(tool, "clippy" | "cargo-audit");
if caps.has_all_required_tools(&checker) {
    println!("Worker is operational");
}
```

##### `all_tools()`

```rust
pub fn all_tools(&self) -> Vec<String>
```

Get list of all tool names including alternatives.

**Returns:** `Vec<String>` - All tool names

**Example:**
```rust
let tools = caps.all_tools();
println!("Worker has {} tools", tools.len());
```

##### `has_flag()`

```rust
pub fn has_flag(&self, flag: &str) -> bool
```

Check if a specific flag is set.

**Returns:** `bool` - true if flag is set

**Example:**
```rust
if caps.has_flag("ast_support") {
    println!("Worker supports AST parsing");
}
```

##### `get_metadata()`

```rust
pub fn get_metadata(&self, key: &str) -> Option<&String>
```

Get metadata value by key.

**Returns:** `Option<&String>` - Metadata value if exists

**Example:**
```rust
if let Some(version) = caps.get_metadata("version") {
    println!("Worker version: {}", version);
}
```

---

## ToolCapability

Individual tool capability definition.

```rust
pub struct ToolCapability {
    pub tool_name: String,
    pub required: bool,
    pub alternatives: Vec<String>,
}
```

#### Methods

##### `new()`

```rust
pub fn new(tool_name: impl Into<String>, required: bool) -> Self
```

Create a new tool capability.

##### `with_alternatives()`

```rust
pub fn with_alternatives(self, alternatives: Vec<String>) -> Self
```

Add alternative tools.

##### `is_satisfied()`

```rust
pub fn is_satisfied(&self, tool_checker: &dyn Fn(&str) -> bool) -> bool
```

Check if this tool or any alternative is available.

**Returns:** `bool` - true if tool or alternative is available

---

## CapabilityRegistry

Registry for managing multiple capability sets.

```rust
pub struct CapabilityRegistry {
    capabilities: HashMap<String, Capabilities>,
}
```

#### Methods

##### `new()`

```rust
pub fn new() -> Self
```

Create a new empty registry.

**Example:**
```rust
let registry = CapabilityRegistry::new();
```

##### `register()`

```rust
pub fn register(&mut self, caps: Capabilities)
```

Register a capability set.

**Parameters:**
- `caps` - Capabilities to register

**Example:**
```rust
let worker = Capabilities::new("worker-1").with_tool("clippy", true);
registry.register(worker);
```

##### `get()`

```rust
pub fn get(&self, id: &str) -> Option<&Capabilities>
```

Get capabilities by worker ID.

**Returns:** `Option<&Capabilities>` - Capabilities if found

**Example:**
```rust
if let Some(caps) = registry.get("worker-1") {
    println!("Found worker");
}
```

##### `list_ids()`

```rust
pub fn list_ids(&self) -> Vec<String>
```

List all registered worker IDs.

**Returns:** `Vec<String>` - All worker IDs

**Example:**
```rust
let workers = registry.list_ids();
println!("Registered workers: {:?}", workers);
```

##### `find_with_capability()`

```rust
pub fn find_with_capability(
    &self,
    capability_type: &str,
    tool_checker: &dyn Fn(&str) -> bool,
) -> Vec<&Capabilities>
```

Find all workers with a specific capability.

**Parameters:**
- `capability_type` - Type of capability
- `tool_checker` - Tool availability checker

**Returns:** `Vec<&Capabilities>` - Matching workers

**Example:**
```rust
let checker = |tool: &str| tool == "clippy";
let workers = registry.find_with_capability("static_analysis", &checker);

for worker in workers {
    println!("Capable worker: {}", worker.id);
}
```

---

## Capability Types

Valid capability type strings:

- `"static_analysis"` - Static analysis tools
- `"security_scanning"` - Security scanning tools
- `"dynamic_analysis"` - Dynamic analysis tools
- `"fuzzing"` - Fuzzing tools
- `"test_framework"` - Test framework tools

---

## Thread Safety

Types are not thread-safe by default. For concurrent access, wrap in `Arc<Mutex<T>>`:

```rust
use std::sync::{Arc, Mutex};

let registry = Arc::new(Mutex::new(CapabilityRegistry::new()));

// Thread 1
{
    let mut reg = registry.lock().unwrap();
    reg.register(capabilities);
}

// Thread 2
{
    let reg = registry.lock().unwrap();
    let caps = reg.get("worker1");
}
```

---

## Serialization

All types implement `Serialize` and `Deserialize`:

```rust
use serde_json;

// Serialize
let json = serde_json::to_string(&capabilities)?;

// Deserialize
let capabilities: Capabilities = serde_json::from_str(&json)?;

// Pretty print
let pretty = serde_json::to_string_pretty(&capabilities)?;
```

---

## Example Patterns

### Worker Registration

```rust
fn register_worker(registry: &mut CapabilityRegistry, worker_id: &str) -> bool {
    let capabilities = detect_capabilities(worker_id);
    
    if capabilities.has_all_required_tools(&command_exists) {
        registry.register(capabilities);
        true
    } else {
        false
    }
}
```

### Task Matching

```rust
fn assign_task(registry: &CapabilityRegistry, task: &Task) -> Option<String> {
    let tool_checker = |tool: &str| task.required_tools.contains(&tool.to_string());
    
    registry.find_with_capability(&task.capability_type, &tool_checker)
        .first()
        .map(|w| w.id.clone())
}
```

---

## Performance Characteristics

| Operation | Time Complexity | Typical Time |
|-----------|----------------|--------------|
| `new()` | O(1) | <1µs |
| `with_*()` (builder) | O(1) | <1µs |
| `register()` | O(1) | <1µs |
| `get()` | O(1) | <1µs |
| `has_capability()` | O(n) | <10µs |
| `find_with_capability()` | O(N×n) | <100µs |

Where:
- N = number of workers
- n = number of tools per worker

---

## Version Compatibility

Current version: `0.1.0`

**Stability:** API is stable for v0.1.x releases


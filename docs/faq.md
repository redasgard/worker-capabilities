# FAQ

## General Questions

### What are Worker Capabilities?

A library for defining, registering, and discovering worker capabilities in distributed systems. Workers advertise what tools they have, and coordinators use this information for intelligent task assignment.

### Why do I need this?

If you have distributed workers with different tools installed, Worker Capabilities helps you:
- Automatically discover what each worker can do
- Route tasks to workers that can handle them
- Handle tool alternatives gracefully
- Avoid hard-coding worker assignments

### Is this thread-safe?

The core types (`Capabilities`, `ToolCapability`) are not thread-safe by default. `CapabilityRegistry` is also not thread-safe. For concurrent access, wrap in `Arc<Mutex<T>>` or `Arc<RwLock<T>>`.

## Usage Questions

### How do I define worker capabilities?

Use the builder pattern:

```rust
let caps = Capabilities::new("my-worker")
    .with_tool("clippy", true)
    .with_flag("feature");
```

### What are the capability types?

Five built-in types:
- `static_analysis` - Code analysis tools
- `security_scanning` - Security tools
- `dynamic_analysis` - Runtime analysis tools
- `fuzzing` - Fuzzing tools
- `test_framework` - Testing frameworks

### How do I check if a worker can do something?

```rust
let checker = |tool: &str| tool == "clippy";
if caps.has_capability("static_analysis", &checker) {
    // Worker can do it
}
```

### What's the difference between required and optional tools?

- **Required (true)**: Worker MUST have this tool
- **Optional (false)**: Worker MAY have this tool

Only required tools are checked by `has_all_required_tools()`.

### How do tool alternatives work?

If a tool has alternatives, any of them satisfies the capability:

```rust
let caps = Capabilities::new("worker")
    .with_alternative("rustfmt", vec!["rustfmt", "cargo-fmt"]);

// Either "rustfmt" OR "cargo-fmt" satisfies this
```

## Registry Questions

### How do I use the registry?

```rust
let mut registry = CapabilityRegistry::new();

// Register
registry.register(capabilities);

// Query
let workers = registry.find_with_capability("analysis", &checker);
```

### Can I have multiple registries?

Yes, create as many as needed:

```rust
let registry1 = CapabilityRegistry::new();  // For team A
let registry2 = CapabilityRegistry::new();  // For team B
```

### How do I remove a worker?

Current version doesn't have a `remove()` method. Workarounds:
1. Recreate registry without that worker
2. Implement your own wrapper with remove functionality
3. Use external storage (database) with CRUD operations

### Can I persist the registry?

Yes, serialize to JSON and save:

```rust
// Save
for id in registry.list_ids() {
    if let Some(caps) = registry.get(&id) {
        let json = serde_json::to_string(caps)?;
        save_to_file(&format!("{}.json", id), &json)?;
    }
}

// Load
let mut registry = CapabilityRegistry::new();
for file in read_json_files()? {
    let caps: Capabilities = serde_json::from_str(&file)?;
    registry.register(caps);
}
```

## Integration Questions

### Does this work with Docker?

Yes, include capabilities in Docker labels or environment variables. See [Integration Guide](./integration-guide.md).

### Does this work with Kubernetes?

Yes, use annotations or custom resources. See [Integration Guide](./integration-guide.md).

### Can I use this with gRPC?

Yes, define protobuf messages for capabilities and convert. See [Integration Guide](./integration-guide.md).

### Does this work with message queues?

Yes, serialize capabilities and send via RabbitMQ, Kafka, etc. See [Integration Guide](./integration-guide.md).

## Technical Questions

### Are capabilities immutable?

No, you can modify by creating new instances:

```rust
let caps1 = Capabilities::new("worker");
let caps2 = caps1.with_tool("new-tool", true);  // Returns new instance
```

### Can I clone capabilities?

Yes, `Capabilities` implements `Clone`:

```rust
let caps1 = Capabilities::new("worker").with_tool("tool", true);
let caps2 = caps1.clone();
```

### How are tool names matched?

Exact string matching. Tool checker must return true for exact tool name:

```rust
let checker = |tool: &str| tool == "clippy";  // Exact match
```

Case-sensitive by default.

### Can I use regex for tool matching?

Not built-in, but you can implement in your tool checker:

```rust
use regex::Regex;

let pattern = Regex::new(r"^cargo-.*$").unwrap();
let checker = |tool: &str| pattern.is_match(tool);
```

## Performance Questions

### How fast is capability checking?

Very fast:
- `has_capability()`: O(n) where n = number of tools, typically <10µs
- `find_with_capability()`: O(N×n) where N = workers, typically <100µs

### Does it scale to thousands of workers?

Yes, lookups are O(1) (HashMap), finding is O(N×n). For 1000 workers with 10 tools each:
- Lookup: ~1µs
- Find: ~10ms

For better performance with >1000 workers, consider database backend.

### Should I cache tool checks?

Yes! Tool checking (via `which`, file I/O) is slow. Cache results:

```rust
use std::collections::HashMap;

struct CachedChecker {
    cache: HashMap<String, bool>,
}

impl CachedChecker {
    fn check(&mut self, tool: &str) -> bool {
        *self.cache.entry(tool.to_string())
            .or_insert_with(|| expensive_tool_check(tool))
    }
}
```

## Troubleshooting

### Capabilities not matching

Check:
- Tool names are exact matches (case-sensitive)
- Tool checker returns true for the tools
- Required tools are marked correctly
- Alternatives are included if using variants

### All workers filtered out

Check:
- Tool checker logic is correct
- At least one worker has the required tools
- Required vs optional marked correctly

### Registration not working

Check:
- Worker ID is not empty
- Capabilities object is valid
- Registry is mutable (`&mut`)

## Best Practices

### Do:

- ✅ Use descriptive worker IDs
- ✅ Mark required/optional correctly
- ✅ Include tool alternatives
- ✅ Add useful metadata
- ✅ Implement robust tool checker
- ✅ Cache tool availability
- ✅ Handle missing workers gracefully

### Don't:

- ❌ Mark everything as required
- ❌ Forget alternatives
- ❌ Use generic IDs
- ❌ Skip metadata
- ❌ Check tools repeatedly
- ❌ Panic on missing workers

## Next Steps

- Read [Getting Started](./getting-started.md) for basic usage
- Check [User Guide](./user-guide.md) for comprehensive patterns
- See [Best Practices](./best-practices.md) for recommendations
- Review [Examples](./examples.md) for code samples

## Still Have Questions?

- Email: hello@redasgard.com
- GitHub Issues: https://github.com/redasgard/worker-capabilities/issues
- Documentation: `/docs/` directory


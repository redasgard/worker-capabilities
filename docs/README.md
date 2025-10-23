# Worker Capabilities Documentation

Welcome to the Worker Capabilities documentation. This library provides capability-aware workflow system for distributed workers with dynamic tool availability checking.

## Documentation Structure

- **[Architecture](./architecture.md)** - System design and capability model
- **[Getting Started](./getting-started.md)** - Quick start guide
- **[User Guide](./user-guide.md)** - Comprehensive usage patterns
- **[API Reference](./api-reference.md)** - Detailed API documentation
- **[Use Cases](./use-cases.md)** - Real-world applications
- **[Integration Guide](./integration-guide.md)** - Integrating with systems
- **[Best Practices](./best-practices.md)** - Recommended patterns
- **[FAQ](./faq.md)** - Frequently asked questions

## Quick Links

- [Why Worker Capabilities?](./why-worker-capabilities.md)
- [Capability Types](./capability-types.md)
- [Tool Alternatives](./tool-alternatives.md)
- [Examples](./examples.md)

## Overview

Worker Capabilities enables distributed systems to dynamically discover and utilize worker capabilities, allowing intelligent task assignment based on available tools and features.

### Key Features

- ✅ **Capability Registration**: Workers declare what they can do
- ✅ **Tool Availability**: Dynamic checking of tool availability
- ✅ **Builder Pattern**: Ergonomic API for defining capabilities
- ✅ **Alternative Tools**: Specify fallback tools
- ✅ **Serializable**: JSON support for capability exchange
- ✅ **Type-Safe**: Strongly typed capability definitions
- ✅ **Zero Dependencies**: Only serde for serialization

### Quick Example

```rust
use worker_capabilities::Capabilities;

// Define capabilities
let worker = Capabilities::new("rust-analyzer")
    .with_static_analysis("clippy", true)       // Required
    .with_security_tool("cargo-audit", false)   // Optional
    .with_flag("ast_support")
    .with_metadata("version", "1.0.0");

// Check capabilities
let tool_checker = |tool: &str| tool == "clippy";

if worker.has_capability("static_analysis", &tool_checker) {
    println!("Worker can perform static analysis!");
}
```

## Use Cases

- Distributed task assignment
- Worker capability negotiation
- Dynamic tool discovery
- Service mesh capabilities
- Plugin system management

## Support

- **GitHub**: https://github.com/redasgard/worker-capabilities
- **Email**: hello@redasgard.com

## License

MIT License - See [LICENSE-MIT](../LICENSE-MIT)


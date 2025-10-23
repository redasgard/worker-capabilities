# Tool Alternatives

Guide for working with alternative tools and fallback mechanisms.

## Why Alternatives?

Different workers may have different variants of the same tool:

```
Worker A: Has "rustfmt"
Worker B: Has "cargo-fmt"
Worker C: Has "rustfmt-nightly"

All should satisfy "formatting" capability!
```

## Defining Alternatives

### Basic Alternative

```rust
use worker_capabilities::Capabilities;

let caps = Capabilities::new("formatter")
    .with_alternative("rustfmt", vec![
        "rustfmt",
        "cargo-fmt"
    ]);
```

### Multiple Alternatives

```rust
let caps = Capabilities::new("worker")
    .with_alternative("compiler", vec![
        "gcc",
        "clang",
        "cc"
    ])
    .with_alternative("debugger", vec![
        "gdb",
        "lldb"
    ]);
```

## How Alternatives Work

### Satisfaction Logic

```rust
// Tool is satisfied if:
// 1. Primary tool is available, OR
// 2. Any alternative is available

let tool = ToolCapability {
    tool_name: "rustfmt",
    required: true,
    alternatives: vec!["cargo-fmt", "rustfmt-nightly"],
};

// Check primary
if tool_checker("rustfmt") {
    // Satisfied!
}

// Check alternatives
if tool_checker("cargo-fmt") {
    // Also satisfied!
}

if tool_checker("rustfmt-nightly") {
    // Also satisfied!
}
```

### Example

```rust
let caps = Capabilities::new("worker")
    .with_alternative("rustfmt", vec!["rustfmt", "cargo-fmt", "rustfmt-nightly"]);

// Worker 1: Has rustfmt
let checker1 = |tool: &str| tool == "rustfmt";
assert!(caps.has_capability("static_analysis", &checker1));

// Worker 2: Has cargo-fmt
let checker2 = |tool: &str| tool == "cargo-fmt";
assert!(caps.has_capability("static_analysis", &checker2));

// Worker 3: Has rustfmt-nightly
let checker3 = |tool: &str| tool == "rustfmt-nightly";
assert!(caps.has_capability("static_analysis", &checker3));

// Worker 4: Has none
let checker4 = |_: &str| false;
assert!(!caps.has_capability("static_analysis", &checker4));
```

## Common Alternative Patterns

### Version Variants

```rust
// Different versions of same tool
let caps = Capabilities::new("worker")
    .with_alternative("tool", vec![
        "tool",
        "tool-v2",
        "tool-latest",
        "tool-stable"
    ]);
```

### Command Variants

```rust
// Different ways to invoke same functionality
let caps = Capabilities::new("worker")
    .with_alternative("format", vec![
        "rustfmt",              // Direct command
        "cargo-fmt",            // Via cargo
        "cargo fmt"             // Alternative syntax
    ]);
```

### Platform-Specific

```rust
// Platform-specific equivalents
let caps = Capabilities::new("compiler")
    .with_alternative("cc", vec![
        "gcc",      // Linux
        "clang",    // macOS
        "cl.exe",   // Windows
        "cc"        // Generic
    ]);
```

### Package Manager Variants

```rust
// Different package managers
let caps = Capabilities::new("installer")
    .with_alternative("install", vec![
        "apt-get",   // Debian/Ubuntu
        "yum",       // RedHat/CentOS
        "brew",      // macOS
        "pacman",    // Arch
        "zypper"     // SUSE
    ]);
```

## Ordering Alternatives

### Priority Order

List alternatives in priority order (preferred first):

```rust
let caps = Capabilities::new("worker")
    .with_alternative("analyzer", vec![
        "advanced-analyzer",    // Preferred
        "standard-analyzer",    // Good
        "basic-analyzer"        // Fallback
    ]);
```

**Note:** Current implementation checks in order, so list preferred tools first.

## Best Practices

### 1. Include Reasonable Alternatives

```rust
// ✅ Good: Related variants
let caps = Capabilities::new("worker")
    .with_alternative("rustfmt", vec![
        "rustfmt",
        "cargo-fmt",
        "rustfmt-nightly"
    ]);

// ❌ Bad: Unrelated tools
let caps = Capabilities::new("worker")
    .with_alternative("rustfmt", vec![
        "rustfmt",
        "prettier",     // JavaScript formatter!
        "black"         // Python formatter!
    ]);
```

### 2. Don't Overdo It

```rust
// ✅ Good: Focused alternatives
let caps = Capabilities::new("worker")
    .with_alternative("linter", vec![
        "clippy",
        "cargo-clippy"
    ]);

// ❌ Bad: Too many alternatives
let caps = Capabilities::new("worker")
    .with_alternative("linter", vec![
        "clippy", "cargo-clippy", "clippy-preview",
        "clippy-nightly", "clippy-beta", "clippy-stable",
        "clippy-1.70", "clippy-1.71", "clippy-1.72"
        // ... 20 more variants
    ]);
```

### 3. Document Alternative Behavior

```rust
/// Creates capabilities for a formatter worker.
///
/// Accepts any of: rustfmt, cargo-fmt, rustfmt-nightly
/// All alternatives provide equivalent functionality.
fn create_formatter_capabilities(worker_id: &str) -> Capabilities {
    Capabilities::new(worker_id)
        .with_alternative("rustfmt", vec![
            "rustfmt",
            "cargo-fmt",
            "rustfmt-nightly"
        ])
}
```

## Alternative Selection Strategies

### Strategy 1: First Match

```rust
// First available alternative is used
fn find_first_available(alternatives: &[String]) -> Option<String> {
    for alt in alternatives {
        if tool_exists(alt) {
            return Some(alt.clone());
        }
    }
    None
}
```

### Strategy 2: Preferred Tool

```rust
// Try preferred, fallback to alternatives
fn select_tool(primary: &str, alternatives: &[String]) -> Option<String> {
    if tool_exists(primary) {
        return Some(primary.to_string());
    }
    
    find_first_available(alternatives)
}
```

### Strategy 3: Version-Based

```rust
// Select newest version
fn select_newest(alternatives: &[String]) -> Option<String> {
    alternatives.iter()
        .filter(|alt| tool_exists(alt))
        .max_by_key(|alt| get_tool_version(alt))
        .cloned()
}
```

## Testing with Alternatives

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_alternatives_satisfied() {
        let caps = Capabilities::new("worker")
            .with_alternative("tool", vec!["tool-a", "tool-b", "tool-c"]);
        
        // Test each alternative
        assert!(caps.has_capability("static_analysis", &|t| t == "tool-a"));
        assert!(caps.has_capability("static_analysis", &|t| t == "tool-b"));
        assert!(caps.has_capability("static_analysis", &|t| t == "tool-c"));
        
        // Test none available
        assert!(!caps.has_capability("static_analysis", &|t| t == "tool-d"));
    }
}
```

## Real-World Examples

### Cross-Platform Compiler

```rust
let caps = Capabilities::new("cross-platform-builder")
    .with_alternative("c_compiler", vec![
        "gcc",          // Linux (preferred)
        "clang",        // macOS
        "cl.exe",       // Windows MSVC
        "gcc.exe",      // Windows MinGW
        "cc"            // Generic fallback
    ])
    .with_metadata("supported_platforms", "linux,macos,windows");
```

### Multi-Version Support

```rust
let caps = Capabilities::new("node-worker")
    .with_alternative("node", vec![
        "node18",       // Preferred
        "node16",       // Supported
        "node14",       // Legacy support
        "node"          // System default
    ])
    .with_metadata("preferred_version", "18");
```

### Ecosystem Tools

```rust
let caps = Capabilities::new("rust-ecosystem")
    .with_alternative("package_manager", vec![
        "cargo",
        "cargo-binstall",   // Faster binary installs
        "cargo-quickinstall" // Precompiled binaries
    ])
    .with_alternative("formatter", vec![
        "rustfmt",
        "cargo-fmt"
    ])
    .with_alternative("linter", vec![
        "clippy",
        "cargo-clippy"
    ]);
```

## Limitations

### All-or-Nothing

Currently, alternatives are all-or-nothing:

```rust
// If any alternative is available, capability is satisfied
// Can't express: "need tool-a AND (tool-b OR tool-c)"
```

**Workaround:** Use multiple capability entries:

```rust
let caps = Capabilities::new("worker")
    .with_tool("tool-a", true)              // Required: tool-a
    .with_alternative("tool-b", vec![       // Required: tool-b OR tool-c
        "tool-b",
        "tool-c"
    ]);
```

### No Preference Indication

Can't indicate which alternative is preferred:

```rust
// All alternatives treated equally
// Can't say "prefer tool-a, but tool-b is acceptable"
```

**Workaround:** Order alternatives by preference (first = preferred).

## Future Enhancements

### v0.2
- Alternative prioritization
- Complex tool expressions (AND/OR/NOT)
- Capability groups

## Conclusion

Tool alternatives provide flexibility for heterogeneous worker environments. Use them to:
- Support different tool versions
- Handle platform differences
- Provide fallback options
- Maximize worker utilization

See [User Guide](./user-guide.md) for more patterns.


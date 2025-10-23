# Capability Types

Detailed explanation of different capability types.

## Overview

Worker Capabilities supports five main capability types:

1. Static Analysis
2. Security Scanning
3. Dynamic Analysis
4. Fuzzing
5. Test Frameworks

## 1. Static Analysis

### Description

Tools that analyze code without executing it.

### Common Tools

**Rust:**
- clippy - Linter
- rustc - Compiler warnings
- rust-analyzer - IDE analysis

**JavaScript/TypeScript:**
- ESLint - Linting
- TSLint - TypeScript linting
- JSHint - Code quality

**Solidity:**
- Slither - Static analyzer
- Solhint - Linter
- Securify - Security analyzer

**Python:**
- Pylint - Code analysis
- Mypy - Type checking
- Pyflakes - Error detection

### Usage

```rust
let caps = Capabilities::new("worker")
    .with_static_analysis("clippy", true)
    .with_static_analysis("rustc", false);

// Check
if caps.has_capability("static_analysis", &tool_checker) {
    // Can perform static analysis
}
```

### Use Cases

- Code quality checking
- Style enforcement
- Bug detection
- Type checking

---

## 2. Security Scanning

### Description

Tools that identify security vulnerabilities and risks.

### Common Tools

**Rust:**
- cargo-audit - Dependency vulnerabilities
- cargo-deny - License and advisory checking
- cargo-geiger - Unsafe code detection

**JavaScript:**
- npm audit - Package vulnerabilities
- Snyk - Dependency scanning
- OWASP Dependency-Check

**Solidity:**
- Mythril - Symbolic execution
- Manticore - Dynamic analysis
- Echidna - Fuzzing

**Python:**
- Bandit - Security linter
- Safety - Dependency vulnerabilities
- Semgrep - Pattern-based scanning

### Usage

```rust
let caps = Capabilities::new("security-worker")
    .with_security_tool("cargo-audit", true)
    .with_security_tool("cargo-geiger", false);

// Check
if caps.has_capability("security_scanning", &tool_checker) {
    // Can perform security scanning
}
```

### Use Cases

- Vulnerability detection
- Dependency auditing
- Security compliance
- Risk assessment

---

## 3. Dynamic Analysis

### Description

Tools that analyze code during execution.

### Common Tools

**Generic:**
- gdb - Debugger
- lldb - LLVM debugger
- valgrind - Memory analysis
- strace - System call tracing

**Rust:**
- cargo-watch - File watching
- cargo-expand - Macro expansion

**JavaScript:**
- Chrome DevTools
- Node Inspector

**Python:**
- pdb - Python debugger
- cProfile - Profiler

### Usage

```rust
let caps = Capabilities::new("debugger-worker")
    .with_dynamic_tool("gdb", true)
    .with_dynamic_tool("valgrind", false);

// Check
if caps.has_capability("dynamic_analysis", &tool_checker) {
    // Can perform dynamic analysis
}
```

### Use Cases

- Debugging
- Performance profiling
- Memory leak detection
- Runtime behavior analysis

---

## 4. Fuzzing

### Description

Tools that generate random inputs to find bugs and vulnerabilities.

### Common Tools

**Rust:**
- cargo-fuzz - libFuzzer frontend
- afl.rs - AFL integration
- honggfuzz - Google's fuzzer

**C/C++:**
- AFL - American Fuzzy Lop
- libFuzzer - LLVM fuzzer
- Hongfuzz

**Solidity:**
- Echidna - Smart contract fuzzer
- Foundry Fuzz - Foundry fuzzing
- Diligence Fuzzing

**Python:**
- Hypothesis - Property-based testing
- pythonfuzz - Fuzzing library

### Usage

```rust
let caps = Capabilities::new("fuzzer-worker")
    .with_fuzzing_tool("cargo-fuzz", true)
    .with_fuzzing_tool("afl", false);

// Check
if caps.has_capability("fuzzing", &tool_checker) {
    // Can perform fuzzing
}
```

### Use Cases

- Crash discovery
- Edge case finding
- Security testing
- Input validation

---

## 5. Test Frameworks

### Description

Tools and frameworks for running tests.

### Common Tools

**Rust:**
- cargo test - Built-in test framework
- cargo-nextest - Next-gen test runner

**JavaScript:**
- Jest - Testing framework
- Mocha - Test framework
- Jasmine - BDD framework

**Solidity:**
- Hardhat - Development environment
- Foundry - Testing framework
- Truffle - Development suite

**Python:**
- pytest - Testing framework
- unittest - Built-in testing
- nose2 - Test runner

### Usage

```rust
let caps = Capabilities::new("test-worker")
    .with_test_framework("cargo-test", true)
    .with_test_framework("cargo-nextest", false);

// Check
if caps.has_capability("test_framework", &tool_checker) {
    // Can run tests
}
```

### Use Cases

- Unit testing
- Integration testing
- Regression testing
- Test automation

---

## Custom Capability Types

While the five built-in types cover most use cases, you can use `with_tool()` for custom types:

```rust
let caps = Capabilities::new("custom-worker")
    .with_tool("custom-analyzer", true);  // Goes to static_analysis_tools
```

For completely custom capabilities, use flags and metadata:

```rust
let caps = Capabilities::new("specialized-worker")
    .with_flag("gpu_support")
    .with_flag("quantum_simulation")
    .with_flag("blockchain_integration")
    .with_metadata("gpu_type", "nvidia-a100")
    .with_metadata("quantum_backend", "qiskit");
```

## Capability Matrix

Common capability combinations:

| Worker Type | Static | Security | Dynamic | Fuzzing | Testing |
|-------------|--------|----------|---------|---------|---------|
| Full Stack | ✅ | ✅ | ✅ | ✅ | ✅ |
| Security Focused | ✅ | ✅ | ⚪ | ✅ | ⚪ |
| Testing Only | ⚪ | ⚪ | ⚪ | ⚪ | ✅ |
| Analyzer | ✅ | ⚪ | ⚪ | ⚪ | ⚪ |
| Debugger | ⚪ | ⚪ | ✅ | ⚪ | ⚪ |

## Best Practices

### 1. Use Appropriate Categories

```rust
// ✅ Good: Correct category
let caps = Capabilities::new("worker")
    .with_static_analysis("clippy", true)     // Linter
    .with_security_tool("cargo-audit", true); // Security

// ❌ Bad: Wrong category
let caps = Capabilities::new("worker")
    .with_fuzzing_tool("clippy", true);       // Clippy is not a fuzzer!
```

### 2. Include Complementary Tools

```rust
// ✅ Good: Related tools together
let caps = Capabilities::new("rust-worker")
    .with_static_analysis("clippy", true)
    .with_static_analysis("rustfmt", false)   // Formatting complements analysis
    .with_security_tool("cargo-audit", true);

// ❌ Bad: Unrelated tools
let caps = Capabilities::new("confused-worker")
    .with_static_analysis("clippy", true)
    .with_fuzzing_tool("solidity-fuzzer", true);  // Rust + Solidity = confusing
```

### 3. Mark Optional Tools Correctly

```rust
// ✅ Good: Core required, extras optional
let caps = Capabilities::new("worker")
    .with_static_analysis("rustc", true)      // Core: required
    .with_static_analysis("clippy", false)    // Enhancement: optional
    .with_security_tool("cargo-audit", false); // Extra: optional
```

## Conclusion

Understanding capability types enables:
- Correct tool categorization
- Effective worker matching
- Clear capability communication
- Maintainable systems

See [User Guide](./user-guide.md) for usage examples.


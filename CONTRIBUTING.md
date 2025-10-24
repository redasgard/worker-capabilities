# Contributing to Worker Capabilities

Thank you for your interest in contributing to Worker Capabilities! This document provides guidelines and information for contributors.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [How to Contribute](#how-to-contribute)
- [Development Setup](#development-setup)
- [Testing](#testing)
- [Security](#security)
- [Documentation](#documentation)
- [Release Process](#release-process)

## Code of Conduct

This project follows the [Contributor Covenant Code of Conduct](CODE_OF_CONDUCT.md). By participating, you agree to uphold this code.

## Getting Started

### Prerequisites

- Rust 1.70+ (latest stable recommended)
- Git
- Understanding of distributed systems and workflow orchestration
- Familiarity with async programming and task scheduling
- Basic knowledge of capability-based security and distributed coordination

### Fork and Clone

1. Fork the repository on GitHub
2. Clone your fork locally:
   ```bash
   git clone https://github.com/YOUR_USERNAME/worker-capabilities.git
   cd worker-capabilities
   ```
3. Add the upstream remote:
   ```bash
   git remote add upstream https://github.com/redasgard/worker-capabilities.git
   ```

## How to Contribute

### Reporting Issues

Before creating an issue, please:

1. **Search existing issues** to avoid duplicates
2. **Check the documentation** in the `docs/` folder
3. **Verify the issue** with the latest version
4. **Test with minimal examples**

When creating an issue, include:

- **Clear description** of the problem
- **Steps to reproduce** with code examples
- **Expected vs actual behavior**
- **Environment details** (OS, Rust version, worker setup)
- **Workflow-specific details** (if related to specific workflows)

### Suggesting Enhancements

For feature requests:

1. **Check existing issues** and roadmap
2. **Describe the use case** clearly
3. **Explain the distributed systems benefit**
4. **Consider implementation complexity**
5. **Provide workflow examples** if applicable

### Pull Requests

#### Before You Start

1. **Open an issue first** for significant changes
2. **Discuss the approach** with maintainers
3. **Ensure the change aligns** with project goals
4. **Consider distributed systems implications**

#### PR Process

1. **Create a feature branch** from `main`:
   ```bash
   git checkout -b feature/your-feature-name
   ```

2. **Make your changes** following our guidelines

3. **Test thoroughly**:
   ```bash
   cargo test
   cargo test --features tracing
   cargo clippy
   cargo fmt
   ```

4. **Update documentation** if needed

5. **Commit with clear messages**:
   ```bash
   git commit -m "Add support for workflow retry policies"
   ```

6. **Push and create PR**:
   ```bash
   git push origin feature/your-feature-name
   ```

#### PR Requirements

- **All tests pass** (CI will check)
- **Code is formatted** (`cargo fmt`)
- **No clippy warnings** (`cargo clippy`)
- **Documentation updated** if needed
- **Clear commit messages**
- **PR description** explains the change
- **Distributed systems compatibility** maintained

## Development Setup

### Project Structure

```
worker-capabilities/
├── src/                 # Source code
│   ├── lib.rs          # Main library interface
│   ├── registry.rs     # Capability registry
│   ├── workflow.rs     # Workflow management
│   ├── builder.rs      # Builder pattern implementation
│   └── types.rs        # Type definitions
├── tests/              # Integration tests
├── examples/           # Usage examples
└── docs/               # Documentation
```

### Running Tests

```bash
# Run all tests
cargo test

# Run with tracing
cargo test --features tracing

# Run specific test
cargo test test_workflow_execution

# Run examples
cargo run --example distributed_workflow
```

### Code Style

We follow standard Rust conventions:

- **Format code**: `cargo fmt`
- **Check linting**: `cargo clippy`
- **Use meaningful names**
- **Add documentation** for public APIs
- **Write tests** for new functionality
- **Consider async performance**

## Testing

### Test Categories

1. **Unit Tests**: Test individual functions
2. **Integration Tests**: Test complete workflows
3. **Distributed Tests**: Test with multiple workers
4. **Failure Tests**: Test fault tolerance
5. **Performance Tests**: Test async operations

### Adding Tests

When adding new functionality:

1. **Write unit tests** for each function
2. **Add integration tests** for workflows
3. **Test distributed scenarios**
4. **Test failure handling**
5. **Test async operations**

Example test structure:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_workflow_execution() {
        let registry = CapabilityRegistry::new();
        registry.register_worker("worker1", &["capability1"]).await?;
        
        let workflow = WorkflowBuilder::new()
            .add_step("step1", "capability1")
            .build()?;
        
        let result = workflow.execute(&registry).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_fault_tolerance() {
        let registry = CapabilityRegistry::new();
        registry.register_worker("worker1", &["capability1"]).await?;
        
        // Test worker failure handling
        let workflow = WorkflowBuilder::new()
            .add_step("step1", "capability1")
            .add_alternative("step1", "capability2")
            .build()?;
        
        let result = workflow.execute(&registry).await;
        assert!(result.is_ok());
    }
}
```

## Security

### Security Considerations

Worker Capabilities is a security-critical library. When contributing:

1. **Understand capability-based security** before making changes
2. **Test with malicious workers** (safely)
3. **Consider sandboxing** implications
4. **Review security implications** of changes
5. **Test with various worker types**

### Security Testing

```bash
# Run security tests
cargo test test_capability_validation
cargo test test_worker_authentication
cargo test test_malicious_worker_handling

# Test with examples
cargo run --example distributed_workflow
```

### Capability Security

When adding security features:

1. **Research capability-based security** best practices
2. **Understand sandboxing** techniques
3. **Test with malicious inputs**
4. **Consider privilege escalation** prevention
5. **Document security implications**

### Reporting Security Issues

**Do not open public issues for security vulnerabilities.**

Instead:
1. Email security@redasgard.com
2. Include detailed description
3. Include worker examples
4. Wait for response before disclosure

## Documentation

### Documentation Standards

- **Public APIs** must have doc comments
- **Examples** in doc comments should be runnable
- **Security implications** should be documented
- **Performance characteristics** should be noted
- **Distributed systems concepts** should be explained

### Documentation Structure

```
docs/
├── README.md              # Main documentation
├── getting-started.md      # Quick start guide
├── api-reference.md       # Complete API docs
├── distributed-systems.md # Distributed systems guide
├── best-practices.md      # Usage guidelines
└── faq.md                 # Frequently asked questions
```

### Writing Documentation

1. **Use clear, concise language**
2. **Include practical examples**
3. **Explain security implications**
4. **Document distributed systems concepts**
5. **Link to related resources**
6. **Keep it up to date**

## Release Process

### Versioning

We follow [Semantic Versioning](https://semver.org/):

- **MAJOR**: Breaking API changes
- **MINOR**: New features (backward compatible)
- **PATCH**: Bug fixes (backward compatible)

### Release Checklist

Before releasing:

- [ ] All tests pass
- [ ] Documentation updated
- [ ] CHANGELOG.md updated
- [ ] Version bumped in Cargo.toml
- [ ] Security review completed
- [ ] Performance benchmarks updated
- [ ] Distributed systems compatibility tested

### Release Steps

1. **Update version** in `Cargo.toml`
2. **Update CHANGELOG.md**
3. **Create release PR**
4. **Review and merge**
5. **Tag release** on GitHub
6. **Publish to crates.io**

## Areas for Contribution

### High Priority

- **Fault tolerance**: Improve worker failure handling
- **Performance improvements**: Optimize async operations and workflow execution
- **Security enhancements**: Better capability-based security
- **Workflow optimization**: Improve workflow scheduling and execution

### Medium Priority

- **Configuration options**: More flexible workflow configuration
- **Error handling**: Better error messages and recovery
- **Testing**: More comprehensive test coverage
- **Documentation**: Improve examples and guides

### Low Priority

- **CLI tools**: Command-line utilities for workflow management
- **Monitoring**: Workflow monitoring and observability
- **Visualization**: Workflow visualization tools
- **Hot reloading**: Runtime workflow updates

## Distributed Systems Development

### System Categories

1. **Workflow Orchestration**: Task scheduling and execution
2. **Capability Management**: Worker capability registration and discovery
3. **Fault Tolerance**: Worker failure handling and recovery
4. **Load Balancing**: Workload distribution across workers

### System Development Process

1. **Design**: Plan the distributed system architecture
2. **Implement**: Create the system components
3. **Test**: Test with multiple workers
4. **Validate**: Ensure fault tolerance and performance
5. **Document**: Document the system and its capabilities
6. **Deploy**: Make the system available

### System Testing

```rust
// Test new distributed system feature
#[tokio::test]
async fn test_new_distributed_feature() {
    let registry = CapabilityRegistry::new();
    
    // Test with multiple workers
    registry.register_worker("worker1", &["capability1"]).await?;
    registry.register_worker("worker2", &["capability2"]).await?;
    
    // Test distributed workflow
    let workflow = WorkflowBuilder::new()
        .add_step("step1", "capability1")
        .add_step("step2", "capability2")
        .build()?;
    
    let result = workflow.execute(&registry).await;
    assert!(result.is_ok());
}
```

## Getting Help

### Resources

- **Documentation**: Check the `docs/` folder
- **Examples**: Look at `examples/` folder
- **Issues**: Search existing GitHub issues
- **Discussions**: Use GitHub Discussions for questions

### Contact

- **Email**: hello@redasgard.com
- **GitHub**: [@redasgard](https://github.com/redasgard)
- **Security**: security@redasgard.com

## Recognition

Contributors will be:

- **Listed in CONTRIBUTORS.md**
- **Mentioned in release notes** for significant contributions
- **Credited in documentation** for major features
- **Acknowledged** for distributed systems development

Thank you for contributing to Worker Capabilities! ⚡🛡️

# Use Cases

Real-world applications and scenarios for Worker Capabilities.

## 1. Distributed Security Analysis Platform

### Scenario: Route Security Tasks to Specialized Workers

Distribute security analysis tasks to workers based on their tools and capabilities.

**Implementation:**

```rust
use worker_capabilities::{Capabilities, CapabilityRegistry};

struct SecurityPlatform {
    registry: CapabilityRegistry,
}

impl SecurityPlatform {
    fn new() -> Self {
        let mut registry = CapabilityRegistry::new();
        
        // Rust security worker
        registry.register(
            Capabilities::new("rust-security")
                .with_static_analysis("clippy", true)
                .with_security_tool("cargo-audit", true)
                .with_security_tool("cargo-geiger", false)
                .with_metadata("language", "rust")
                .with_metadata("max_concurrent", "5")
        );
        
        // Solidity security worker
        registry.register(
            Capabilities::new("solidity-security")
                .with_static_analysis("slither", true)
                .with_security_tool("mythril", true)
                .with_security_tool("manticore", false)
                .with_metadata("language", "solidity")
        );
        
        // JavaScript security worker
        registry.register(
            Capabilities::new("js-security")
                .with_static_analysis("eslint", true)
                .with_security_tool("npm-audit", true)
                .with_metadata("language", "javascript")
        );
        
        Self { registry }
    }
    
    fn assign_audit_task(&self, language: &str, required_tools: Vec<&str>) -> Option<String> {
        let tool_checker = move |tool: &str| required_tools.contains(&tool);
        
        // Find workers capable of security scanning
        let capable_workers = self.registry.find_with_capability(
            "security_scanning",
            &tool_checker
        );
        
        // Filter by language
        capable_workers.iter()
            .find(|w| w.get_metadata("language").map_or(false, |l| l == language))
            .map(|w| w.id.clone())
    }
}

fn main() {
    let platform = SecurityPlatform::new();
    
    // Assign Rust audit
    if let Some(worker) = platform.assign_audit_task("rust", vec!["clippy", "cargo-audit"]) {
        println!("Assigned Rust audit to: {}", worker);
    }
    
    // Assign Solidity audit
    if let Some(worker) = platform.assign_audit_task("solidity", vec!["slither", "mythril"]) {
        println!("Assigned Solidity audit to: {}", worker);
    }
}
```

**Benefits:**
- Intelligent task routing
- Language-specific workers
- Automatic capability matching
- Load distribution

## 2. CI/CD Build Agent Selection

### Scenario: Assign Build Jobs to Capable Agents

Select build agents based on their installed tools and platform.

**Implementation:**

```rust
use worker_capabilities::{Capabilities, CapabilityRegistry};

struct BuildOrchestrator {
    agents: CapabilityRegistry,
}

impl BuildOrchestrator {
    fn register_agents(&mut self) {
        // Linux agent
        self.agents.register(
            Capabilities::new("linux-agent-1")
                .with_tool("docker", true)
                .with_tool("cargo", true)
                .with_tool("gcc", true)
                .with_metadata("os", "linux")
                .with_metadata("arch", "x86_64")
                .with_metadata("cpu_cores", "16")
        );
        
        // macOS agent
        self.agents.register(
            Capabilities::new("macos-agent-1")
                .with_tool("xcodebuild", true)
                .with_tool("cargo", true)
                .with_metadata("os", "macos")
                .with_metadata("arch", "aarch64")
                .with_metadata("cpu_cores", "8")
        );
        
        // Windows agent
        self.agents.register(
            Capabilities::new("windows-agent-1")
                .with_tool("msbuild", true)
                .with_tool("cargo", true)
                .with_metadata("os", "windows")
                .with_metadata("arch", "x86_64")
        );
    }
    
    fn find_agent_for_build(&self, build_requirements: &BuildRequirements) -> Option<String> {
        let tool_checker = |tool: &str| {
            build_requirements.required_tools.contains(&tool.to_string())
        };
        
        self.agents.find_with_capability("static_analysis", &tool_checker)
            .iter()
            .find(|agent| {
                agent.get_metadata("os") == Some(&build_requirements.target_os)
            })
            .map(|agent| agent.id.clone())
    }
}

struct BuildRequirements {
    target_os: String,
    required_tools: Vec<String>,
}
```

**Benefits:**
- Platform-specific builds
- Tool requirement matching
- Resource optimization
- Parallel builds

## 3. Distributed Testing Framework

### Scenario: Distribute Tests Based on Framework Support

Route tests to workers that have the required test frameworks.

**Implementation:**

```rust
use worker_capabilities::{Capabilities, CapabilityRegistry};

struct TestOrchestrator {
    registry: CapabilityRegistry,
}

impl TestOrchestrator {
    fn new() -> Self {
        let mut registry = CapabilityRegistry::new();
        
        // Rust test worker
        registry.register(
            Capabilities::new("rust-tester")
                .with_test_framework("cargo-test", true)
                .with_test_framework("cargo-nextest", false)
                .with_fuzzing_tool("cargo-fuzz", false)
        );
        
        // JavaScript test worker
        registry.register(
            Capabilities::new("js-tester")
                .with_test_framework("jest", true)
                .with_test_framework("mocha", false)
        );
        
        // Python test worker
        registry.register(
            Capabilities::new("python-tester")
                .with_test_framework("pytest", true)
                .with_test_framework("unittest", true)
        );
        
        Self { registry }
    }
    
    fn assign_test_suite(&self, framework: &str) -> Vec<String> {
        let tool_checker = |tool: &str| tool == framework;
        
        self.registry.find_with_capability("test_framework", &tool_checker)
            .iter()
            .map(|w| w.id.clone())
            .collect()
    }
}
```

**Benefits:**
- Framework-specific routing
- Parallel test execution
- Dynamic worker selection
- Efficient resource usage

## 4. Plugin System with Capability Discovery

### Scenario: Load Plugins Based on Available Capabilities

Dynamically load and use plugins based on their advertised capabilities.

**Implementation:**

```rust
use worker_capabilities::Capabilities;

trait Plugin {
    fn name(&self) -> &str;
    fn capabilities(&self) -> Capabilities;
    fn execute(&self, task: &Task) -> Result<Output>;
}

struct PluginManager {
    plugins: Vec<Box<dyn Plugin>>,
}

impl PluginManager {
    fn load_plugins(&mut self) {
        // Plugins self-register their capabilities
        self.plugins.push(Box::new(RustAnalyzerPlugin));
        self.plugins.push(Box::new(SolidityAnalyzerPlugin));
    }
    
    fn find_plugin_for_task(&self, task: &Task) -> Option<&dyn Plugin> {
        let tool_checker = |tool: &str| {
            task.required_tools.contains(&tool.to_string())
        };
        
        self.plugins.iter()
            .find(|plugin| {
                let caps = plugin.capabilities();
                caps.has_capability(&task.capability_type, &tool_checker)
            })
            .map(|boxed| &**boxed)
    }
}

struct RustAnalyzerPlugin;

impl Plugin for RustAnalyzerPlugin {
    fn name(&self) -> &str {
        "rust-analyzer"
    }
    
    fn capabilities(&self) -> Capabilities {
        Capabilities::new("rust-analyzer")
            .with_static_analysis("clippy", true)
            .with_security_tool("cargo-audit", true)
    }
    
    fn execute(&self, task: &Task) -> Result<Output> {
        // Execute task
        Ok(Output::default())
    }
}

struct SolidityAnalyzerPlugin;

impl Plugin for SolidityAnalyzerPlugin {
    fn name(&self) -> &str {
        "solidity-analyzer"
    }
    
    fn capabilities(&self) -> Capabilities {
        Capabilities::new("solidity-analyzer")
            .with_static_analysis("slither", true)
            .with_security_tool("mythril", true)
    }
    
    fn execute(&self, task: &Task) -> Result<Output> {
        Ok(Output::default())
    }
}

struct Task {
    capability_type: String,
    required_tools: Vec<String>,
}

#[derive(Default)]
struct Output;
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
```

**Benefits:**
- Dynamic plugin loading
- Capability-based routing
- Extensible architecture
- Zero configuration

## 5. Service Mesh Capabilities

### Scenario: Advertise Microservice Capabilities

Microservices advertise their capabilities for service discovery.

**Implementation:**

```rust
use worker_capabilities::Capabilities;

struct Microservice {
    name: String,
    capabilities: Capabilities,
}

impl Microservice {
    fn new(name: &str) -> Self {
        let capabilities = match name {
            "auth-service" => {
                Capabilities::new(name)
                    .with_flag("authentication")
                    .with_flag("authorization")
                    .with_metadata("version", "2.0.0")
                    .with_metadata("max_rps", "10000")
            }
            "payment-service" => {
                Capabilities::new(name)
                    .with_flag("payment_processing")
                    .with_flag("refunds")
                    .with_metadata("providers", "stripe,paypal")
            }
            _ => Capabilities::new(name),
        };
        
        Self {
            name: name.to_string(),
            capabilities,
        }
    }
    
    fn can_handle(&self, feature: &str) -> bool {
        self.capabilities.has_flag(feature)
    }
}
```

**Benefits:**
- Service discovery
- Feature detection
- Version management
- Load balancing

## 6. Job Queue with Worker Matching

### Scenario: Match Jobs to Worker Capabilities

Intelligently assign jobs from queue to workers with matching capabilities.

**Implementation:**

```rust
use worker_capabilities::{Capabilities, CapabilityRegistry};

struct JobQueue {
    workers: CapabilityRegistry,
    pending_jobs: Vec<Job>,
}

impl JobQueue {
    fn assign_jobs(&mut self) -> Vec<(String, Job)> {
        let mut assignments = Vec::new();
        
        for job in &self.pending_jobs {
            if let Some(worker_id) = self.find_capable_worker(&job) {
                assignments.push((worker_id, job.clone()));
            }
        }
        
        assignments
    }
    
    fn find_capable_worker(&self, job: &Job) -> Option<String> {
        let tool_checker = |tool: &str| {
            job.required_tools.contains(&tool.to_string())
        };
        
        // Find workers with required capability
        let capable = self.workers.find_with_capability(&job.job_type, &tool_checker);
        
        // Select best worker (e.g., least loaded)
        capable.iter()
            .min_by_key(|w| {
                w.get_metadata("current_jobs")
                    .and_then(|j| j.parse::<usize>().ok())
                    .unwrap_or(0)
            })
            .map(|w| w.id.clone())
    }
}

#[derive(Clone)]
struct Job {
    id: String,
    job_type: String,
    required_tools: Vec<String>,
}
```

**Benefits:**
- Automatic job matching
- Load balancing
- Resource optimization
- Scalability

## Summary

Worker Capabilities is ideal for:

✅ **Distributed Systems** - Task routing and assignment
✅ **CI/CD Platforms** - Build agent selection
✅ **Testing Frameworks** - Test distribution
✅ **Plugin Systems** - Capability-based loading
✅ **Service Mesh** - Service discovery
✅ **Job Queues** - Worker matching

Choose Worker Capabilities when you need:
- Dynamic capability discovery
- Tool-based task routing
- Distributed worker management
- Flexible task assignment
- Alternative tool support

## Next Steps

- Review [Architecture](./architecture.md) for system design
- Check [Getting Started](./getting-started.md) for quick start
- See [Integration Guide](./integration-guide.md) for integration patterns
- Read [Best Practices](./best-practices.md) for recommended usage


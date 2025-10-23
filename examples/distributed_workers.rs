//! Example: Distributed worker system with capability matching

use worker_capabilities::{Capabilities, CapabilityRegistry};

fn main() {
    println!("=== Worker Capabilities: Distributed System Example ===\n");

    // Example 1: Define worker capabilities
    println!("1. Defining Worker Capabilities");
    println!("--------------------------------");

    let rust_worker = Capabilities::new("rust-worker-01")
        .with_static_analysis("clippy", true)
        .with_security_tool("cargo-audit", true)
        .with_fuzzing_tool("cargo-fuzz", false)
        .with_flag("ast_support")
        .with_flag("llm_support")
        .with_metadata("version", "1.0.0")
        .with_metadata("platform", "linux");

    println!("Rust Worker:");
    println!("  ID: {}", rust_worker.id);
    println!("  Static analysis tools: {}", rust_worker.static_analysis_tools.len());
    println!("  Security tools: {}", rust_worker.security_scanning_tools.len());
    println!("  Has AST support: {}", rust_worker.has_flag("ast_support"));

    let solidity_worker = Capabilities::new("solidity-worker-01")
        .with_static_analysis("slither", true)
        .with_security_tool("mythril", false)
        .with_security_tool("manticore", false)
        .with_flag("evm_support")
        .with_metadata("version", "1.0.0")
        .with_metadata("platform", "linux");

    println!("\nSolidity Worker:");
    println!("  ID: {}", solidity_worker.id);
    println!("  Has EVM support: {}", solidity_worker.has_flag("evm_support"));

    // Example 2: Tool availability checking
    println!("\n2. Tool Availability Checking");
    println!("------------------------------");

    // Simulate tool checker
    let available_tools = vec!["clippy", "cargo-audit", "slither"];
    let tool_checker = |tool: &str| available_tools.contains(&tool);

    println!("Available tools: {:?}", available_tools);
    println!("\nRust worker can do:");
    println!("  Static analysis? {}", rust_worker.has_capability("static_analysis", &tool_checker));
    println!("  Security scanning? {}", rust_worker.has_capability("security_scanning", &tool_checker));
    println!("  Fuzzing? {}", rust_worker.has_capability("fuzzing", &tool_checker));

    println!("\nSolidity worker can do:");
    println!("  Static analysis? {}", solidity_worker.has_capability("static_analysis", &tool_checker));
    println!("  Security scanning? {}", solidity_worker.has_capability("security_scanning", &tool_checker));

    // Example 3: Registry and capability matching
    println!("\n3. Worker Registry & Capability Matching");
    println!("-----------------------------------------");

    let mut registry = CapabilityRegistry::new();

    // Register multiple workers
    registry.register(rust_worker);
    registry.register(solidity_worker);

    let python_worker = Capabilities::new("python-worker-01")
        .with_static_analysis("pylint", true)
        .with_security_tool("bandit", true);

    registry.register(python_worker);

    println!("Registered {} workers", registry.list_ids().len());
    println!("Worker IDs: {:?}", registry.list_ids());

    // Find workers with static analysis capability
    println!("\nWorkers with static analysis capability:");
    let analyzers = registry.find_with_capability("static_analysis", &tool_checker);
    for worker in &analyzers {
        println!("  - {} (tools available: {})", 
            worker.id,
            worker.static_analysis_tools.iter()
                .filter(|t| tool_checker(&t.tool_name))
                .count()
        );
    }

    // Example 4: Alternative tools
    println!("\n4. Alternative Tools");
    println!("--------------------");

    let formatter = Capabilities::new("formatter-worker")
        .with_alternative("rustfmt", vec!["rustfmt", "cargo-fmt", "rustfmt-nightly"]);

    println!("Formatter worker accepts any of:");
    for tool in formatter.all_tools() {
        println!("  - {}", tool);
    }

    // Test with different tools available
    println!("\nWith 'cargo-fmt' available:");
    let has_fmt = formatter.has_capability("static_analysis", &|tool| tool == "cargo-fmt");
    println!("  Can format: {}", has_fmt);

    // Example 5: Required vs Optional tools
    println!("\n5. Required vs Optional Tools");
    println!("------------------------------");

    let worker_with_reqs = Capabilities::new("strict-worker")
        .with_static_analysis("required-tool", true)
        .with_security_tool("optional-tool", false);

    println!("Worker has:");
    println!("  Required: required-tool");
    println!("  Optional: optional-tool");

    // Only required tool available
    let partial_checker = |tool: &str| tool == "required-tool";
    println!("\nWith only required tool:");
    println!("  Meets requirements? {}", worker_with_reqs.has_all_required_tools(&partial_checker));

    // Neither available
    let no_tools = |_: &str| false;
    println!("\nWith no tools:");
    println!("  Meets requirements? {}", worker_with_reqs.has_all_required_tools(&no_tools));

    // Example 6: Metadata usage
    println!("\n6. Worker Metadata");
    println!("------------------");

    let worker = Capabilities::new("metadata-worker")
        .with_tool("analyzer", false)
        .with_metadata("version", "2.1.0")
        .with_metadata("platform", "linux-x86_64")
        .with_metadata("max_concurrent_jobs", "8")
        .with_metadata("memory_limit_mb", "4096");

    println!("Worker metadata:");
    if let Some(version) = worker.get_metadata("version") {
        println!("  Version: {}", version);
    }
    if let Some(platform) = worker.get_metadata("platform") {
        println!("  Platform: {}", platform);
    }
    if let Some(max_jobs) = worker.get_metadata("max_concurrent_jobs") {
        println!("  Max concurrent jobs: {}", max_jobs);
    }

    println!("\n=== Example completed successfully ===");
}


//! Constants for worker capabilities

// Default capability limits
pub const DEFAULT_MEMORY_LIMIT_MB: u64 = 128;
pub const DEFAULT_CPU_LIMIT_PERCENT: u8 = 50;
pub const DEFAULT_TIMEOUT_SECONDS: u64 = 30;

// Default expiration times
pub const DEFAULT_EXPIRATION_HOURS: u64 = 24;
pub const MAX_EXPIRATION_DAYS: u64 = 365;

// Attestation constants
pub const ATTESTATION_EXPIRY_DAYS: u64 = 365;
pub const DEFAULT_ATTESTATION_ALGORITHM: &str = "SHA256-RSA";

// Security limits
pub const MAX_TOOL_NAME_LENGTH: usize = 256;
pub const MAX_ALTERNATIVE_TOOLS: usize = 10;
pub const MAX_CAPABILITY_FLAGS: usize = 100;
pub const MAX_METADATA_ENTRIES: usize = 50;

// Hash constants
pub const CAPABILITY_HASH_LENGTH: usize = 64; // SHA256 hex length

// Registry limits
pub const MAX_REGISTERED_WORKERS: usize = 1000;
pub const MAX_TOOLS_PER_WORKER: usize = 100;

// Permission constants
pub const PERMISSION_FILESYSTEM_ACCESS: &str = "filesystem_access";
pub const PERMISSION_NETWORK_ACCESS: &str = "network_access";
pub const PERMISSION_PROCESS_SPAWN: &str = "process_spawn";
pub const PERMISSION_ENV_ACCESS: &str = "env_access";
pub const PERMISSION_SYSTEM_ACCESS: &str = "system_access";

// Capability types
pub const CAPABILITY_STATIC_ANALYSIS: &str = "static_analysis";
pub const CAPABILITY_SECURITY_SCANNING: &str = "security_scanning";
pub const CAPABILITY_DYNAMIC_ANALYSIS: &str = "dynamic_analysis";
pub const CAPABILITY_FUZZING: &str = "fuzzing";
pub const CAPABILITY_TEST_FRAMEWORK: &str = "test_framework";

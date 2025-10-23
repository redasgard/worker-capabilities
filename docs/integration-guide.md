# Integration Guide

Guide for integrating Worker Capabilities with various systems.

## gRPC/Protobuf Integration

### Proto Definition

```protobuf
syntax = "proto3";

message WorkerCapabilitiesProto {
    string id = 1;
    repeated ToolCapability static_analysis_tools = 2;
    repeated ToolCapability security_scanning_tools = 3;
    map<string, bool> flags = 4;
    map<string, string> metadata = 5;
}

message ToolCapability {
    string tool_name = 1;
    bool required = 2;
    repeated string alternatives = 3;
}
```

### Conversion

```rust
use worker_capabilities::Capabilities;

impl From<Capabilities> for WorkerCapabilitiesProto {
    fn from(caps: Capabilities) -> Self {
        WorkerCapabilitiesProto {
            id: caps.id,
            static_analysis_tools: caps.static_analysis_tools.into_iter()
                .map(|t| t.into())
                .collect(),
            // ... convert other fields
        }
    }
}

impl From<WorkerCapabilitiesProto> for Capabilities {
    fn from(proto: WorkerCapabilitiesProto) -> Self {
        Capabilities {
            id: proto.id,
            static_analysis_tools: proto.static_analysis_tools.into_iter()
                .map(|t| t.into())
                .collect(),
            // ... convert other fields
        }
    }
}
```

## REST API Integration

### HTTP Endpoints

```rust
use axum::{Json, Router, routing::{get, post}};
use worker_capabilities::{Capabilities, CapabilityRegistry};
use std::sync::{Arc, Mutex};

#[derive(Clone)]
struct AppState {
    registry: Arc<Mutex<CapabilityRegistry>>,
}

async fn register_worker(
    axum::extract::State(state): axum::extract::State<AppState>,
    Json(caps): Json<Capabilities>,
) -> Json<RegisterResponse> {
    state.registry.lock().unwrap().register(caps);
    
    Json(RegisterResponse {
        success: true,
        message: "Worker registered".to_string(),
    })
}

async fn find_workers(
    axum::extract::State(state): axum::extract::State<AppState>,
    Json(req): Json<FindRequest>,
) -> Json<Vec<Capabilities>> {
    let checker = |tool: &str| req.required_tools.contains(&tool.to_string());
    
    let registry = state.registry.lock().unwrap();
    let workers = registry.find_with_capability(&req.capability_type, &checker);
    
    Json(workers.into_iter().cloned().collect())
}

fn create_router() -> Router {
    let state = AppState {
        registry: Arc::new(Mutex::new(CapabilityRegistry::new())),
    };
    
    Router::new()
        .route("/workers/register", post(register_worker))
        .route("/workers/find", post(find_workers))
        .with_state(state)
}

#[derive(serde::Deserialize, serde::Serialize)]
struct RegisterResponse {
    success: bool,
    message: String,
}

#[derive(serde::Deserialize)]
struct FindRequest {
    capability_type: String,
    required_tools: Vec<String>,
}
```

## Message Queue Integration

### RabbitMQ Example

```rust
use lapin::{Channel, Connection, ConnectionProperties, BasicProperties};
use worker_capabilities::Capabilities;

async fn publish_capabilities(
    channel: &Channel,
    capabilities: &Capabilities,
) -> Result<(), lapin::Error> {
    let json = serde_json::to_string(capabilities).unwrap();
    
    channel.basic_publish(
        "",
        "worker.capabilities",
        Default::default(),
        json.as_bytes(),
        BasicProperties::default(),
    ).await?;
    
    Ok(())
}

async fn consume_capabilities(channel: &Channel) -> Result<(), lapin::Error> {
    let mut consumer = channel.basic_consume(
        "worker.capabilities",
        "coordinator",
        Default::default(),
        Default::default(),
    ).await?;
    
    while let Some(delivery) = consumer.next().await {
        let delivery = delivery?;
        
        let json = std::str::from_utf8(&delivery.data).unwrap();
        let caps: Capabilities = serde_json::from_str(json).unwrap();
        
        // Register capabilities
        println!("Received capabilities from {}", caps.id);
        
        delivery.ack(Default::default()).await?;
    }
    
    Ok(())
}
```

## Redis Integration

### Store Capabilities

```rust
use redis::AsyncCommands;
use worker_capabilities::Capabilities;

async fn store_capabilities(
    conn: &mut redis::aio::Connection,
    capabilities: &Capabilities,
) -> redis::RedisResult<()> {
    let json = serde_json::to_string(capabilities).unwrap();
    
    // Store with worker ID as key
    conn.set(&capabilities.id, json).await?;
    
    // Add to capability index
    for tool in capabilities.all_tools() {
        conn.sadd(format!("tool:{}", tool), &capabilities.id).await?;
    }
    
    Ok(())
}

async fn find_workers_with_tool(
    conn: &mut redis::aio::Connection,
    tool: &str,
) -> redis::RedisResult<Vec<String>> {
    conn.smembers(format!("tool:{}", tool)).await
}
```

## Kubernetes Integration

### Custom Resource Definition

```yaml
apiVersion: valkra.io/v1
kind: WorkerCapabilities
metadata:
  name: rust-worker-1
spec:
  id: rust-worker-1
  staticAnalysisTools:
    - toolName: clippy
      required: true
      alternatives: []
    - toolName: rustfmt
      required: false
      alternatives: ["cargo-fmt"]
  securityScanningTools:
    - toolName: cargo-audit
      required: true
  flags:
    ast_support: true
    llm_support: true
  metadata:
    version: "1.0.0"
    platform: "linux"
```

### Controller

```rust
use k8s_openapi::api::core::v1::Pod;
use kube::{Api, Client};
use worker_capabilities::Capabilities;

async fn sync_worker_capabilities(client: Client) -> anyhow::Result<()> {
    let pods: Api<Pod> = Api::default_namespaced(client);
    
    for pod in pods.list(&Default::default()).await? {
        if let Some(annotations) = pod.metadata.annotations {
            if let Some(caps_json) = annotations.get("worker.capabilities") {
                let caps: Capabilities = serde_json::from_str(caps_json)?;
                // Register in global registry
                println!("Registered k8s worker: {}", caps.id);
            }
        }
    }
    
    Ok(())
}
```

## Database Integration

### PostgreSQL Schema

```sql
CREATE TABLE worker_capabilities (
    id VARCHAR(255) PRIMARY KEY,
    static_analysis_tools JSONB,
    security_scanning_tools JSONB,
    dynamic_analysis_tools JSONB,
    fuzzing_tools JSONB,
    test_framework_tools JSONB,
    flags JSONB,
    metadata JSONB,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);

CREATE INDEX idx_tools ON worker_capabilities 
    USING GIN ((static_analysis_tools || security_scanning_tools));
```

### CRUD Operations

```rust
use sqlx::PgPool;
use worker_capabilities::Capabilities;

async fn save_capabilities(pool: &PgPool, caps: &Capabilities) -> sqlx::Result<()> {
    sqlx::query!(
        r#"
        INSERT INTO worker_capabilities (id, static_analysis_tools, flags, metadata)
        VALUES ($1, $2, $3, $4)
        ON CONFLICT (id) DO UPDATE SET
            static_analysis_tools = $2,
            flags = $3,
            metadata = $4,
            updated_at = NOW()
        "#,
        caps.id,
        serde_json::to_value(&caps.static_analysis_tools).unwrap(),
        serde_json::to_value(&caps.flags).unwrap(),
        serde_json::to_value(&caps.metadata).unwrap(),
    )
    .execute(pool)
    .await?;
    
    Ok(())
}

async fn load_capabilities(pool: &PgPool, id: &str) -> sqlx::Result<Capabilities> {
    let row = sqlx::query!(
        "SELECT * FROM worker_capabilities WHERE id = $1",
        id
    )
    .fetch_one(pool)
    .await?;
    
    Ok(Capabilities {
        id: row.id,
        static_analysis_tools: serde_json::from_value(row.static_analysis_tools).unwrap(),
        flags: serde_json::from_value(row.flags).unwrap(),
        metadata: serde_json::from_value(row.metadata).unwrap(),
        // ... other fields
    })
}
```

## WebSocket Integration

### Real-time Capability Updates

```rust
use tokio_tungstenite::tungstenite::Message;
use worker_capabilities::Capabilities;

async fn handle_websocket_message(message: Message, registry: &mut CapabilityRegistry) {
    if let Message::Text(text) = message {
        match serde_json::from_str::<CapabilityUpdate>(&text) {
            Ok(update) => match update.action {
                UpdateAction::Register => {
                    registry.register(update.capabilities);
                }
                UpdateAction::Update => {
                    // Update existing capabilities
                }
                UpdateAction::Unregister => {
                    // Remove from registry
                }
            },
            Err(e) => eprintln!("Failed to parse update: {}", e),
        }
    }
}

#[derive(serde::Deserialize)]
struct CapabilityUpdate {
    action: UpdateAction,
    capabilities: Capabilities,
}

#[derive(serde::Deserialize)]
enum UpdateAction {
    Register,
    Update,
    Unregister,
}
```

## Consul Integration

### Service Registration

```rust
use consul::Client;
use worker_capabilities::Capabilities;

async fn register_with_consul(
    client: &Client,
    capabilities: &Capabilities,
) -> Result<(), consul::Error> {
    // Register service
    client.register_service(&consul::ServiceRegistration {
        name: capabilities.id.clone(),
        tags: capabilities.all_tools(),
        meta: capabilities.metadata.clone(),
        // ... other fields
    }).await?;
    
    Ok(())
}

async fn find_service_by_tool(
    client: &Client,
    tool: &str,
) -> Result<Vec<String>, consul::Error> {
    let services = client.catalog_service(tool, None).await?;
    
    Ok(services.iter()
        .map(|s| s.service_name.clone())
        .collect())
}
```

## Docker Swarm Integration

### Service Labels

```dockerfile
FROM rust:latest

LABEL worker.capabilities='{"id":"rust-worker","static_analysis_tools":[{"tool_name":"clippy","required":true,"alternatives":[]}]}'
```

### Discovery

```rust
use bollard::Docker;
use worker_capabilities::Capabilities;

async fn discover_workers() -> Result<Vec<Capabilities>, bollard::errors::Error> {
    let docker = Docker::connect_with_local_defaults()?;
    let containers = docker.list_containers::<String>(None).await?;
    
    let mut workers = Vec::new();
    
    for container in containers {
        if let Some(labels) = container.labels {
            if let Some(caps_json) = labels.get("worker.capabilities") {
                if let Ok(caps) = serde_json::from_str::<Capabilities>(caps_json) {
                    workers.push(caps);
                }
            }
        }
    }
    
    Ok(workers)
}
```

## Next Steps

- Review [Architecture](./architecture.md) for design patterns
- Check [Use Cases](./use-cases.md) for practical examples
- See [Best Practices](./best-practices.md) for recommendations


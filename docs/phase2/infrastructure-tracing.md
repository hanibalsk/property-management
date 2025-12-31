# Story 87.3: Infrastructure Tracing Strategy

## Overview

This document outlines the strategy for implementing distributed tracing and observability across the Property Management System. Currently, the system uses basic logging via the `tracing` crate but lacks distributed tracing capabilities.

## Current Implementation Status

### Logging (Partial)

The codebase uses `tracing` crate for structured logging:

```rust
// Common pattern throughout the codebase
tracing::error!(error = %e, "Failed to create session");
tracing::warn!(sub = %claims.sub, "Failed to parse user_id from JWT claims");
```

**Current State:**
| Component | Status | Notes |
|-----------|--------|-------|
| `tracing` crate | Used | Basic logging |
| `tracing::instrument` | 1 file | `models/infrastructure.rs` only |
| OpenTelemetry | Not integrated | - |
| Distributed tracing | None | - |
| Metrics | None | - |
| Dashboards | None | - |

**Key File:**
- `backend/crates/db/src/models/infrastructure.rs` - Only file with `#[instrument]`

## Phase 2 Requirements

### 1. OpenTelemetry Integration

**Dependencies to add:**
```toml
# backend/Cargo.toml
[workspace.dependencies]
opentelemetry = "0.24"
opentelemetry-otlp = { version = "0.17", features = ["grpc-tonic"] }
opentelemetry-semantic-conventions = "0.16"
tracing-opentelemetry = "0.25"
```

**Initialization:**
```rust
// backend/crates/api-core/src/tracing.rs (new file)
use opentelemetry::trace::TracerProvider;
use opentelemetry_otlp::WithExportConfig;
use tracing_subscriber::layer::SubscriberExt;

pub fn init_telemetry(service_name: &str, otlp_endpoint: &str) -> Result<(), Box<dyn Error>> {
    // Create OTLP exporter
    let exporter = opentelemetry_otlp::new_exporter()
        .tonic()
        .with_endpoint(otlp_endpoint);

    // Create tracer provider
    let tracer = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(exporter)
        .with_trace_config(
            opentelemetry::sdk::trace::config()
                .with_resource(Resource::new(vec![
                    KeyValue::new("service.name", service_name.to_string()),
                ]))
        )
        .install_batch(opentelemetry::runtime::Tokio)?;

    // Set up tracing subscriber with OpenTelemetry layer
    let telemetry_layer = tracing_opentelemetry::layer()
        .with_tracer(tracer);

    let subscriber = Registry::default()
        .with(telemetry_layer)
        .with(tracing_subscriber::fmt::layer());

    tracing::subscriber::set_global_default(subscriber)?;
    Ok(())
}
```

### 2. Request Tracing Middleware

**Axum middleware for trace context:**
```rust
// backend/crates/api-core/src/middleware/tracing.rs
use axum::{
    extract::Request,
    middleware::Next,
    response::Response,
};
use opentelemetry::propagation::TextMapPropagator;
use opentelemetry_sdk::propagation::TraceContextPropagator;
use tracing::Instrument;

pub async fn trace_request(req: Request, next: Next) -> Response {
    let propagator = TraceContextPropagator::new();

    // Extract trace context from headers
    let parent_context = propagator.extract(&HeaderExtractor(req.headers()));

    // Create span with request info
    let span = tracing::info_span!(
        "http_request",
        http.method = %req.method(),
        http.route = %req.uri().path(),
        http.status_code = tracing::field::Empty,
        otel.kind = "server",
    );

    // Execute request within span
    let response = next.run(req).instrument(span.clone()).await;

    // Record response status
    span.record("http.status_code", response.status().as_u16());

    response
}
```

### 3. Database Query Tracing

**Instrument SQLx queries:**
```rust
// backend/crates/db/src/lib.rs
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use tracing::instrument;

pub async fn create_pool(database_url: &str) -> Result<PgPool, sqlx::Error> {
    let options = PgConnectOptions::from_str(database_url)?
        .log_statements(log::LevelFilter::Debug)
        .log_slow_statements(log::LevelFilter::Warn, Duration::from_secs(1));

    PgPoolOptions::new()
        .max_connections(10)
        .connect_with(options)
        .await
}

// Example repository method with tracing
#[instrument(skip(self), fields(user_id = %user_id))]
pub async fn find_by_id(&self, user_id: Uuid) -> Result<Option<User>, sqlx::Error> {
    sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", user_id)
        .fetch_optional(&self.pool)
        .await
}
```

### 4. External Service Tracing

**Instrument HTTP clients:**
```rust
// backend/crates/integrations/src/llm.rs
use tracing::instrument;

impl LlmClient {
    #[instrument(skip(self, request), fields(
        llm.provider = "openai",
        llm.model = %request.model,
        llm.tokens_used = tracing::field::Empty,
    ))]
    pub async fn openai_chat(
        &self,
        request: &ChatCompletionRequest,
    ) -> Result<ChatCompletionResponse, LlmError> {
        let response = self.http_client
            .post("https://api.openai.com/v1/chat/completions")
            // ... existing code ...
            .await?;

        // Record token usage in span
        tracing::Span::current()
            .record("llm.tokens_used", response.usage.total_tokens);

        Ok(response)
    }
}
```

### 5. Trace Context Propagation

**Propagate context to downstream services:**
```rust
// When calling external services
use opentelemetry::propagation::TextMapPropagator;

async fn call_external_service(&self, url: &str) -> Result<Response, Error> {
    let propagator = TraceContextPropagator::new();

    // Inject current context into headers
    let mut headers = HeaderMap::new();
    propagator.inject_context(
        &opentelemetry::Context::current(),
        &mut HeaderInjector(&mut headers),
    );

    self.client
        .get(url)
        .headers(headers)
        .send()
        .await
}
```

## Metrics Collection

### Prometheus Metrics

**Add metrics endpoint:**
```rust
// backend/servers/api-server/src/routes/metrics.rs
use axum::routing::get;
use prometheus::{Encoder, TextEncoder, Registry};

lazy_static! {
    static ref HTTP_REQUESTS: CounterVec = CounterVec::new(
        Opts::new("http_requests_total", "Total HTTP requests"),
        &["method", "path", "status"]
    ).unwrap();

    static ref HTTP_DURATION: HistogramVec = HistogramVec::new(
        HistogramOpts::new("http_request_duration_seconds", "HTTP request duration"),
        &["method", "path"]
    ).unwrap();

    static ref DB_QUERY_DURATION: HistogramVec = HistogramVec::new(
        HistogramOpts::new("db_query_duration_seconds", "Database query duration"),
        &["query_type"]
    ).unwrap();

    static ref LLM_TOKENS_USED: CounterVec = CounterVec::new(
        Opts::new("llm_tokens_total", "Total LLM tokens used"),
        &["provider", "model", "type"]  // type: input/output
    ).unwrap();
}

pub fn router() -> Router<AppState> {
    Router::new().route("/metrics", get(metrics_handler))
}

async fn metrics_handler() -> impl IntoResponse {
    let encoder = TextEncoder::new();
    let mut buffer = Vec::new();
    encoder.encode(&REGISTRY.gather(), &mut buffer).unwrap();
    String::from_utf8(buffer).unwrap()
}
```

### Key Metrics to Track

| Metric | Type | Labels | Purpose |
|--------|------|--------|---------|
| `http_requests_total` | Counter | method, path, status | Request volume |
| `http_request_duration_seconds` | Histogram | method, path | Latency |
| `db_query_duration_seconds` | Histogram | query_type | DB performance |
| `db_connections_active` | Gauge | - | Pool utilization |
| `llm_tokens_total` | Counter | provider, model, type | AI cost tracking |
| `llm_request_duration_seconds` | Histogram | provider, model | AI latency |
| `auth_failures_total` | Counter | reason | Security monitoring |
| `websocket_connections` | Gauge | - | Real-time connections |

## Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                        Applications                              │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────────┐ │
│  │ api-server  │  │reality-srvr │  │ Background Jobs          │ │
│  └──────┬──────┘  └──────┬──────┘  └────────────┬────────────┘ │
└─────────┼────────────────┼─────────────────────┼───────────────┘
          │                │                     │
          │ OTLP (gRPC)    │ OTLP (gRPC)        │ OTLP (gRPC)
          v                v                     v
┌─────────────────────────────────────────────────────────────────┐
│                     OpenTelemetry Collector                      │
│  ┌────────────┐  ┌────────────────┐  ┌───────────────────────┐ │
│  │ Receivers  │→ │ Processors     │→ │ Exporters             │ │
│  │ - OTLP     │  │ - Batch        │  │ - Jaeger/Tempo        │ │
│  │ - Prom     │  │ - Memory Limit │  │ - Prometheus          │ │
│  └────────────┘  └────────────────┘  │ - Loki (logs)         │ │
│                                       └───────────────────────┘ │
└─────────────────────────────────────────────────────────────────┘
                          │                      │
          ┌───────────────┼──────────────────────┼───────────────┐
          v               v                      v               │
┌──────────────┐  ┌──────────────┐  ┌──────────────┐  ┌────────┐ │
│    Jaeger    │  │  Prometheus  │  │     Loki     │  │Grafana │ │
│   (Traces)   │  │  (Metrics)   │  │   (Logs)     │  │(Dash)  │ │
└──────────────┘  └──────────────┘  └──────────────┘  └────────┘ │
└────────────────────────────────────────────────────────────────┘
```

## Deployment Configuration

### Docker Compose (Development)

```yaml
# docker-compose.observability.yml
version: '3.8'

services:
  otel-collector:
    image: otel/opentelemetry-collector-contrib:0.92.0
    ports:
      - "4317:4317"   # OTLP gRPC
      - "4318:4318"   # OTLP HTTP
    volumes:
      - ./otel-config.yaml:/etc/otelcol-contrib/config.yaml

  jaeger:
    image: jaegertracing/all-in-one:1.53
    ports:
      - "16686:16686"  # UI
      - "14250:14250"  # gRPC

  prometheus:
    image: prom/prometheus:v2.48.0
    ports:
      - "9090:9090"
    volumes:
      - ./prometheus.yml:/etc/prometheus/prometheus.yml

  grafana:
    image: grafana/grafana:10.2.3
    ports:
      - "3030:3000"
    environment:
      - GF_AUTH_ANONYMOUS_ENABLED=true
      - GF_AUTH_ANONYMOUS_ORG_ROLE=Admin
```

### Kubernetes (Production)

```yaml
# Use Grafana Tempo for traces in production
apiVersion: v1
kind: ConfigMap
metadata:
  name: otel-collector-config
data:
  config.yaml: |
    receivers:
      otlp:
        protocols:
          grpc:
            endpoint: 0.0.0.0:4317

    processors:
      batch:
        timeout: 10s

    exporters:
      otlp/tempo:
        endpoint: tempo:4317
        tls:
          insecure: true
      prometheus:
        endpoint: "0.0.0.0:8889"

    service:
      pipelines:
        traces:
          receivers: [otlp]
          processors: [batch]
          exporters: [otlp/tempo]
        metrics:
          receivers: [otlp]
          processors: [batch]
          exporters: [prometheus]
```

## Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `OTEL_EXPORTER_OTLP_ENDPOINT` | Collector endpoint | `http://localhost:4317` |
| `OTEL_SERVICE_NAME` | Service name for traces | `api-server` |
| `OTEL_TRACES_SAMPLER` | Sampling strategy | `parentbased_traceidratio` |
| `OTEL_TRACES_SAMPLER_ARG` | Sampling ratio | `0.1` (10%) |
| `METRICS_ENABLED` | Enable Prometheus metrics | `true` |
| `LOG_FORMAT` | Log format (json/pretty) | `json` |
| `LOG_LEVEL` | Minimum log level | `info` |

## Implementation Plan

### Phase 2a: Basic Tracing
1. Add OpenTelemetry dependencies
2. Initialize tracer in main.rs
3. Add request tracing middleware
4. Instrument key repository methods

### Phase 2b: Metrics
1. Add Prometheus dependencies
2. Create metrics endpoint
3. Instrument HTTP handlers
4. Add database metrics

### Phase 2c: Dashboards
1. Deploy observability stack
2. Create Grafana dashboards
3. Set up alerting rules
4. Document runbooks

## Testing

1. **Unit Tests:** Mock tracer for testing instrumentation
2. **Integration Tests:** Verify trace context propagation
3. **Load Tests:** Measure tracing overhead (<2% latency impact)

## References

- [OpenTelemetry Rust](https://opentelemetry.io/docs/languages/rust/)
- [tracing-opentelemetry](https://docs.rs/tracing-opentelemetry/)
- [Grafana Tempo](https://grafana.com/oss/tempo/)
- [Prometheus Best Practices](https://prometheus.io/docs/practices/naming/)

//! Tracing example demonstrating logging and instrumentation
//!
//! Run with: `RUST_LOG=trace cargo run --example tracing --features tracing`

#![cfg(feature = "tracing")]

use {
  prjenv::prelude::*,
  std::{
    thread::sleep,
    time::{
      Duration,
      Instant,
    },
  },
  tracing::{
    debug,
    debug_span,
    error,
    info,
    info_span,
    instrument,
    trace,
    warn,
  },
  tracing_subscriber::{
    EnvFilter,
    fmt::layer,
    layer::SubscriberExt,
    registry,
    util::SubscriberInitExt,
  },
};

fn main() {
  // Initialize tracing subscriber
  setup_tracing();

  println!("🚀 prjenv - Tracing Example\n");

  //╔═══════════════════════════════════════════════════════════╗
  //║ Example 1: Traced Environment Initialization              ║
  //╚═══════════════════════════════════════════════════════════╝
  info!("Starting environment initialization");

  let env = get();
  debug!(
    kind = ?env.kind,
    workspace = %env.workspace.metadata.name,
    package = %env.package.metadata.name,
    "Environment initialized"
  );

  println!("📦 Environment Initialized:");
  println!("  Kind: {:?}", env.kind);
  println!("  Workspace: {}", env.workspace.metadata.display_name());
  println!("  Package: {}", env.package.metadata.display_name());
  println!();

  //╔═══════════════════════════════════════════════════════════╗
  //║ Example 2: Workspace Discovery with Tracing               ║
  //╚═══════════════════════════════════════════════════════════╝
  info!("Discovering workspace root");

  let root = find_cargo_root();
  debug!(path = %root.display(), "Workspace root discovered");

  println!("📁 Workspace Discovery:");
  println!("  Root: {}", root.display());

  let cargo_toml = root.join("Cargo.toml");
  let is_workspace = is_workspace_toml(&cargo_toml);
  trace!(
    path = %cargo_toml.display(),
    is_workspace = is_workspace,
    "Checked Cargo.toml"
  );

  println!("  Is workspace: {is_workspace}");
  println!();

  //╔═══════════════════════════════════════════════════════════╗
  //║ Example 3: Metadata Loading with Instrumentation          ║
  //╚═══════════════════════════════════════════════════════════╝
  println!("📝 Metadata Loading (check trace logs):");

  // This will trigger traced metadata loading
  let metadata = get_cached_workspace_metadata();

  println!("  Cached workspace metadata:");
  println!("    Name: {}", metadata.name);
  println!("    Version: {}", metadata.version);
  println!();

  //╔═══════════════════════════════════════════════════════════╗
  //║ Example 4: Configuration with Logging                     ║
  //╚═══════════════════════════════════════════════════════════╝
  info!("Creating configuration");

  let cfg = Configuration::new()
    .with_db("sqlite:///tmp/example.db")
    .with_port(8080_u16)
    .with_ip("127.0.0.1");

  debug!(
    db = %cfg.db,
    ip = %cfg.ip,
    port = cfg.port,
    "Configuration created"
  );

  println!("⚙️  Configuration:");
  println!("  Database: {}", cfg.db);
  println!("  Server: {}:{}", cfg.ip, cfg.port);
  println!();

  //╔═══════════════════════════════════════════════════════════╗
  //║ Example 5: Structured Logging Example                     ║
  //╚═══════════════════════════════════════════════════════════╝
  println!("📊 Structured Logging Example:");

  trace!("This is a TRACE level message");
  debug!("This is a DEBUG level message");
  info!("This is an INFO level message");
  warn!("This is a WARN level message");
  error!("This is an ERROR level message");

  println!("  Check console output for log messages at different levels");
  println!();

  //╔═══════════════════════════════════════════════════════════╗
  //║ Example 6: Span-Based Tracing                             ║
  //╚═══════════════════════════════════════════════════════════╝
  println!("🔍 Span-Based Tracing:");

  let workspace_span = info_span!(
    "workspace_operations",
    workspace = %env.workspace.metadata.name
  );

  let guard = workspace_span.enter();
  info!("Starting workspace operations");

  simulate_workspace_operations(env);

  drop(guard); // Exit span
  println!("  Workspace operations completed (check trace logs)");
  println!();

  //╔═══════════════════════════════════════════════════════════╗
  //║ Example 7: Error Tracing                                  ║
  //╚═══════════════════════════════════════════════════════════╝
  println!("⚠️  Error Tracing Example:");

  let result = simulate_potential_error();
  match result {
    Ok(value) => {
      info!(value = value, "Operation succeeded");
      println!("  Success: {value}");
    }
    Err(e) => {
      error!(error = %e, "Operation failed");
      println!("  Error: {e}");
    }
  }

  println!();

  //╔═══════════════════════════════════════════════════════════╗
  //║ Example 8: Performance Tracing                            ║
  //╚═══════════════════════════════════════════════════════════╝
  println!("⏱️  Performance Tracing:");

  let start_span = info_span!("performance_test");
  let _guard = start_span.enter();

  let start = Instant::now();
  simulate_work();
  let duration = start.elapsed();

  info!(duration_ms = duration.as_millis(), "Work completed");

  println!("  Duration: {duration:?}");
  println!();

  info!("Tracing example completed");
  println!("✅ Tracing example completed successfully!");
  println!("\n💡 Tracing Tips:");
  println!("   • Set RUST_LOG=trace for maximum verbosity");
  println!("   • Set RUST_LOG=prjenv=debug for library-specific logs");
  println!("   • Set RUST_LOG=info for production-level logging");
  println!("   • Use structured fields (key=value) for queryable logs");
  println!("\n📚 Log Levels (verbosity decreasing):");
  println!("   TRACE → DEBUG → INFO → WARN → ERROR");
}

/// Setup tracing subscriber for the example
fn setup_tracing() {
  let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("trace"));
  let layer = layer().with_target(true);
  let registry = registry().with(filter).with(layer);
  registry.init();
}

/// Simulate workspace operations for tracing demonstration
#[instrument(skip(env))]
fn simulate_workspace_operations(env: &Environment) {
  debug!("Simulating workspace operations");

  let operation_span = debug_span!("package_iteration");
  let _guard = operation_span.enter();

  for (i, package) in env.workspace.packages.iter().enumerate() {
    trace!(
      index = i,
      package = %package.metadata.name,
      version = %package.metadata.version,
      "Processing package"
    );
  }

  info!(
    package_count = env.workspace.packages.len(),
    "All packages processed"
  );
}

/// Simulate an operation that might fail - demonstrates error tracing
#[instrument]
fn simulate_potential_error() -> Result<String> {
  debug!("Attempting operation that might fail");

  // Simulate some condition that could fail
  let should_fail = false; // Change to true to see error tracing

  if should_fail {
    trace!("Operation failed - returning error");
    Err(IOError::new(
      IOErrorKind::Other,
      "Simulated error for tracing demonstration"
    ))
  } else {
    let value = "Operation successful";
    trace!(result = value, "Operation completed successfully");
    Ok(value.to_string())
  }
}

/// Simulate some work for performance tracing
#[instrument]
fn simulate_work() {
  trace!("Starting work simulation");

  sleep(Duration::from_millis(10));

  trace!("Work simulation completed");
}

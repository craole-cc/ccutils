//! # Global Allocator Configuration
//!
//! This crate supports multiple memory allocators, allowing the developer to choose between
//! the default Rust allocator (`std-alloc`) or the `mimalloc` allocator (`mi-alloc`) for performance benefits.
//!
//! ## Features
//!
//! - `mi-alloc`: Enables the `mimalloc` allocator, which is a high-performance memory allocator. This allocator
//!    is used globally across the entire program if this feature is enabled.
//! - `std-alloc`: Uses the default Rust allocator. This is the default feature if no other allocator is specified.
//!
//! ## Usage
//!
//! By default, no allocator is selected unless explicitly specified in the `Cargo.toml` file.
//! To enable `mimalloc`, you can add the following to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! my-crate = { version = "0.1", features = ["mi-alloc"] }
//! ```
//!
//! If you want to fall back to the standard allocator, use the `std-alloc` feature instead:
//!
//! ```toml
//! [dependencies]
//! my-crate = { version = "0.1", features = ["std-alloc"] }
//! ```
//!
//! ## Compile-time Enforcement
//!
//! If neither `mi-alloc` nor `std-alloc` is enabled, the crate will fail to compile with a helpful
//! error message indicating that one allocator must be selected.

#[cfg(feature = "mi-alloc")]
use mimalloc::MiMalloc;

#[cfg(feature = "mi-alloc")]
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

#[cfg(not(any(feature = "mi-alloc", feature = "std-alloc")))]
compile_error!(
	"Either 'mi-alloc' or 'std-alloc' feature must be enabled"
);

#![no_std]
#![allow(bad_style)]

//! Makes the OpenGL 3.3 Core API (+GL_KHR_debug) available for use.
//!
//! The crate's interface is provided as a "struct" style loader. Construct a
//! [`GlFns`] using an appropriate [`get_proc_address`] function, and then call
//! methods on your `GlFns`.
//!
//! There's also a "global" style loader if the `global_loader` feature is
//! enabled. This lets you load up functions pointers that can be freely
//! accessed from anywhere.
//!
//! ## Inlining
//!
//! This crate does **not** use the `#[inline]` attribute. If you want full
//! inlining just turn on Link-Time Optimization in your cargo profile:
//!
//! ```toml
//! [profile.release]
//! lto = "thin"
//! ```
//!
//! ## `trace_caller`
//!
//! If the `trace_caller` feature is enables then this attribute is placed on
//! any function that can panic. A panic will only happen if you call a function
//! that is not loaded.

use chlorine::*;

pub mod get_proc_address;

mod gl_command_types;
pub(crate) use gl_command_types::*;

pub mod gl_core_types;
pub use gl_core_types::*;

pub mod gl_enumerations;
pub use gl_enumerations::*;

pub mod gl_groups;
pub use gl_groups::*;

mod struct_loader;
pub use struct_loader::*;

#[cfg(feature = "global_loader")]
pub mod global_loader;

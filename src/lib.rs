#![no_std]
#![allow(bad_style)]

use chlorine::*;

mod gl_command_types;
pub(crate) use gl_command_types::*;

mod gl_core_types;
pub use gl_core_types::*;

mod gl_enumerations;
pub use gl_enumerations::*;

mod gl_groups;
pub use gl_groups::*;

mod struct_loader;
pub use struct_loader::*;

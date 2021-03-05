#![cfg_attr(not(feature = "println_debug"), no_std)]
#![allow(bad_style)]

//! Makes the OpenGL 3.3 Core API (+GL_KHR_debug) available for use.
//!
//! The crate's interface is provided as a "struct" style loader. Construct a
//! [`GlFns`] using an appropriate "gl_get_proc_address" function and then call
//! its methods.
//!
//! There's also a "global" style loader if the `global_loader` feature is
//! enabled.
//!
//! ## `gl_get_proc_address`
//!
//! GL must generally be dynamically loaded at runtime. This is done via a
//! function which we'll call "gl_get_proc_address". The expected operation of
//! this function is very simple: The caller passes in the pointer to a
//! null-terminated string with the name of a GL function, and
//! `gl_get_proc_address` returns the pointer to that function.
//!
//! The way that you get an appropriate `gl_get_proc_address` function is
//! platform dependent.
//! * With Win32 you'd use [wglGetProcAddress][wglGetProcAddress] for any
//!   function from *after* OpenGL 1.1, and [GetProcAddress][GetProcAddress] on
//!   an open `HMODULE` to "OpenGL.dll" for any OpenGL function from either 1.1
//!   or 1.0. That sounds silly, but it's true.
//! * With SDL2 you'd call [SDL_GL_GetProcAddress][SDL_GL_GetProcAddress], or
//!   the equivalent function within your SDL2 bindings.
//! * With [glutin][glutin] you'd call [Context::get_proc_address][glutin-gpa]
//!
//! [wglGetProcAddress]:
//! (https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-wglgetprocaddress)
//!
//! [GetProcAddress]:
//! (https://docs.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-getprocaddress)
//! [SDL_GL_GetProcAddress]: (https://wiki.libsdl.org/SDL_GL_GetProcAddress)
//!
//! [glutin]: (https://docs.rs/glutin/0.26.0/glutin)
//!
//! [glutin-gpa]:
//! (https://docs.rs/glutin/0.26.0/glutin/struct.Context.html#method.get_proc_address)
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

use chlorine::*;

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

#[cfg(feature = "println_debug")]
pub unsafe extern "system" fn println_debug_message_callback(source: GLenum, type_: GLenum, id: u32, severity: GLenum, length: i32, message: *const u8, _user_param: *const c_void) {
  let src = match source {
    GL_DEBUG_SOURCE_API => "API",
    GL_DEBUG_SOURCE_WINDOW_SYSTEM => "WindowSystem",
    GL_DEBUG_SOURCE_SHADER_COMPILER => "ShaderCompiler",
    GL_DEBUG_SOURCE_THIRD_PARTY => "3rdParty",
    GL_DEBUG_SOURCE_APPLICATION => "App",
    GL_DEBUG_SOURCE_OTHER => "Other",
    _ => "Unknown",
  };
  let ty = match type_ {
    GL_DEBUG_TYPE_ERROR => "Error",
    GL_DEBUG_TYPE_DEPRECATED_BEHAVIOR => "DeprecatedBehavior",
    GL_DEBUG_TYPE_UNDEFINED_BEHAVIOR => "UndefinedBehavior",
    GL_DEBUG_TYPE_PORTABILITY => "Portability",
    GL_DEBUG_TYPE_PERFORMANCE => "Performance",
    GL_DEBUG_TYPE_MARKER => "Marker",
    GL_DEBUG_TYPE_PUSH_GROUP => "PushGroup",
    GL_DEBUG_TYPE_POP_GROUP => "PopGroup",
    GL_DEBUG_TYPE_OTHER => "Other",
    _ => "Unknown",
  };
  let sev = match severity {
    GL_DEBUG_SEVERITY_HIGH => "High",
    GL_DEBUG_SEVERITY_MEDIUM => "Medium",
    GL_DEBUG_SEVERITY_LOW => "Low",
    GL_DEBUG_SEVERITY_NOTIFICATION => "Note",
    _ => "Unknown",
  };
  let msg = String::from_utf8_lossy(core::slice::from_raw_parts(message, length as usize));
  println!("GL>{id} [Src:{src}][Ty:{ty}][Severity:{sev}]> {msg}", id = id, src = src, ty = ty, sev = sev, msg = msg,);
}

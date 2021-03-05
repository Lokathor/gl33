//! ## `gl_get_proc_address`
//!
//! GL must generally be dynamically loaded at runtime. This is done via a
//! callback function that can look up the function pointer for a given function
//! name. The caller passes in the pointer to a null-terminated string with the
//! name of a GL function, and the callback returns the pointer to that
//! function. This is generally called something like "gl get proc address".
//!
//! The way that you get an appropriate `gl_get_proc_address` function is
//! platform dependent. Here's some examples:
//!
//! ### SDL2
//!
//! With SDL2, you should use the
//! [SDL_GL_GetProcAddress](https://wiki.libsdl.org/SDL_GL_GetProcAddress)
//! function
//! ([fermium](https://docs.rs/fermium/20014.4.1/fermium/video/fn.SDL_GL_GetProcAddress.html)
//! crate docs,
//! [sdl2](https://docs.rs/sdl2/0.34.3/sdl2/struct.VideoSubsystem.html#method.gl_get_proc_address)
//! crate docs).
//!
//! ```ignore
//! use gl33::*;
//! use fermium::prelude::*;
//!
//! todo!("create the GL context.");
//! let gl = unsafe { GlFns::load_from(&|p| SDL_GL_GetProcAddress(p) as _).unwrap() };
//! ```
//!
//! ### Glutin
//!
//! Glutin has a
//! [get_proc_address](https://docs.rs/glutin/0.26.0/glutin/struct.Context.html#method.get_proc_address)
//! method on the `Context` type.
//! ```ignore
//! use gl33::*;
//!
//! let ctx = todo!("get your GL context created by glutin");
//!
//! let gl = unsafe {
//!   GlFns::load_from(&|p| {
//!     let c_str = std::ffi::CStr::from_ptr(c);
//!     let rust_str = c_str.to_str().unwrap();
//!     ctx.get_proc_address(rust_str) as _
//!   })
//!   .unwrap()
//! };
//! ```
//!
//! ### Direct WinApi Calls
//!
//! If you're using [winapi](https://docs.rs) directly, you'll have to use
//! `wglGetProcAddress` for any function from OpenGL 1.2 or later, and then
//! `GetProcAddress` on a handle to "opengl32.dll" for any function that's from
//! OpenGL 1.0 or 1.1.
//! ```ignore
//! use gl33::*;
//!
//! use winapi::um::{
//!   libloaderapi::{GetProcAddress, LoadLibraryA},
//!   wingdi::wglGetProcAddress,
//! };
//!
//! todo!("first make your GL context and make it current");
//!
//! let ogl32_lib = unsafe { LoadLibraryA(b"opengl32.dll\0") };
//! assert!(!ogl32_lib.is_null());
//! let gl = unsafe {
//!   GlFns::load_from(&|p| match wglGetProcAddress as usize {
//!     // some non-zero values are still error values
//!     0 | 1 | 2 | 3 | usize::MAX => GetProcAddress(ogl32_lib, name_ptr.cast()),
//!     otherwise => otherwise as _,
//!   })
//!   .unwrap()
//! };
//! ```

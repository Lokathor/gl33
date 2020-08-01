#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[cfg(not(any(feature = "global_loader", feature = "struct_loader")))]
compile_error!(
  "Please enable either the 'global_loader' or 'static_loader' feature."
);

#[cfg(all(feature = "global_loader", feature = "struct_loader"))]
compile_error!("Please enable only one loader, not both.");

use fermium::*;
use gl33::*;

fn main() {
  unsafe {
    SDL_Init(SDL_INIT_VIDEO);
    // 0 for success, negative for error
    assert_eq!(0, SDL_GL_SetAttribute(SDL_GL_CONTEXT_MAJOR_VERSION, 3));
    assert_eq!(0, SDL_GL_SetAttribute(SDL_GL_CONTEXT_MINOR_VERSION, 3));
    assert_eq!(
      0,
      SDL_GL_SetAttribute(
        SDL_GL_CONTEXT_PROFILE_MASK,
        SDL_GL_CONTEXT_PROFILE_CORE as _
      )
    );
    // make window
    let win = SDL_CreateWindow(
      b"gl33 fermium demo\0".as_ptr().cast(),
      50,
      50,
      800,
      600,
      (SDL_WINDOW_SHOWN | SDL_WINDOW_OPENGL) as _,
    );
    if win.is_null() {
      let mut p = SDL_GetError();
      while *p != 0 {
        print!("{}", *p as u8 as char);
        p = p.add(1);
      }
      println!();
      panic!();
    }
    // make context the window will use
    let ctx = SDL_GL_CreateContext(win);
    if ctx.is_null() {
      let mut p = SDL_GetError();
      while *p != 0 {
        print!("{}", *p as u8 as char);
        p = p.add(1);
      }
      println!();
      panic!();
    }
    //
    #[cfg(feature = "global_loader")]
    {
      load_global_gl_with(|c_char_ptr| SDL_GL_GetProcAddress(c_char_ptr));
      assert!(glClearColor_is_loaded());
      assert!(glClear_is_loaded());
      glClearColor(0.2, 0.3, 0.3, 1.0);
      let mut event: SDL_Event = core::mem::zeroed();
      loop {
        if SDL_PollEvent(&mut event) != 0 && event.common.type_ == SDL_QUIT as _
        {
          break;
        } else {
          glClear(GL_COLOR_BUFFER_BIT);
          SDL_GL_SwapWindow(win);
        }
      }
    }
    //
    #[cfg(feature = "struct_loader")]
    {
      let gl = GlFns::load_with(|c_char_ptr| SDL_GL_GetProcAddress(c_char_ptr));
      assert!(gl.ClearColor_is_loaded());
      assert!(gl.Clear_is_loaded());
      gl.ClearColor(0.2, 0.3, 0.3, 1.0);
      let mut event: SDL_Event = core::mem::zeroed();
      loop {
        if SDL_PollEvent(&mut event) != 0 && event.common.type_ == SDL_QUIT as _
        {
          break;
        } else {
          gl.Clear(GL_COLOR_BUFFER_BIT);
          SDL_GL_SwapWindow(win);
        }
      }
    }
    //
    SDL_DestroyWindow(win);
    SDL_Quit();
  }
}

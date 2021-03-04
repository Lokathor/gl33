#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use fermium::{error::*, events::*, video::*, *};
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
        SDL_GL_CONTEXT_PROFILE_CORE.0 as _
      )
    );
    // make window
    let win = SDL_CreateWindow(
      b"gl33 fermium demo\0".as_ptr().cast(),
      50,
      50,
      800,
      600,
      (SDL_WINDOW_SHOWN | SDL_WINDOW_OPENGL).0 as _,
    );
    if win.is_null() {
      let mut v = Vec::with_capacity(4096);
      let mut p = SDL_GetErrorMsg(v.as_mut_ptr(), v.capacity() as _);
      while *p != 0 {
        print!("{}", *p as u8 as char);
        p = p.add(1);
      }
      println!();
      panic!();
    }
    // make context the window will use
    let ctx = SDL_GL_CreateContext(win);
    if ctx.0.is_null() {
      let mut v = Vec::with_capacity(4096);
      let mut p = SDL_GetErrorMsg(v.as_mut_ptr(), v.capacity() as _);
      while *p != 0 {
        print!("{}", *p as u8 as char);
        p = p.add(1);
      }
      println!();
      panic!();
    }
    //
    let gl =
      GlFns::load_from(&|c_char_ptr| SDL_GL_GetProcAddress(c_char_ptr.cast()))
        .unwrap();
    gl.ClearColor(0.2, 0.3, 0.3, 1.0);
    let mut event: SDL_Event = core::mem::zeroed();
    loop {
      if SDL_PollEvent(&mut event) != 0 && event.common.type_ == SDL_QUIT as _ {
        break;
      } else {
        gl.Clear(GL_COLOR_BUFFER_BIT);
        SDL_GL_SwapWindow(win);
      }
    }
    //
    SDL_DestroyWindow(win);
    SDL_Quit();
  }
}

cargo install phosphorus
phosphorus ../gl.xml gl 3 3 core GL_ARB_debug_output,GL_KHR_debug,GL_ARB_texture_filter_anisotropic >src/lib.rs
cargo fmt

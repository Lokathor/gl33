cargo install phosphorus
mkdir target
phosphorus ../gl.xml gl 3 3 core GL_ARB_debug_output,GL_KHR_debug >src/lib.rs
cargo fmt

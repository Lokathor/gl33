cargo install phosphorus
mkdir target
phosphorus ../gl.xml gl 3 3 core >target/lib.rs
rustfmt target/lib.rs

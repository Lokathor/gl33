[![License:Zlib](https://img.shields.io/badge/License-Zlib-brightgreen.svg)](https://opensource.org/licenses/Zlib)
![min-rust-1.34](https://img.shields.io/badge/Min%20Rust-1.34-green.svg)
[![crates.io](https://img.shields.io/crates/v/gl33.svg)](https://crates.io/crates/gl33)
[![docs.rs](https://docs.rs/gl33/badge.svg)](https://docs.rs/gl33/)

![Unsafe-101-Percent](https://img.shields.io/badge/Unsafety-101%25-red.svg)

# gl33
Bindings to OpenGL 3.3

## Stability

The `gl33` crate presents OpenGL 3.3 bindings for Rust, described by [gl.xml](https://github.com/KhronosGroup/OpenGL-Registry/blob/master/xml/gl.xml).

As often as `gl.xml` updates I will also attempt to issue updates for this crate.

Because `gl33` follows the current content of `gl.xml` as closely as possible,
it's *possible* (though highly unlikely) that there could be a `gl.xml` update
that would somehow cause a breaking change. This is most likely to occur if an
argument's type changes between signed and unsigned, which is not a big
difference in C but it would cause a type mismatch in Rust (you'd need to add an
`as _` to make it cast the value).

# Rust, C, Fortran Sega Dreamcast Interop Demo

This is a basic demo I made showing interop between Rust, C, and Fortran on the Sega Dreamcast in the form of a 3D textured rotating cube,
based on the classic nehe06 demo. The Rust code drives the main logic and is compiled using the Rust-GCC compiler "gccrs" since the
standard LLVM-based Rust toolchain does not support the SuperH architecture in the Dreamcast. Due to its early and experimental nature,
only a fraction of the Rust language is available, and pointer arithmetic functions are not yet working, so Fortran was used for this as a
fun twist. The GLdc/KallistiOS C libraries are called, but no other C is used in the demo.

[![Watch the video](https://img.youtube.com/vi/VUiRoEcpvtI/maxresdefault.jpg)](https://youtu.be/VUiRoEcpvtI)

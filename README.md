[![WolkenWelten CI](https://github.com/wolkenwelten/wolkenwelten/actions/workflows/ci.yml/badge.svg?branch=master)](https://github.com/wolkenwelten/wolkenwelten/actions/workflows/ci.yml)

# WolkenWelten
This is the rust port of the WolkenWelten codebase, everything is still super early so prepare for pretty much nothing
working whatsoever.

Also keep in mind that this is the first Rust I've ever written, so if you spot some ugly bits of code
be sure to open a PR so as to improve the quality of the codebase :)

# How to build/run it
You need a rust toolchain installed, preferrably the latest stable one using `rustup`,
then just checking out this repository and running `cargo run --release` in it should get you going.

# Platform support
I am regularly testing it on X86_64 FreeBSD/Arch Linux/MacOS/Win10 with Intel/AMD/Nvidia GPUs (no nouveau though) and on a RaspberryPI 4 running Raspbian 64-bit.

Web/WASM is intentionally NOT a supported platform for the foreseeable future, since WebGL has been quite the pain to support in the past.

# Changes from the C version (these are 99% certain to happen if not checked already)
- [X] No more WASM/Emscripten build (really liked that, but it was A LOT of work and broke all the time, so in the beginning WW will only support Lin/Mac/Win)
- [X] No SDL2 (it worked quite well, but it simplifies the build process to have as much as possible be written in Rust)
- [X] Bigger world Size (32-bit per axis, instead of 16, allowing for ~4 Billion Blocks per Axis as compared to ~65 thousand before)
- [ ] OpenAL for sound output
- [ ] Single executable for client/server (the client should be a feature that can be disabled though)
- [ ] Meshes are voxel based, just like the world
- [ ] Include V8 as a scripting runtime instead of Nujel

# Current ToDo's
- [X] Chunk fade
- [X] "Infinite" world
- [X] Simple placeholder worldgen
- [X] Remove hidden surfaces from BlockMeshes
- [X] Simple player controls (gravity/collision with the world)
- [X] Chunk/BlockMesh GC
- [X] Unit Tests
- [X] Frustum culling (port from WW)
- [X] CI running tests/fmt/clippy
- [X] Indexed BlockMeshes
- [X] Nicer player movement
- [X] Simple voxel side occlusion-culling (port from WW)
- [X] Proper chunk draw ordering, back to front due to fade-in (port from WW)
- [X] Chunk fade-in after generation
- [X] Sky sphere (port from WW)
- [ ] Greedy meshing (port from WW)
- [ ] Lighting (port from WW, without ASM/SIMD (for now))
- [ ] Frame-rate independent physics/gameplay (maybe try out rapier3d?)
- [ ] Block manipulation (simple removal/placement as well as block selection)
- [ ] Block highlight (port from WW)
- [ ] Import Models made with Goxel
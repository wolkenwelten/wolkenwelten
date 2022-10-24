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
- [X] Frame-rate independent physics/gameplay
- [ ] Greedy meshing (port from WW)
- [ ] Lighting (port from WW, without ASM/SIMD (for now))
- [ ] Block manipulation (simple removal/placement as well as block selection)
- [ ] Block highlight (port from WW)
- [ ] Import Models made with Goxel
- [ ] Make entities use voxel meshes

- [ ] Embed deno_core and make sure the event loop runs regularly
- [ ] (If simple) allow for TypeScript, but only if it's simple to do since it's not strictly necessary, but would be nice to have from the beginning
- [ ] Add `world.getBlock(x, y, z)` and `world.setBlock(x, y, z, b)`
- [ ] Make worldgen call into deno and receive an entire chunk of data
- [ ] Allow entities to be created from within deno for example `world.createEntity()`
- [ ] Allow entities to be enumerated from within deno and add `entity.getPos(), entity.setPos(x, y, z), entity.hide()` methods
- [ ] Add support for `entity.onCollide` 
- [ ] Allow for GUI output, text and images from the atlas would suffice for the beginning (should allow for a Healthbar and super simple non interactive Inventory)
- [ ] Add support for `game.onKey{Up,Down}` | `game.onMouse{Up,Down}` higher-level abstractions can then be built within JS/TS

- [ ] Super simple multiplayer (connecting with login/logout messages only for now)
- [ ] Synchronize world mutations
- [ ] Synchronize character movement (and render characters)
- [ ] Synchronize entities
- [ ] Allow for a simple JSON-RPC inspired interface for client<->server communication
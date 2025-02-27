# 🎳 physx-rs

[![Build Status](https://travis-ci.com/EmbarkStudios/physx-rs.svg?branch=master)](https://travis-ci.com/EmbarkStudios/physx-rs)
[![Contributor Covenant](https://img.shields.io/badge/contributor%20covenant-v1.4%20adopted-ff69b4.svg)](CODE_OF_CONDUCT.md)
[![Embark](https://img.shields.io/badge/embark-open%20source-blueviolet.svg)](http://embark.games)
[![FOSSA Status](https://app.fossa.io/api/projects/git%2Bgithub.com%2FEmbarkStudios%2Fphysx-rs.svg?type=shield)](https://app.fossa.io/projects/git%2Bgithub.com%2FEmbarkStudios%2Fphysx-rs?ref=badge_shield)

Rust binding and wrapper over [NVIDIA PhysX](https://github.com/NVIDIAGameWorks/PhysX), a popular and mature physics engine particularly well-suited for games.

Created and maintained by [Embark](http://embark.games) and _**not**_ officially supported by NVIDIA.

This repository contains 3 crates:

| Name | Description | Links |
| --- | --- | --- |
| [`physx`](physx/) | High-level interface on top of `physx-sys` 🚧 | [![Crates.io](https://img.shields.io/crates/v/physx.svg)](https://crates.io/crates/physx) [![Docs](https://docs.rs/physx/badge.svg)](https://docs.rs/physx) |
| [`physx-sys`](physx-sys/) | Unsafe bindings to the [PhysX C++ API](https://github.com/NVIDIAGameWorks/PhysX) | [![Crates.io](https://img.shields.io/crates/v/physx-sys.svg)](https://crates.io/crates/physx-sys) [![Docs](https://docs.rs/physx-sys/badge.svg)](https://docs.rs/physx-sys) |
| [`physx-macros`](physx-macros/) | Utility macros used internally by the `physx` crate | [![Crates.io](https://img.shields.io/crates/v/physx-macros.svg)](https://crates.io/crates/physx-macros) [![Docs](https://docs.rs/physx-macros/badge.svg)](https://docs.rs/physx-macros) |

## Why use it?

* You want a feature-rich and performant physics engine to use in your project.

## Caveats

* The high-level `physx` wrapper is work-in-progress, and only covers a part of PhysX functionality. You can follow our progress and see where contributions are needed in our [*Tracking Issue for High-Level API Completeness*](https://github.com/EmbarkStudios/physx-rs/issues/5).

* Any other features have to be accessed through the unsafe [physx-sys](physx-sys/) crate.

* It's a large C++ codebase which requires a C++ toolchain, and comes with a non-trivial build system.

### Alternatives

* [nphysics](https://github.com/rustsim/nphysics): a 2- and 3-dimensional physics engine for games and animations written in Rust. It is a good option for projects which do not require the full feature set of PhysX or prefer a native Rust solution.

## Usage

The following code example shows how [`physx`](physx/) can be initialized.

``` Rust
const PX_PHYSICS_VERSION: u32 = physx::version(4, 1, 0);
let mut foundation = Foundation::new(PX_PHYSICS_VERSION);

let mut physics = PhysicsBuilder::default()
    .load_extensions(false) // switch this flag to load extensions during setup
    .build(&mut foundation);

let mut scene = physics.create_scene(
    SceneBuilder::default()
        .set_gravity(glm::vec3(0.0, -9.81, 0.0))
        .set_simulation_threading(SimulationThreadType::Dedicated(1)),
);

```

You can run an example with `cargo run --example ball`, which should show the following output:

![Example](images/example-ball.png)

Information about all wrapper functionality can be found in the [physx](physx/) crate docs.

If you require functionality not covered by the [physx](physx/) wrapper you can use the low level [physx-sys](physx-sys) crate, which closely maps to the official PhysX SDK. You can find the PhysX user guide [here](https://gameworksdocs.nvidia.com/PhysX/4.1/documentation/physxguide/Manual/Introduction.html).

## Prerequisites

* C++ compiler ([see the `cc` crate](https://crates.io/crates/cc) for requirements)
* CMake ([see the `cmake` crate](https://crates.io/crates/cmake) for requirements)

## Contributing

We welcome community contributions to this project.

Please read our [Contributor Guide](CONTRIBUTING.md) for more information on how to get started.

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

Note that the [PhysX C++ SDK](https://github.com/NVIDIAGameWorks/PhysX) has its [own BSD 3 license](https://gameworksdocs.nvidia.com/PhysX/4.1/documentation/physxguide/Manual/License.html) and depends on [additional C++ third party libraries](https://github.com/NVIDIAGameWorks/PhysX/tree/4.1/externals).


[![FOSSA Status](https://app.fossa.io/api/projects/git%2Bgithub.com%2FEmbarkStudios%2Fphysx-rs.svg?type=large)](https://app.fossa.io/projects/git%2Bgithub.com%2FEmbarkStudios%2Fphysx-rs?ref=badge_large)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
# Snapback engine
A small game engine written in Rust, backed by SDL2.

It is an exploration into [Entity Component Systems](https://en.wikipedia.org/wiki/Entity_component_system) and is not intended for proper usage. It is implemented as a dense array of entity ids with various flags available (like mass and transform, for physics, or sprite for rendering.)
You can see the entry point into the engine in `world.rs` which sets up several systems, such as masses, transforms, and movements.
Components for entities are implemented via the `compenent!` macro in `src/core/component.rs` which sets up helpful functions.

Game update loops are decoupled from rendering and handled by separate systems.

[![Build Status](https://travis-ci.org/Winwardo/snapback-game-engine.svg?branch=master)](https://travis-ci.org/Winwardo/snapback-game-engine)
[![Coverage Status](https://coveralls.io/repos/Winwardo/snapback-game-engine/badge.svg?branch=master&service=github)](https://coveralls.io/github/Winwardo/snapback-game-engine?branch=master)

## Running
Clone the git repo, and place a copy of SDL2.dll in the top level directory. Then `cargo run` will build and run the engine with some sample data.

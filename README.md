# hematite_server [![Build Status](https://travis-ci.org/PistonDevelopers/hematite_server.svg?branch=master)](https://travis-ci.org/PistonDevelopers/hematite_server)

This is the [Hematite][] server, an implementation of the [Minecraft][] server in [Rust][]. Some of our [goals](https://github.com/PistonDevelopers/hematite_server/issues/3) are:

*   feature parity with the “Notchian” vanilla server,
*   increased performance,
*   and support for server mods written in Rust.

The Hematite server is currently in very early stages of development and completely unusable.

## Goalish Things
* [ ] Split player connection out of world.rs
* [ ] Load player state from disk
* [ ] Save player state to disk
* [ ] Command framework (using clap)
* [ ] Item give command
* [ ] Loading in any amount of world
* [ ] Loading in world dynamically as needed

Farther off things:

* [ ] Ticks
* [ ] Tab completion
* [ ] Multiplayer

## Dependencies

![dependencies](./Cargo.png)

[Hematite]: http://hematite.piston.rs/ (Hematite)
[Minecraft]: https://minecraft.net/ (Minecraft)
[Rust]: http://www.rust-lang.org/ (The Rust Programming Language)

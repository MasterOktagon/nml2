# NML2

since [NMLs](https://github.com/OpenTTD/nml) dependency PLY was moved to end-of-life, I decided to rewrite it in rust using LALRPOP.
This is very crude and unfinished right now.

## Building

The NML2 compiler can and should be built using the rust package framework `cargo`

1. [Install rust and cargo](https://rust-lang.org/tools/install/)
2. run `$ cargo build`. The executable `nmlc` should be in the `target/debug` folder

### Semantic Versioning

this project adheres to semantic versioning with one major difference: NML 1.0 was written in python, so release version numbers will begin with 2.0.0
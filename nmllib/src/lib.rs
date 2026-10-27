//! NMLLib is the rust library that contains all the actual code that does the work of compiling NML into NFO.
//! I decided to split this into an extra subcrate since it allows developers who want to use their own build stack
//! or preprocessing to automate things to simply include this in their project and use the library without
//! needing to interfere with the CLI
//! 
//! Documentation of this crate contains the internals of NML2 and does not contain instructions on how to use NML.

pub mod ast;
pub mod parser;
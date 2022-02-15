//! # math vector
//!
//! A simple and convenient 3D vector library without excessive use of external
//! dependencies. If other vector crates are swiss-army knives, math_vector is a
//! spoon; safe, intuitive, and convenient. As an added bonus, you won't run
//! into any excursions with the law using this library thanks to the awfully
//! permissive Unlicense.
//!
//! The only type in this crate is `Vector`, which is highly generic;
//! shifting functionality depending upon the traits implemented by its internal
//! components' types.

pub mod ops;

pub mod protocol;
pub mod vector;

pub use vector::Vector;

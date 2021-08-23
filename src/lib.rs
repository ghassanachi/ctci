//! # \[Ongoing\] Rust CTCI Problem
//!
//! Ongoing project to both learn rust and practice some interview questions using the amazing
//! `Cracking The Coding Interview` book
//!
//! ## Notes
//!  - I've used [SinglyLinkedList][structures::SinglyLinkedList] and
//!  [DoubleLinkedList][structures::LinkedList] implementations grabbed
//!  from [modulitos/CtCI-rust][`ll-url`] since the [`std::collections::LinkedList`]'s iterators
//!  iterate on inner type which goes against the intended solutions.
//!  - I've made some tweaks to the above implementations with some helper methods as well as
//!  loosening the Bounds restrictions on the inner type.
//!
//! [`ll-url`]: https://github.com/modulitos/CtCI-rust
pub mod problems;
pub mod structures;

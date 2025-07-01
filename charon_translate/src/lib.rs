//! Charon translation library - rustc integration
//!
//! This library contains the rustc-dependent translation logic that converts
//! Rust MIR into Charon's AST format. It requires rustc_private feature.

#![feature(rustc_private)]
#![recursion_limit = "256"]
#![feature(assert_matches)]
#![feature(box_patterns)]
#![feature(deref_pure_trait)]
#![feature(if_let_guard)]
#![feature(impl_trait_in_assoc_type)]
#![feature(iterator_try_collect)]
#![feature(iter_array_chunks)]
#![feature(let_chains)]
#![feature(trait_alias)]
#![expect(incomplete_features)]
#![feature(deref_patterns)]

extern crate rustc_abi;
extern crate rustc_ast;
extern crate rustc_ast_pretty;
extern crate rustc_driver;
extern crate rustc_error_messages;
extern crate rustc_errors;
extern crate rustc_hir;
extern crate rustc_index;
extern crate rustc_interface;
extern crate rustc_middle;
extern crate rustc_session;
extern crate rustc_span;
extern crate rustc_target;

// Re-export charon_lib for convenience
pub use charon_lib;

#[macro_use]
pub mod translate;

// Re-export translate module contents
pub use translate::*;
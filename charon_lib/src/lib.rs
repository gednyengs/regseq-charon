//! Core Charon library containing AST definitions and utilities
//! 
//! This library provides the core data structures for the Charon project,
//! including the AST definitions for ULLBC and LLBC, without any rustc
//! dependencies. This allows it to be used in contexts where rustc_private
//! is not available.

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
// For when we use charon on itself :3
#![feature(register_tool)]
#![register_tool(charon)]

#[macro_use]
pub mod ids;
#[macro_use]
pub mod logger;
pub mod ast;
pub mod common;
pub mod errors;
pub mod export;
pub mod name_matcher;
pub mod options;
pub mod pretty;
pub mod transform;

// Re-export all the ast modules so we can keep the old import structure.
pub use ast::{builtins, expressions, gast, llbc_ast, meta, names, types, ullbc_ast, values};
pub use pretty::formatter;
pub use transform::{graphs, reorder_decls, ullbc_to_llbc};

/// The version of the crate, as defined in `Cargo.toml`.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Read a `.llbc` file.
pub fn deserialize_llbc(path: &std::path::Path) -> anyhow::Result<ast::TranslatedCrate> {
    use crate::export::CrateData;
    use anyhow::Context;
    use serde::Deserialize;
    use std::fs::File;
    use std::io::BufReader;
    let file = File::open(&path)
        .with_context(|| format!("Failed to read llbc file {}", path.display()))?;
    let reader = BufReader::new(file);
    let mut deserializer = serde_json::Deserializer::from_reader(reader);
    // Deserialize without recursion limit.
    deserializer.disable_recursion_limit();
    // Grow stack space as needed.
    let deserializer = serde_stacker::Deserializer::new(&mut deserializer);
    Ok(CrateData::deserialize(deserializer)?.translated)
}
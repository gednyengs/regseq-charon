# regseq-charon

A Rust compiler frontend that translates MIR (Mid-level Intermediate Representation) to LLBC (Low-Level Borrow Checked), adapted from [AeneasVerif's Charon](https://github.com/AeneasVerif/charon) for the regseq compiler infrastructure.

## Overview

regseq-charon provides a clean, maintainable intermediate representation for Rust programs that preserves ownership and borrowing information while being easier to analyze than raw MIR. It serves as the foundation for program analysis and verification tools in the regseq ecosystem.

## Architecture

The project is organized as a Cargo workspace with two main crates:

### `charon_lib`
Core library containing AST definitions and utilities:
- **No rustc dependencies** - Can be used without `#![feature(rustc_private)]`
- AST definitions for ULLBC (Unstructured LLBC) and LLBC
- Pretty printing and formatting utilities
- Transformation passes and optimizations
- Name resolution and pattern matching
- Error handling infrastructure

### `charon_translate`
Translation layer that interfaces with the Rust compiler:
- **Requires `#![feature(rustc_private)]`** - Direct rustc integration
- MIR to ULLBC translation
- Type translation and generic handling
- Trait resolution and method translation
- Closure desugaring
- Integration with rustc's query system

## Key Features

- **Preserves Rust semantics**: Maintains ownership, borrowing, and lifetime information
- **Multiple IR levels**: Both unstructured (ULLBC) and structured (LLBC) representations
- **Extensible passes**: Modular transformation pipeline for analysis and optimization
- **Rich metadata**: Preserves source information, attributes, and documentation
- **Monomorphization support**: Can generate monomorphized or generic code
- **Configurable extraction**: Fine-grained control over what gets translated

## Usage

### As a Library

For crates that only need the AST definitions:
```toml
[dependencies]
charon_lib = { path = "path/to/regseq-charon/charon_lib" }
```

For crates that need translation capabilities:
```toml
[dependencies]
charon_translate = { path = "path/to/regseq-charon/charon_translate" }
```

Note: Dependencies on `charon_translate` will require `#![feature(rustc_private)]` in your crate.

### Building

```bash
cargo build
```

### Running Tests

```bash
cargo test
```

## Translation Pipeline

1. **MIR Extraction**: Retrieve MIR from rustc for the target crate
2. **ULLBC Translation**: Convert MIR to unstructured LLBC representation
3. **Transformation Passes**: Apply various cleanup and optimization passes
4. **Control Flow Reconstruction**: Build structured LLBC from ULLBC
5. **Serialization**: Output the final LLBC representation

## Configuration Options

Key options available through `CliOpts`:

- `--mir <level>`: MIR extraction level (built, promoted, elaborated, optimized)
- `--monomorphize`: Generate monomorphized code
- `--hide-marker-traits`: Hide Sized, Send, Sync, Unpin traits
- `--include/--exclude`: Fine-grained filtering of items to translate
- `--no-serialize`: Skip serialization (useful for analysis tools)
- Various debugging outputs (`--print-ullbc`, `--print-llbc`, etc.)

## Integration with regseq

regseq-charon provides the foundational IR for the regseq compiler's analysis framework. Analysis passes in regseq-analysis consume the LLBC representation to perform various program analyses while avoiding direct dependency on rustc internals where possible.

## Differences from Upstream Charon

This fork adapts Charon for use in the regseq compiler with several modifications:

- Workspace structure separating rustc-dependent and independent code
- Integration points for regseq's analysis framework
- Adapted for regseq's specific use cases and requirements

### Upstream Version

This adaptation is based on Charon commit [`309ffd88a1e7948f480aef9a7524036b5e636b80`](https://github.com/AeneasVerif/charon/commit/309ffd88a1e7948f480aef9a7524036b5e636b80) from the main branch.

## License

Licensed under Apache-2.0, consistent with the upstream Charon project.

## Acknowledgments

This project is adapted from [Charon](https://github.com/AeneasVerif/charon) by the Aeneas verification project team. We gratefully acknowledge their foundational work in creating a clean intermediate representation for Rust.
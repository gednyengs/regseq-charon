pub mod get_mir;
pub mod resolve_path;
pub mod translate_bodies;
pub mod translate_closures;
pub mod translate_constants;
pub mod translate_crate;
pub mod translate_ctx;
pub mod translate_functions;
pub mod translate_generics;
pub mod translate_items;
pub mod translate_meta;
pub mod translate_predicates;
pub mod translate_types;

// charon_lib is now an external dependency

use charon_lib::{
    options::CliOpts,
    transform::{
        Pass, PrintCtxPass, FINAL_CLEANUP_PASSES, INITIAL_CLEANUP_PASSES, LLBC_PASSES,
        SHARED_FINALIZING_PASSES, ULLBC_PASSES,
    },
};
use std::fmt;

/// Custom `DefId` debug routine that doesn't print unstable values like ids and hashes.
pub fn def_id_debug(def_id: rustc_hir::def_id::DefId, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    rustc_middle::ty::tls::with_opt(|opt_tcx| {
        if let Some(tcx) = opt_tcx {
            let crate_name = if def_id.is_local() {
                tcx.crate_name(rustc_hir::def_id::LOCAL_CRATE)
            } else {
                tcx.cstore_untracked().crate_name(def_id.krate)
            };
            write!(
                f,
                "{}{}",
                crate_name,
                tcx.def_path(def_id).to_string_no_crate_verbose()
            )?;
        } else {
            write!(f, "<can't access `tcx` to print `DefId` path>")?;
        }
        Ok(())
    })
}

/// Calculate the list of passes we will run on the crate before outputting it.
pub fn transformation_passes(options: &CliOpts) -> Vec<Pass> {
    let mut passes: Vec<Pass> = vec![];

    passes.push(Pass::NonBody(PrintCtxPass::new(
        options.print_original_ullbc,
        format!("# ULLBC after translation from MIR"),
    )));

    passes.extend(INITIAL_CLEANUP_PASSES);
    passes.extend(ULLBC_PASSES);

    if !options.ullbc {
        // If we're reconstructing control-flow, print the ullbc here.
        passes.push(Pass::NonBody(PrintCtxPass::new(
            options.print_ullbc,
            format!("# Final ULLBC before control-flow reconstruction"),
        )));
    }

    if !options.ullbc {
        passes.extend(LLBC_PASSES);
    }
    passes.extend(SHARED_FINALIZING_PASSES);

    if options.ullbc {
        // If we're not reconstructing control-flow, print the ullbc after finalizing passes.
        passes.push(Pass::NonBody(PrintCtxPass::new(
            options.print_ullbc,
            format!("# Final ULLBC before serialization"),
        )));
    } else {
        passes.push(Pass::NonBody(PrintCtxPass::new(
            options.print_llbc,
            format!("# Final LLBC before serialization"),
        )));
    }

    // Run the final passes after pretty-printing so that we get some output even if check_generics
    // fails.
    passes.extend(FINAL_CLEANUP_PASSES);
    passes
}

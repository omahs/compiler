use miden_codegen_masm as masm;
use miden_frontend_wasm as wasm;
use miden_hir::{
    self as hir,
    parser::ast,
    pass::{AnalysisManager, ConversionPass, RewritePass},
};
use midenc_session::Session;

use super::Stage;
use crate::{CompilerError, CompilerResult};

mod codegen;
mod link;
mod parse;
mod rewrite;
mod sema;

pub use self::{
    codegen::{CodegenStage, Compiled},
    link::{LinkerStage, MaybeLinked},
    parse::{ParseOutput, ParseStage},
    rewrite::ApplyRewritesStage,
    sema::SemanticAnalysisStage,
};

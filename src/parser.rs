//! A wrapper module that loads the generated parser

use lalrpop_util::lalrpop_mod;

lalrpop_mod!(
    #[allow(unused_imports)]
    pub(super) parser
);

pub use parser::*;

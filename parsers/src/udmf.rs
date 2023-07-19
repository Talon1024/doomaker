/// The parser for UDMF-format Doom maps
pub mod parser {
    use pest_derive::Parser;
    #[derive(Debug, Parser)]
    #[grammar = "pest-grammars/common.pest"]
    #[grammar = "pest-grammars/udmf.pest"]
    pub struct UDMFParser;
}

/// Input types, used to parse the tokens into UDMF data
pub mod input;

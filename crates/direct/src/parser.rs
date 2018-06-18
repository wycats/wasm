use crate::{ast, ModuleParser, Token};

pub type ParseError<'input> = lalrpop_util::ParseError<usize, Token<'input>, &'static str>;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ErrorLocation {
    EOF,
    Byte(usize),
}

pub fn parse(source: &'input str) -> Result<ast::Module<'input>, ParseError<'input>> {
    ModuleParser::new().parse(source)
}

pub fn location(error: ParseError) -> ErrorLocation {
    match error {
        lalrpop_util::ParseError::InvalidToken { location } => ErrorLocation::Byte(location),
        lalrpop_util::ParseError::UnrecognizedToken { token: None, .. } => ErrorLocation::EOF,
        lalrpop_util::ParseError::UnrecognizedToken {
            token: Some((location, ..)),
            ..
        } => ErrorLocation::Byte(location),
        lalrpop_util::ParseError::ExtraToken {
            token: (location, ..),
            ..
        } => ErrorLocation::Byte(location),
        lalrpop_util::ParseError::User { error } => panic!("{:?}", error),
    }
}
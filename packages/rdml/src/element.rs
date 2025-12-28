use syn::{
    Path, Result,
    parse::{Parse, ParseStream},
    token::Paren,
};

use crate::{Attributes, Block};

#[derive(Debug, PartialEq, Hash)]
pub struct Element {
    pub path: Path,
    pub attributes: Option<Attributes>,
    pub children: Block,
}

impl Parse for Element {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Element {
            path: input.parse()?,
            attributes: input.peek(Paren).then(|| input.parse()).transpose()?,
            children: input.parse()?,
        })
    }
}

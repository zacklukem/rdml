use syn::{
    Path, Result,
    parse::{Parse, ParseStream},
    token::Paren,
};

use crate::{Attributes, Block};

/// An element
///
/// # Examples
/// ## Without attributes
/// ```ignore
/// div {}
/// ```
///
/// ## With attributes
/// ```ignore
/// div(class="hello", name=format!("{first_name} {last_name}")) {}
/// ```
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

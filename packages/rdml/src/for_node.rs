use syn::{
    Expr, Pat, Result, Token,
    parse::{Parse, ParseStream},
};

use crate::Block;

#[derive(Debug, PartialEq, Hash)]
pub struct ForNode {
    pub for_token: Token![for],
    pub pat: Pat,
    pub in_token: Token![in],
    pub expr: Expr,
    pub body: Block,
}

impl Parse for ForNode {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self {
            for_token: input.parse()?,
            pat: input.call(Pat::parse_single)?,
            in_token: input.parse()?,
            expr: input.call(Expr::parse_without_eager_brace)?,
            body: input.parse()?,
        })
    }
}

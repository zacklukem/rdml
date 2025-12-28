use syn::{
    Expr, LitStr, Result, braced,
    parse::{Parse, ParseStream},
    token::Brace,
};

use crate::Element;

#[derive(Debug, PartialEq, Hash)]
pub struct ExprNode {
    brace_token: Brace,
    expr: Expr,
}

impl Parse for ExprNode {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        Ok(Self {
            brace_token: braced!(content in input),
            expr: content.parse()?,
        })
    }
}

#[derive(Debug, PartialEq, Hash)]
pub enum Node {
    Element(Element),
    Text(LitStr),
    Expr(ExprNode),
}

impl Parse for Node {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.peek(LitStr) {
            Ok(Self::Text(input.parse()?))
        } else if input.peek(Brace) {
            Ok(Self::Expr(input.parse()?))
        } else {
            Ok(Self::Element(input.parse()?))
        }
    }
}

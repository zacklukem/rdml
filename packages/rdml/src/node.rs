use syn::{
    Attribute, Expr, LitStr, Result, Token, braced,
    parse::{Parse, ParseStream},
    token::Brace,
};

use crate::{Element, ForNode, IfNode};

#[derive(Debug, PartialEq, Hash)]
pub struct ExprNode {
    pub brace_token: Brace,
    pub expr: Expr,
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
pub enum NodeType {
    Element(Element),
    Text(LitStr),
    Expr(ExprNode),
    If(IfNode),
    For(ForNode),
}

impl Parse for NodeType {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.peek(Token![if]) {
            Ok(Self::If(input.parse()?))
        } else if input.peek(Token![for]) {
            Ok(Self::For(input.parse()?))
        } else if input.peek(LitStr) {
            Ok(Self::Text(input.parse()?))
        } else if input.peek(Brace) {
            Ok(Self::Expr(input.parse()?))
        } else {
            Ok(Self::Element(input.parse()?))
        }
    }
}

#[derive(Debug, PartialEq, Hash)]
pub struct Node {
    pub attrs: Vec<Attribute>,
    pub node: NodeType,
}

impl Parse for Node {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Node {
            attrs: input.call(Attribute::parse_outer)?,
            node: input.parse()?,
        })
    }
}

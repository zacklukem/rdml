use syn::{
    Attribute, Expr, LitStr, Result, Token, parenthesized,
    parse::{Parse, ParseStream},
    token::{Brace, Paren},
};

use crate::{Block, Element, ForNode, IfNode, MatchNode};

#[derive(Debug, PartialEq, Hash)]
pub struct ExprNode {
    pub paren_token: Paren,
    pub expr: Expr,
}

impl Parse for ExprNode {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        Ok(Self {
            paren_token: parenthesized!(content in input),
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
    Match(MatchNode),
    Block(Block),
}

impl Parse for NodeType {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.peek(Token![if]) {
            Ok(Self::If(input.parse()?))
        } else if input.peek(Token![for]) {
            Ok(Self::For(input.parse()?))
        } else if input.peek(Token![match]) {
            Ok(Self::Match(input.parse()?))
        } else if input.peek(LitStr) {
            Ok(Self::Text(input.parse()?))
        } else if input.peek(Brace) {
            Ok(Self::Block(input.parse()?))
        } else if input.peek(Paren) {
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

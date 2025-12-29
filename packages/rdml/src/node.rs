use syn::{
    Attribute, Expr, LitStr, Result, Token, parenthesized,
    parse::{Parse, ParseStream},
    token::{Brace, Paren},
};

use crate::{Block, Element, ForNode, IfNode, MatchNode};

/// An expression interpolation node: `(1 + 1)`.
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

/// A [`Node`] without attributes
#[derive(Debug, PartialEq, Hash)]
pub enum NodeType {
    /// Element: `div {}`
    Element(Element),

    /// Text literal: `"Hello, world!"`
    Text(LitStr),

    /// Expression literal: `(1 + 1)`
    Expr(ExprNode),

    /// If node: `if condition { [...] }`
    If(IfNode),

    /// For node: `for pattern in expr { [...] }`
    For(ForNode),

    /// Match node: `match expr { [...] }`
    Match(MatchNode),

    /// A block fragment: `{ div {} span {} [...] }`
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

/// A node with attributes
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

use syn::{
    Expr, Pat, Result, Token, braced,
    parse::{Parse, ParseStream},
    token::Brace,
};

use crate::{Node, NodeType, helpers::ParseHelpers};

#[derive(Debug, PartialEq, Hash)]
pub struct MatchNode {
    pub match_token: Token![match],
    pub expr: Expr,
    pub brace_token: Brace,
    pub arms: Vec<MatchNodeArm>,
}

impl Parse for MatchNode {
    fn parse(input: ParseStream) -> Result<Self> {
        let contents;

        Ok(Self {
            match_token: input.parse()?,
            expr: input.call(Expr::parse_without_eager_brace)?,
            brace_token: braced!(contents in input),
            arms: contents.parse_all()?,
        })
    }
}

#[derive(Debug, PartialEq, Hash)]
pub struct MatchNodeArm {
    pub pat: Pat,
    pub guard: Option<(Token![if], Expr)>,
    pub fat_arrow_token: Token![=>],
    pub body: Node,
    pub comma: Option<Token![,]>,
}

impl Parse for MatchNodeArm {
    fn parse(input: ParseStream) -> Result<Self> {
        let requires_comma;
        Ok(Self {
            pat: input.call(Pat::parse_multi_with_leading_vert)?,
            guard: if input.peek(Token![if]) {
                Some((input.parse()?, input.parse()?))
            } else {
                None
            },
            fat_arrow_token: input.parse()?,
            body: {
                let body = input.parse()?;
                requires_comma = requires_comma_to_be_match_arm(&body);
                body
            },
            comma: {
                if requires_comma && !input.is_empty() {
                    Some(input.parse()?)
                } else {
                    input.parse()?
                }
            },
        })
    }
}

fn requires_comma_to_be_match_arm(body: &Node) -> bool {
    match &body.node {
        NodeType::Element(_)
        | NodeType::If(_)
        | NodeType::For(_)
        | NodeType::Match(_)
        | NodeType::Block(_) => false,

        NodeType::Expr(_) | NodeType::Text(_) => true,
    }
}

use syn::{
    Expr, Result, Token,
    parse::{Parse, ParseStream},
};

use crate::Block;

#[derive(Debug, PartialEq, Hash)]
pub struct IfNode {
    pub if_token: Token![if],
    pub cond: Expr,
    pub then_branch: Block,
    pub else_branch: Option<(Token![else], ElseNode)>,
}

impl Parse for IfNode {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self {
            if_token: input.parse()?,
            cond: input.call(Expr::parse_without_eager_brace)?,
            then_branch: input.parse()?,
            else_branch: input
                .peek(Token![else])
                .then(|| Result::Ok((input.parse()?, input.parse()?)))
                .transpose()?,
        })
    }
}

#[derive(Debug, PartialEq, Hash)]
pub enum ElseNode {
    If(Box<IfNode>),
    Else(Block),
}

impl Parse for ElseNode {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.peek(Token![if]) {
            Ok(Self::If(input.parse()?))
        } else {
            Ok(Self::Else(input.parse()?))
        }
    }
}

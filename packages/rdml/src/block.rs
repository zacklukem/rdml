use syn::{
    Result, braced,
    parse::{Parse, ParseStream},
    token::Brace,
};

use crate::{Node, helpers::ParseHelpers};

#[derive(Debug, PartialEq, Hash)]
pub struct Block {
    pub brace_token: Brace,
    pub nodes: Vec<Node>,
}

impl Parse for Block {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        Ok(Self {
            brace_token: braced!(content in input),
            nodes: content.parse_all()?,
        })
    }
}

#[derive(Debug, PartialEq, Hash)]
pub struct Nodes {
    pub nodes: Vec<Node>,
}

impl Parse for Nodes {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self {
            nodes: input.parse_all()?,
        })
    }
}

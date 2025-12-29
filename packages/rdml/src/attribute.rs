use syn::{
    Expr, Ident, LitStr, Path, Result, Token,
    ext::IdentExt,
    parenthesized,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    token::Paren,
};

/// An attribute name with a directive: `on:click`
#[derive(Debug, PartialEq, Hash)]
pub struct AttributeNameDirective {
    pub directive: Ident,
    pub colon_token: Token![:],
    pub path: Path,
}

impl Parse for AttributeNameDirective {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self {
            directive: input.parse()?,
            colon_token: input.parse()?,
            path: input.parse()?,
        })
    }
}

/// An attribute name
#[derive(Debug, PartialEq, Hash)]
pub enum AttributeName {
    /// A single attribute name: `class` or `::package::attributes::id`
    Single(Path),

    /// A quoted attribute name: `"aria-label"`
    Quoted(LitStr),

    /// A directive attribute name: `on:click` or `bind:binds::value`
    Directive(AttributeNameDirective),
}

impl Parse for AttributeName {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.peek(Ident::peek_any) && !input.peek2(Token![::]) && input.peek2(Token![:]) {
            Ok(AttributeName::Directive(input.parse()?))
        } else if input.peek(LitStr) {
            Ok(AttributeName::Quoted(input.parse()?))
        } else {
            Ok(AttributeName::Single(input.parse()?))
        }
    }
}

/// An attribute: `class="value"`
#[derive(Debug, PartialEq, Hash)]
pub struct Attribute {
    pub name: AttributeName,
    pub eq_token: Token![=],
    pub value: Expr,
}

impl Parse for Attribute {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self {
            name: input.parse()?,
            eq_token: input.parse()?,
            value: input.parse()?,
        })
    }
}

/// An list of attributes: `(class="value", id="value")`
#[derive(Debug, PartialEq, Hash)]
pub struct Attributes {
    pub paren_token: Paren,
    pub attributes: Punctuated<Attribute, Token![,]>,
}

impl Parse for Attributes {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        Ok(Self {
            paren_token: parenthesized!(content in input),
            attributes: content.parse_terminated(Attribute::parse, Token![,])?,
        })
    }
}

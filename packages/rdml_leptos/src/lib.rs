//! An alternative templating macro for leptos with more consise syntax based on [`rdml`](https://docs.rs/rdml)
//!
//! # Examples
//!
//! ## Elements
//!
//! Elements are made up of the tag or component name, followed by attributes and children surrounded by braces.
//!
//! Attributes are optional, must be surrounded by parenthesis, and are separated by commas.
//!
//! ```ignore
//! rdml! {
//!     div(id="root", class="container") {
//!         span { "Hello, world!" }
//!         ButtonComponent(on:click=move || println!("Clicked!")) {}
//!     }
//! }
//! ```
//! ## Text node
//!
//! Quoted text will be interpreted as a text node
//!
//! ```ignore
//! rdml! {
//!     div { "Text here" }
//! }
//! ```
//!
//! ## Expressions
//!
//! Expressions can be any rust expression surrounded by parenthesis.
//!
//! ```ignore
//! rdml! {
//!     div { (if i > 1 { "Greater" } else { "Less" }) }
//! }
//! ```
//!
//! ## If blocks
//!
//! If blocks can conditionally render certain nodes.
//!
//! ```ignore
//! rdml! {
//!     if i > 1 {
//!         span { "This is a span" }
//!     } else {
//!         div { "This is a div" }
//!     }
//! }
//! ```
//!
//! By default, the if generates creates a normal rust if expression in a closure (i.e. `{move || if condition {} [...]}`),
//! however the `#[show]` attribute can be applied to use the [`Show`](https://docs.rs/leptos/latest/leptos/control_flow/fn.Show.html)
//! component instead. (See [control flow](https://book.leptos.dev/view/06_control_flow.html) in the leptos book for more deatails).
//!
//! ```ignore
//! rdml! {
//!     #[show]
//!     if i > 1 {
//!         span { "This is a span" }
//!     } else {
//!         div { "This is a div" }
//!     }
//! }
//! ```
//!
//! ## For blocks
//!
//! For blocks can render a list of nodes
//!
//! ```ignore
//! rdml! {
//!     for i in 0..50 {
//!         div { (i) }
//!     }
//! }
//! ```
//!
//! By default, the for node collects the given iterable into a `Vec<_>`, however the [`For`](https://docs.rs/leptos/latest/leptos/control_flow/fn.For.html)
//! component can be used instead by adding the `#[key([expr])]` attribute. (See [iteration](https://book.leptos.dev/view/04_iteration.html) in the leptos book).
//!
//! ```ignore
//! rdml! {
//!     #[key(item.clone())]
//!     for item in items.get() {
//!         div { (item) }
//!     }
//! }
//! ```
//!
//! ## Match blocks
//!
//! You can also use match statements for control flow (this always generates a move closure with a rust match block)
//!
//! ```ignore
//! rdml! {
//!     match name {
//!         Some("a") => div { "a" }
//!         Some("b") => "b",
//!         Some("c") => {
//!             span { "c" }
//!             button {}
//!         }
//!         None => (other_name.to_string()),
//!     }
//! }
//! ```
//!
//! ## With attributes
//!
//! Most nodes and blocks can have the `#[with([stmt])]` attribute applied to enter a new scope with the given statment.
//! This attribute can be applied multiple times.
//!
//! ```ignore
//! rdml! {
//!     #[with(let value = x + 1;)]
//!     {
//!         div {}
//!         (value)
//!     }
//!     #[with(let ref = ref.clone();)]
//!     #[with(let ref1 = ref.clone();)]
//!     if condition {
//!         div { (ref) (ref1) }
//!     }
//! }
//! ```
//!

use proc_macro2::{Span, TokenStream};
use quote::{ToTokens, TokenStreamExt, quote, quote_spanned};
use rdml::{
    Attribute, AttributeName, Block, Element, ElseNode, ExprNode, ForNode, IfNode, MatchNode,
    MatchNodeArm, Node, NodeType, Nodes,
};
use syn::{Expr, Result, Stmt, parse_macro_input, spanned::Spanned, token::Paren};

fn generate_attribute_name(attr_name: &AttributeName) -> Result<TokenStream> {
    match attr_name {
        AttributeName::Single(path) => Ok(quote! { #path }), // TODO: assert not path
        AttributeName::Directive(rdml::AttributeNameDirective {
            directive,
            path,
            colon_token,
        }) => Ok(quote! {#directive #colon_token #path }),
        AttributeName::Quoted(_lit_str) => todo!("Implement quoted attributes"),
    }
}

fn generate_attribute(attr: &Attribute) -> Result<TokenStream> {
    let name = generate_attribute_name(&attr.name)?;
    let eq_token = &attr.eq_token;
    let value = &attr.value;

    Ok(quote_spanned! {eq_token.span()=>
        #name #eq_token {#value}
    })
}

fn generate_element(el: &Element) -> Result<TokenStream> {
    let path = &el.path;

    let attributes = el
        .attributes
        .as_ref()
        .map(|attributes| {
            let builders = attributes
                .attributes
                .iter()
                .map(generate_attribute)
                .collect::<Result<Vec<_>>>()?;
            Result::Ok(quote! {#(#builders )*})
        })
        .transpose()?;

    let children = generate_block(&el.children)?;

    Ok(quote_spanned! {path.span()=>
        <#path #attributes>
            #children
        </#path>
    })
}

fn generate_block(block: &Block) -> Result<TokenStream> {
    let nodes = block
        .nodes
        .iter()
        .map(generate_node)
        .collect::<Result<Vec<_>>>()?;

    Ok(quote! { #(#nodes)* })
}

fn generate_if_node(if_node: &IfNode, attrs: &[syn::Attribute]) -> Result<TokenStream> {
    if attrs
        .iter()
        .any(|attr| attr.path().get_ident().is_some_and(|id| id == "show"))
    {
        fn generate_rec(node: &IfNode) -> Result<TokenStream> {
            let if_token = &node.if_token;
            let cond = &node.cond;
            let fallback = node.else_branch.as_ref().map(|(else_token, else_branch)| {
                let else_branch = match else_branch {
                    ElseNode::If(if_node) => generate_rec(if_node)?,
                    ElseNode::Else(block) => generate_block(block)?,
                };
                Result::Ok(quote_spanned! {else_token.span()=> fallback=(move || view! { #else_branch })})
            }).transpose()?;
            let then_branch = generate_block(&node.then_branch)?;
            Ok(quote_spanned! {if_token.span()=>
                <Show
                    when=(move || #cond)
                    #fallback
                >
                    #then_branch
                </Show>
            })
        }

        generate_rec(if_node)
    } else {
        let if_token = &if_node.if_token;
        let cond = &if_node.cond;
        let then_branch = generate_block(&if_node.then_branch)?;
        let mut result = quote_spanned! {if_token.span()=>
            #if_token #cond {
                view! { #then_branch }.into_any()
            }
        };
        let mut else_branch = if_node.else_branch.as_ref();

        let mut had_else = false;

        while let Some((else_token, else_node)) = else_branch {
            match else_node {
                ElseNode::If(if_node) => {
                    let if_token = &if_node.if_token;
                    let cond = &if_node.cond;
                    let then_branch = generate_block(&if_node.then_branch)?;
                    result.append_all(quote_spanned! {else_token.span()=>
                        #else_token #if_token #cond {
                            view! { #then_branch }.into_any()
                        }
                    });
                    else_branch = if_node.else_branch.as_ref();
                }
                ElseNode::Else(block) => {
                    let block = generate_block(&block)?;
                    result.append_all(quote_spanned! {else_token.span()=>
                        #else_token {
                            view! { #block }.into_any()
                        }
                    });
                    had_else = true;
                    break;
                }
            }
        }

        if !had_else {
            result.append_all(quote_spanned! {if_token.span()=>
                else {
                    view! { }.into_any()
                }
            });
        }

        Ok(quote_spanned! {if_token.span()=> {move || #result} })
    }
}

fn generate_for_node(for_node: &ForNode, attrs: &[syn::Attribute]) -> Result<TokenStream> {
    let key_attr = attrs
        .iter()
        .find(|attr| attr.path().get_ident().is_some_and(|id| id == "key"));

    let for_token = &for_node.for_token;
    let pat = &for_node.pat;
    let expr = &for_node.expr;
    let body = generate_block(&for_node.body)?;

    if let Some(key_attr) = key_attr {
        let key: Expr = key_attr.parse_args().unwrap(); // TODO: expose error
        Ok(quote_spanned! {for_token.span()=>
            <For
                each=(move || { #expr })
                key=(move |#pat| { #key })
                children=(move |#pat| { view! { #body } })
            />
        })
    } else {
        Ok(quote_spanned! {for_token.span()=>
            {(#expr).into_iter().map(|#pat| view! { #body }).collect::<Vec<_>>()}
        })
    }
}

fn generate_match_node_arm(arm: &MatchNodeArm) -> Result<TokenStream> {
    let pat = &arm.pat;
    let guard = arm
        .guard
        .as_ref()
        .map(|(if_token, expr)| quote! { #if_token #expr });
    let fat_arrow_token = &arm.fat_arrow_token;
    let body = generate_node(&arm.body)?;
    Ok(quote_spanned! {fat_arrow_token.span()=>
        #pat #guard #fat_arrow_token view! { #body }.into_any(),
    })
}

fn generate_match_node(node: &MatchNode) -> Result<TokenStream> {
    let match_token = &node.match_token;
    let expr = &node.expr;
    let arms = node
        .arms
        .iter()
        .map(generate_match_node_arm)
        .collect::<Result<Vec<_>>>()?;
    Ok(quote_spanned! {match_token.span()=>
        {move || #match_token #expr {
            #(#arms)*
        }}
    })
}

fn paren_span(paren: &Paren) -> Span {
    let mut tokens = quote! {};
    paren.surround(&mut tokens, |_| {});
    tokens.span()
}

fn generate_node(node: &Node) -> Result<TokenStream> {
    let node_tokens = match &node.node {
        NodeType::Element(element) => generate_element(element)?,
        NodeType::Text(lit_str) => lit_str.to_token_stream(),
        NodeType::Expr(ExprNode { expr, paren_token }) => {
            quote_spanned! {paren_span(&paren_token)=>{ #expr }}
        }
        NodeType::If(if_node) => generate_if_node(if_node, &node.attrs)?,
        NodeType::For(for_node) => generate_for_node(for_node, &node.attrs)?,
        NodeType::Match(match_node) => generate_match_node(match_node)?,
        NodeType::Block(block) => generate_block(block)?,
    };

    let with_attr = node
        .attrs
        .iter()
        .filter(|attr| attr.path().get_ident().is_some_and(|id| id == "with"))
        .map(|attr| attr.parse_args::<Stmt>())
        .collect::<Result<Vec<_>>>()?;

    if !with_attr.is_empty() {
        let span = node
            .attrs
            .iter()
            .filter(|attr| attr.path().get_ident().is_some_and(|id| id == "with"))
            .next()
            // Ok because with_attr is non-empty
            .unwrap()
            .path()
            .span();

        Ok(quote_spanned! {span=>
            {{
                #(#with_attr)*
                view! { #node_tokens }
            }}
        })
    } else {
        Ok(node_tokens)
    }
}

#[proc_macro]
pub fn rdml(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let nodes = parse_macro_input!(tokens as Nodes);

    let nodes = nodes
        .nodes
        .iter()
        .map(generate_node)
        .collect::<Result<Vec<_>>>();

    match nodes {
        Ok(nodes) => quote! {{
            #[allow(unused_variables)]
            #[allow(unused_parens)]
            #[allow(unused_braces)]
            {
                ::leptos::prelude::view! {
                    #(#nodes)*
                }
            }
        }}
        .into(),
        Err(error) => error.into_compile_error().into(),
    }
}

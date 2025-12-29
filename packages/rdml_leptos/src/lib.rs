use proc_macro2::TokenStream;
use quote::{ToTokens, TokenStreamExt, quote};
use rdml::{
    Attribute, AttributeName, Block, Element, ElseNode, ExprNode, ForNode, IfNode, MatchNode,
    MatchNodeArm, Node, NodeType, Nodes,
};
use syn::{Expr, parse_macro_input};

fn generate_attribute_name(attr_name: &AttributeName) -> TokenStream {
    match attr_name {
        AttributeName::Single(path) => quote! { #path }, // TODO: assert not path
        AttributeName::Directive(rdml::AttributeNameDirective {
            directive, path, ..
        }) => quote! { #directive:#path },
        AttributeName::Quoted(_lit_str) => todo!("Implement quoted attributes"),
    }
}

fn generate_attribute(attr: &Attribute) -> TokenStream {
    let name = generate_attribute_name(&attr.name);
    let eq_token = &attr.eq_token;
    let value = &attr.value;

    quote! {
        #name #eq_token {#value}
    }
}

fn generate_element(el: &Element) -> TokenStream {
    let path = &el.path;

    let attributes = el.attributes.as_ref().map(|attributes| {
        let builders = attributes.attributes.iter().map(generate_attribute);
        quote! {#(#builders )*}
    });

    let children = generate_block(&el.children);

    quote! {
        <#path #attributes>
            #children
        </#path>
    }
}

fn generate_block(block: &Block) -> TokenStream {
    let nodes = block.nodes.iter().map(generate_node);

    quote! { #(#nodes)* }
}

fn generate_if_node(if_node: &IfNode) -> TokenStream {
    let if_token = &if_node.if_token;
    let cond = &if_node.cond;
    let then_branch = generate_block(&if_node.then_branch);
    let mut result = quote! {
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
                let then_branch = generate_block(&if_node.then_branch);
                result.append_all(quote! {
                    #else_token #if_token #cond {
                        view! { #then_branch }.into_any()
                    }
                });
                else_branch = if_node.else_branch.as_ref();
            }
            ElseNode::Else(block) => {
                let block = generate_block(&block);
                result.append_all(quote! {
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
        result.append_all(quote! {
            else {
                view! { }.into_any()
            }
        });
    }

    quote! { {move || #result} }
}

fn generate_for_node(for_node: &ForNode, attrs: &[syn::Attribute]) -> TokenStream {
    let key_attr = attrs
        .iter()
        .find(|attr| attr.path().get_ident().is_some_and(|id| id == "key"));

    let pat = &for_node.pat;
    let expr = &for_node.expr;
    let body = generate_block(&for_node.body);

    if let Some(key_attr) = key_attr {
        let key: Expr = key_attr.parse_args().unwrap();
        let a = quote! {
            <For
                each=(move || { #expr })
                key=(move |#pat| { #key })
                children=(move |#pat| { view! { #body } })
            />
        };

        println!("{a}");

        a
    } else {
        quote! {
            {(#expr).into_iter().map(|#pat| view! { #body }).collect::<Vec<_>>()}
        }
    }
}

fn generate_match_node_arm(arm: &MatchNodeArm) -> TokenStream {
    let pat = &arm.pat;
    let guard = arm
        .guard
        .as_ref()
        .map(|(if_token, expr)| quote! { #if_token #expr });
    let fat_arrow_token = &arm.fat_arrow_token;
    let body = generate_node(&arm.body);
    quote! {
        #pat #guard #fat_arrow_token view! { #body }.into_any(),
    }
}

fn generate_match_node(node: &MatchNode) -> TokenStream {
    let match_token = &node.match_token;
    let expr = &node.expr;
    let arms = node.arms.iter().map(generate_match_node_arm);
    quote! {
        {move || #match_token #expr {
            #(#arms)*
        }}
    }
}

fn generate_node(node: &Node) -> TokenStream {
    match &node.node {
        NodeType::Element(element) => generate_element(element),
        NodeType::Text(lit_str) => lit_str.to_token_stream(),
        NodeType::Expr(ExprNode { expr, .. }) => quote! {{ #expr }},
        NodeType::If(if_node) => generate_if_node(if_node),
        NodeType::For(for_node) => generate_for_node(for_node, &node.attrs),
        NodeType::Match(match_node) => generate_match_node(match_node),
        NodeType::Block(block) => generate_block(block),
    }
}

#[proc_macro]
pub fn rdml(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let nodes = parse_macro_input!(tokens as Nodes);

    let nodes = nodes.nodes.iter().map(generate_node);

    quote! {
        ::leptos::prelude::view! {
            #(#nodes)*
        }
    }
    .into()
}

mod helpers;

mod attribute;
mod block;
mod element;
mod for_node;
mod if_node;
mod match_node;
mod node;

pub use attribute::*;
pub use block::*;
pub use element::*;
pub use for_node::*;
pub use if_node::*;
pub use match_node::*;
pub use node::*;

#[cfg(test)]
mod tests {
    use crate::*;

    macro_rules! snapshot_test {
        ($($input:tt)*) => {{
            let result: Nodes = syn::parse_quote! {$($input)*};
            insta::assert_debug_snapshot!(result);
        }};
    }

    #[test]
    fn test_parse_empty() {
        let result: Nodes = syn::parse_quote! {};
        assert_eq!(result, Nodes { nodes: vec![] })
    }

    #[test]
    fn test_parse_single_node() {
        snapshot_test! {
            div {}
            ::full::path::to::div {}
            Component::<WithGenerics> {}
        }
    }

    #[test]
    fn test_match_node() {
        snapshot_test! {
            match expr {
                Some(X) | Some(Y) => {
                    div {}
                }
                Some(A) | Some(B) if true => "1",
                Some(C) => "2",
                None => span {},
            }
        }
    }

    #[test]
    fn test_parse_attributes() {
        snapshot_test! {
            div(
                single="single",
                "quoted"="quoted",
                dir:ective="directive",
                ::rdml::attribute::Attribute="leading non-directive path",
                rdml::attribute::Attribute="non-directive path",
                directive:rdml::attribute::Attribute="directive path",
            ) {}
        }
    }

    #[test]
    fn test_parse_literals() {
        snapshot_test! {
            "text here"
            div {
                "stuff inside of elements"
            }
            (expr(here))
        }
    }

    #[test]
    fn test_parse_node_attribute() {
        snapshot_test! {
            #[attribute]
            div {}
            #[attribute(|| expr)]
            if true {}
            #[attribute]
            "hello"
            #[attribute]
            (expr)
        }
    }

    #[test]
    fn test_parse_for() {
        snapshot_test! {
            for pattern in expr {
                "for body"
            }
        }
    }

    #[test]
    fn test_parse_if() {
        snapshot_test! {
            if condition {
                "if body"
            }

            if condition && else_block {
                "if body"
            } else {
                "else body"
            }

            if condition && else_block {
                "if body"
            } else if condition && else_if_block {
                "else if body"
            } else {
                "else body"
            }
        }
    }
}

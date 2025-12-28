mod helpers;

mod attribute;
mod block;
mod element;
mod if_node;
mod node;

pub use attribute::*;
pub use block::*;
pub use element::*;
pub use if_node::*;
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
            {expr(here)}
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

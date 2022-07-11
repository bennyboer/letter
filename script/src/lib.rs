use comrak::markdown_to_html;

#[cfg(test)]
mod tests {
    use crate::*;
    use comrak::nodes::{Ast, AstNode, NodeValue};
    use comrak::ComrakRenderOptions;
    use comrak::{format_html, parse_document, Arena, ComrakOptions};

    #[test]
    fn test_1() {
        assert_eq!(
            markdown_to_html(
                "<document><i>Hello</i> **there**!</document>",
                &ComrakOptions {
                    render: ComrakRenderOptions {
                        unsafe_: true,
                        ..ComrakRenderOptions::default()
                    },
                    ..ComrakOptions::default()
                }
            ),
            "<p>Hello, <strong>世界</strong>!</p>\n"
        );
    }

    #[test]
    fn test_2() {
        // The returned nodes are created in the supplied Arena, and are bound by its lifetime.
        let arena = Arena::new();

        let root = parse_document(
            &arena,
            "Some content before<footnote>Richtig cool</footnote>\n
<document>
This is my input.

1. Also my input.
2. Certainly my input.

<b>Some bold text</b>
</document>
",
            &ComrakOptions::default(),
        );

        fn iter_nodes<'a, F>(node: &'a AstNode<'a>, f: &F)
        where
            F: Fn(&'a AstNode<'a>),
        {
            f(node);
            for c in node.children() {
                iter_nodes(c, f);
            }
        }

        iter_nodes(root, &|node| match &mut node.data.borrow_mut().value {
            &mut NodeValue::Text(ref mut text) => {
                let orig = std::mem::replace(text, vec![]);
                let s = String::from_utf8(orig).unwrap();
                println!("[Text] {s}");
                *text = s.replace("my", "your").as_bytes().to_vec();
            }
            NodeValue::HtmlBlock(block) => {
                let s = String::from_utf8_lossy(&block.literal);
                println!("[HTML Block] {s}");
            }
            NodeValue::HtmlInline(text) => {
                let s = String::from_utf8_lossy(text);
                println!("[HTML Inline] {s}");
            }
            _ => {}
        });

        let mut html = vec![];
        format_html(root, &ComrakOptions::default(), &mut html).unwrap();

        assert_eq!(
            String::from_utf8(html).unwrap(),
            "<p>This is your input.</p>\n\
     <ol>\n\
     <li>Also your input.</li>\n\
     <li>Certainly your input.</li>\n\
     </ol>\n\
     <b>Some bold text</b>"
        );
    }
}

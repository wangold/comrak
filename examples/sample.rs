// Samples used in the README.  Wanna make sure they work as advertised.

fn small() {
    use comrak::{markdown_to_html, Options};

    assert_eq!(
        markdown_to_html("Hello, **世界**!", &Options::default()),
        "<p>Hello, <strong>世界</strong>!</p>\n"
    );
}

fn large() {
    use comrak::nodes::{AstNode, NodeValue};
    use comrak::{format_html, parse_document, Arena, Options};

    // The returned nodes are created in the supplied Arena, and are bound by its lifetime.
    let arena = Arena::new();

    let root = parse_document(
        &arena,
        "This is my input.\n\n1. Also my input.\n2. Certainly my input.\n",
        &Options::default(),
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

    iter_nodes(root, &|node| {
        if let NodeValue::Text(ref mut text) = node.data.borrow_mut().value {
            let orig = std::mem::take(text);
            *text = orig.replace("my", "your");
        }
    });

    let mut html = vec![];
    format_html(root, &Options::default(), &mut html).unwrap();

    assert_eq!(
        String::from_utf8(html).unwrap(),
        "<p>This is your input.</p>\n\
         <ol>\n\
         <li>Also your input.</li>\n\
         <li>Certainly your input.</li>\n\
         </ol>\n"
    );
}

fn main() {
    small();
    large();
}

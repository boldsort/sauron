use log::*;
use sauron::{
    html::{
        attributes::*,
        events::*,
        *,
    },
    mt_dom::patch::*,
    Patch,
    *,
};

#[test]
fn test_render() {
    let view1: Node<()> = main(
        vec![class("container")],
        vec![article(vec![inner_html("<h1>Lorep Ipsum</h1>")], vec![])],
    );

    let expected = r#"<main class="container"><article ><h1>Lorep Ipsum</h1></article></main>"#;

    assert_eq!(expected, view1.render_to_string());
}

#[test]
fn test_self_closing_tag() {
    let view1: Node<()> = main(
        vec![class("container")],
        vec![
            input(vec![type_("text")], vec![]),
            img(vec![src("image1.jpg")], vec![]),
        ],
    );

    let expected = r#"<main class="container"><input type="text"/><img src="image1.jpg"/></main>"#;

    assert_eq!(expected, view1.render_to_string());
}
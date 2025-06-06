use asciidork_ast::prelude::*;
use asciidork_parser::prelude::*;
use test_utils::*;

#[test]
fn test_parse_section() {
  assert_section!(
    adoc! {"
      == foo

      bar
    "},
    Section {
      meta: chunk_meta!(0),
      level: 1,
      id: Some(bstr!("_foo")),
      heading: nodes![node!("foo"; 3..6)],
      blocks: vecb![Block {
        context: BlockContext::Paragraph,
        content: BlockContent::Simple(nodes![node!("bar"; 8..11)]),
        ..empty_block!(8, 11)
      }],
      loc: (0..11).into()
    }
  );
}

#[test]
fn test_parse_section_w_reftext() {
  assert_section!(
    adoc! {r#"
      [reftext=so _baz_]
      == foo

      bar
    "#},
    reftext: Some(nodes![
      node!("so "; 9..12),
      node!(Inline::Italic(just!("baz", 13..16)), 12..17),
    ]),
    Section {
      meta: ChunkMeta {
        attrs: vecb![AttrList {
          positional: vecb![None],
          named: Named::from(vecb![
             (src!("reftext", 1..8), nodes![
               node!("so "; 9..12),
               node!(Inline::Italic(just!("baz", 13..16)), 12..17),
             ])
          ]),
          ..attr_list!(0..18)
        }].into(),
        ..chunk_meta!(0, 1)
      },
      level: 1,
      id: Some(bstr!("_foo")),
      heading: nodes![node!("foo"; 22..25)],
      blocks: vecb![Block {
        context: BlockContext::Paragraph,
        content: BlockContent::Simple(nodes![node!("bar"; 27..30)]),
        ..empty_block!(27, 30)
      }],
        loc: (19..30).into()
    }
  );
}

#[test]
fn test_parse_nested_section() {
  assert_section!(
    adoc! {"
      == one

      === two

      bar
    "},
    Section {
      meta: chunk_meta!(0),
      level: 1,
      id: Some(bstr!("_one")),
      heading: nodes![node!("one"; 3..6)],
      blocks: vecb![Block {
        meta: chunk_meta!(8),
        loc: (8..20).into(),
        context: BlockContext::Section,
        content: BlockContent::Section(Section {
          meta: chunk_meta!(8),
          level: 2,
          id: Some(bstr!("_two")),
          heading: nodes![node!("two"; 12..15)],
          blocks: vecb![Block {
            context: BlockContext::Paragraph,
            content: BlockContent::Simple(nodes![node!("bar"; 17..20)]),
            ..empty_block!(17, 20)
          }],
          loc: (8..20).into()
        }),
      }],
      loc: (0..20).into()
    }
  );
}

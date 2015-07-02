extern crate diecast;
extern crate pulldown_cmark;
extern crate typemap;

use diecast::{Handle, Item};

use pulldown_cmark::{Parser, html};

pub struct CommonMark;

pub fn commonmark() -> CommonMark {
    CommonMark
}

impl Handle<Item> for CommonMark {
    fn handle(&self, item: &mut Item) -> diecast::Result<()> {
        let mut s = String::with_capacity(item.body.len() * 3 / 2);

        {
            let p = Parser::new(&item.body);
            html::push_html(&mut s, p);
        }

        item.body = s;

        Ok(())
    }
}

#[test]
fn test_render() {
    use diecast::util::handle::item;

    let mut test = Item::reading("tests/input.md");

    item::read(&mut test).unwrap();

    CommonMark.handle(&mut test).unwrap();

    assert_eq!(&test.body, "<p>this <em>is</em> a <strong>test</strong></p>\n");
}

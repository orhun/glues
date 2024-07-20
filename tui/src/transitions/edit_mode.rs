use {
    crate::{traits::*, Node},
    cursive::Cursive,
};

pub fn edit_mode(siv: &mut Cursive) {
    Node::editor().content().find(siv).enable();

    siv.focus_name(&Node::editor().content().name())
        .log_unwrap();
}

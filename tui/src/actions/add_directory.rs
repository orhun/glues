use {
    crate::{actions, traits::*, views::note_tree::directory::render_directory, Node},
    cursive::Cursive,
    glues_core::types::DirectoryId,
};

pub fn add_directory(siv: &mut Cursive, parent_id: &DirectoryId, directory_name: &str) {
    // data
    let directory = siv
        .glues()
        .add_directory(parent_id.clone(), directory_name.to_owned())
        .log_unwrap();
    let directory_id = directory.id.clone();

    // ui
    if !siv.glues().check_opened(parent_id) {
        actions::open_directory(siv, parent_id);
    } else {
        let mut container = if &siv.glues().root_id == parent_id {
            Node::note_tree().note_list().find(siv)
        } else {
            Node::note_tree().directory(parent_id).note_list().find(siv)
        };

        container.add_child(render_directory(siv, directory));
    }

    siv.cb_sink()
        .send(Box::new(move |siv| {
            siv.focus_name(
                &Node::note_tree()
                    .directory(&directory_id)
                    .name_button()
                    .name(),
            )
            .log_unwrap();
        }))
        .log_unwrap();
}
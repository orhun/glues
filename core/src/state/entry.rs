use crate::{state::note_tree::NoteTreeState, Error, Event, Glues, Result, Transition};

pub struct EntryState;

impl EntryState {
    pub async fn consume(glues: &mut Glues, event: Event) -> Result<Transition> {
        match event {
            Event::Initialize => {
                glues.state = NoteTreeState::new(glues).await?.into();

                Ok(Transition::Initialize)
            }
            Event::Key(_) => Ok(Transition::Inedible(event)),
            _ => Err(Error::Wip("todo: EntryState::consume".to_owned())),
        }
    }

    pub fn describe(&self) -> Result<String> {
        Ok("Entry".to_owned())
    }

    pub fn shortcuts(&self) -> Vec<&str> {
        vec![":)"]
    }
}

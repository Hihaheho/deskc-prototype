use components::{snapshot::Snapshot, event::Event};

#[derive(Default)]
pub struct History {}

impl History {
    pub fn handle_event(&mut self, _snapshot: &Snapshot, _log: &Event) {}
}

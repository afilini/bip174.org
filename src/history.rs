use bitcoin::psbt;

use crate::app::*;

#[derive(Clone, Debug, Default)]
pub struct History {
    items: Vec<PsbtMessage>,
    position: usize,
}

impl History {
    pub fn add(&mut self, action: PsbtMessage) {
        self.items.drain(self.position..);
        self.items.push(action);
        self.position += 1;
    }

    pub fn undo(&mut self, psbt: &mut Option<psbt::Psbt>) -> bool {
        let prev_position = match self.position {
            0 => return false,
            x => x - 1,
        };

        let action = &self.items[prev_position];
        self.items[prev_position] = action.clone().apply_to(psbt);
        self.position -= 1;

        true
    }

    pub fn redo(&mut self, psbt: &mut Option<psbt::Psbt>) -> bool {
        if let Some(action) = self.items.get(self.position) {
            self.items[self.position] = action.clone().apply_to(psbt);
            self.position += 1;

            true
        } else {
            false
        }
    }
}

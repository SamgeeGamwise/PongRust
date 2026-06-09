use crate::state_machine::state::State;

pub enum StateEvent {
    None,
    Push(Box<dyn State>),
    Pop,
    Switch(Box<dyn State>),
}
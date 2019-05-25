use amethyst::{
    prelude::*,
    renderer::VirtualKeyCode,
    input::is_key_down
};

pub struct PauseState;

impl SimpleState for PauseState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        println!("Paused");
    }

    fn handle_event(&mut self, _data: StateData<'_, GameData<'_, '_>>, event: StateEvent) -> SimpleTrans {
        if let StateEvent::Window(event) = &event {
            if is_key_down(&event, VirtualKeyCode::Escape) {
                println!("Resumed");
                return Trans::Pop;
            }
        }

        Trans::None
    }
}
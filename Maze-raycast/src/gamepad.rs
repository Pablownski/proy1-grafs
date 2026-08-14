use gilrs::{Axis, Gilrs};

const DEADZONE: f32 = 0.15;

#[derive(Default, Clone, Copy)]
pub struct GamepadState {
    pub move_axis: f32,
    pub turn_axis: f32,
}

fn apply_deadzone(v: f32) -> f32 {
    if v.abs() < DEADZONE {
        0.0
    } else {
        v
    }
}

pub struct GamepadInput {
    gilrs: Option<Gilrs>,
}

impl GamepadInput {
    pub fn new() -> Self {
        match Gilrs::new() {
            Ok(gilrs) => GamepadInput { gilrs: Some(gilrs) },
            Err(e) => {
                eprintln!("gamepad: no se pudo inicializar gilrs: {e}");
                GamepadInput { gilrs: None }
            }
        }
    }

    pub fn poll(&mut self) -> GamepadState {
        let Some(gilrs) = &mut self.gilrs else {
            return GamepadState::default();
        };

        while gilrs.next_event().is_some() {}

        if let Some((_id, gamepad)) = gilrs.gamepads().next() {
            let move_axis = apply_deadzone(gamepad.value(Axis::LeftStickY));
            let turn_axis = apply_deadzone(gamepad.value(Axis::RightStickX));
            GamepadState { move_axis, turn_axis }
        } else {
            GamepadState::default()
        }
    }
}

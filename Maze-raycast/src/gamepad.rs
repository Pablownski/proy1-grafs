use std::time::{Duration, Instant};

use gilrs::{Axis, Button, Event, EventType, Gilrs};

const DEADZONE: f32 = 0.15;
const MENU_STICK_THRESHOLD: f32 = 0.5;
const MENU_STICK_REPEAT_DELAY: Duration = Duration::from_millis(220);

#[derive(Default, Clone, Copy)]
pub struct GamepadState {
    pub move_axis: f32,
    pub turn_axis: f32,
    pub confirm_pressed: bool,
    pub up_pressed: bool,
    pub down_pressed: bool,
    pub map_toggle_pressed: bool,
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
    stick_dir: i8,
    last_stick_repeat: Instant,
}

impl GamepadInput {
    pub fn new() -> Self {
        let gilrs = match Gilrs::new() {
            Ok(gilrs) => Some(gilrs),
            Err(e) => {
                eprintln!("gamepad: no se pudo inicializar gilrs: {e}");
                None
            }
        };
        GamepadInput {
            gilrs,
            stick_dir: 0,
            last_stick_repeat: Instant::now(),
        }
    }

    pub fn poll(&mut self) -> GamepadState {
        let Some(gilrs) = &mut self.gilrs else {
            return GamepadState::default();
        };

        let mut confirm_pressed = false;
        let mut up_pressed = false;
        let mut down_pressed = false;
        let mut map_toggle_pressed = false;

        while let Some(Event { event, .. }) = gilrs.next_event() {
            if let EventType::ButtonPressed(button, _) = event {
                match button {
                    // South = X en PlayStation / A en Xbox (boton inferior estandar)
                    Button::South => confirm_pressed = true,
                    Button::DPadUp => up_pressed = true,
                    Button::DPadDown => down_pressed = true,
                    // North = Triangulo en PlayStation / Y en Xbox
                    Button::North => map_toggle_pressed = true,
                    _ => {}
                }
            }
        }

        if let Some((_id, gamepad)) = gilrs.gamepads().next() {
            let move_axis = apply_deadzone(gamepad.value(Axis::LeftStickY));
            let turn_axis = apply_deadzone(gamepad.value(Axis::RightStickX));

            let raw_y = gamepad.value(Axis::LeftStickY);
            let dir = if raw_y > MENU_STICK_THRESHOLD {
                1
            } else if raw_y < -MENU_STICK_THRESHOLD {
                -1
            } else {
                0
            };

            let now = Instant::now();
            if dir != 0
                && (dir != self.stick_dir
                    || now.duration_since(self.last_stick_repeat) >= MENU_STICK_REPEAT_DELAY)
            {
                if dir > 0 {
                    up_pressed = true;
                } else {
                    down_pressed = true;
                }
                self.last_stick_repeat = now;
            }
            self.stick_dir = dir;

            GamepadState {
                move_axis,
                turn_axis,
                confirm_pressed,
                up_pressed,
                down_pressed,
                map_toggle_pressed,
            }
        } else {
            GamepadState::default()
        }
    }
}

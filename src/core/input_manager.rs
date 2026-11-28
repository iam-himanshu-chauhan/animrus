use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct InputAction(String);

impl InputAction {
    pub fn new(name: impl Into<String>) -> Self {
        Self(name.into())
    }

    pub fn name(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputState {
    Idle,
    Pressed,
    Held,
    Released,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InputBinding {
    Key(egui::Key),
    MouseButton(egui::PointerButton),
    MouseWheel,
}

pub struct InputManager {
    bindings: HashMap<InputAction, Vec<InputBinding>>,
    states: HashMap<InputAction, InputState>,
    values: HashMap<InputAction, egui::Vec2>,
}

impl InputManager {
    pub fn new() -> Self {
        Self {
            bindings: HashMap::new(),
            states: HashMap::new(),
            values: HashMap::new(),
        }
    }

    pub fn bind(&mut self, action: InputAction, binding: InputBinding) {
        let bindings = self.bindings.entry(action).or_default();

        if !bindings.contains(&binding) {
            bindings.push(binding);
        }
    }

    pub fn unbind(&mut self, action: &InputAction, binding: &InputBinding) {
        if let Some(bindings) = self.bindings.get_mut(action) {
            bindings.retain(|b| b != binding);

            if bindings.is_empty() {
                self.bindings.remove(action);
            }
        }
    }

    pub fn update(&mut self, ctx: &egui::Context) {
        for (action, bindings) in &self.bindings {
            let previous_state = self.action_state(action);
            let mut wheel_delta = egui::Vec2::ZERO;

            let (is_down, was_pressed, was_released, _wheel_moved) = ctx.input(|input| {
                let mut is_down = false;
                let mut was_pressed = false;
                let mut was_released = false;

                for binding in bindings {
                    match binding {
                        InputBinding::Key(key) => {
                            is_down |= input.key_down(*key);
                            was_pressed |= input.key_pressed(*key);
                            was_released |= input.key_released(*key);
                        }

                        InputBinding::MouseButton(button) => {
                            is_down |= input.pointer.button_down(*button);
                            was_pressed |= input.pointer.button_pressed(*button);
                            was_released |= input.pointer.button_released(*button);
                        }

                        InputBinding::MouseWheel => {
                            wheel_delta = input.smooth_scroll_delta;
                        }
                    }
                }

                if wheel_delta != egui::Vec2::ZERO {
                    self.values.insert(action.clone(), wheel_delta);
                } else {
                    self.values.insert(action.clone(), egui::Vec2::ZERO);
                }

                (is_down, was_pressed, was_released, wheel_delta)
            });

            let wheel_moved = wheel_delta != egui::Vec2::ZERO;

            let state = if wheel_moved || was_pressed {
                InputState::Pressed
            } else if is_down {
                InputState::Held
            } else if was_released
                || matches!(previous_state, InputState::Pressed | InputState::Held)
            {
                InputState::Released
            } else {
                InputState::Idle
            };

            self.states.insert(action.clone(), state);
        }
    }

    pub fn get_bindings(&self, action: &InputAction) -> Option<&[InputBinding]> {
        self.bindings.get(action).map(Vec::as_slice)
    }

    pub fn action_value(&self, action: &InputAction) -> egui::Vec2 {
        self.values.get(action).copied().unwrap_or(egui::Vec2::ZERO)
    }

    pub fn action_state(&self, action: &InputAction) -> InputState {
        self.states.get(action).copied().unwrap_or(InputState::Idle)
    }

    pub fn is_pressed(&self, action: &InputAction) -> bool {
        self.action_state(action) == InputState::Pressed
    }

    pub fn is_held(&self, action: &InputAction) -> bool {
        matches!(
            self.action_state(action),
            InputState::Pressed | InputState::Held
        )
    }

    pub fn is_released(&self, action: &InputAction) -> bool {
        self.action_state(action) == InputState::Released
    }
}

impl Default for InputManager {
    fn default() -> Self {
        Self::new()
    }
}

use std::collections::HashMap;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum InputState {
    #[default]
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

#[derive(Default)]
pub struct ActionData {
    bindings: Vec<InputBinding>,
    state: InputState,
    value: egui::Vec2,
}

pub struct InputManager {
    actions: HashMap<InputAction, ActionData>,
}

impl InputManager {
    pub fn new() -> Self {
        Self {
            actions: HashMap::new(),
        }
    }

    pub fn bind(&mut self, action: InputAction, binding: InputBinding) {
        let action_data = self.actions.entry(action).or_default();

        if !action_data.bindings.contains(&binding) {
            action_data.bindings.push(binding);
        }
    }

    pub fn unbind(&mut self, action: &InputAction, binding: &InputBinding) {
        if let Some(action_data) = self.actions.get_mut(action) {
            action_data.bindings.retain(|b| b != binding);
        }
    }

    pub fn update(&mut self, ctx: &egui::Context) {
        for action_data in self.actions.values_mut() {
            let previous_state = action_data.state;

            let mut is_down = false;
            let mut was_pressed = false;
            let mut was_released = false;
            let mut wheel_delta = egui::Vec2::ZERO;

            ctx.input(|input| {
                for binding in &action_data.bindings {
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
            });

            action_data.value = wheel_delta;

            let wheel_moved = wheel_delta != egui::Vec2::ZERO;

            action_data.state = if wheel_moved || was_pressed {
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
        }
    }

    pub fn get_bindings(&self, action: &InputAction) -> Option<&[InputBinding]> {
        self.actions
            .get(action)
            .map(|data| data.bindings.as_slice())
    }

    pub fn action_value(&self, action: &InputAction) -> egui::Vec2 {
        self.actions
            .get(action)
            .map(|data| data.value)
            .unwrap_or(egui::Vec2::ZERO)
    }

    pub fn action_state(&self, action: &InputAction) -> InputState {
        self.actions
            .get(action)
            .map(|data| data.state)
            .unwrap_or(InputState::Idle)
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

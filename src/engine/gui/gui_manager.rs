use std::cell::RefCell;

use glfw::Action;

use crate::engine::{
    model::model::Model, texture::resource_manager::ResourceManager, window_handler::WindowHandler,
};

use super::{game_gui::GameGui, gui::Gui, listener::Listener};

pub enum GuiState {
    Game,
}

pub struct GuiManager {
    current_state: GuiState,
    game_gui: GameGui,
}

impl GuiManager {
    pub fn new() -> Self {
        GuiManager {
            current_state: GuiState::Game,
            // Could create later so that it is not constantly loaded into memory
            game_gui: GameGui::create(),
        }
    }

    pub fn set_state(&mut self, state: GuiState) {
        self.current_state = state;
    }

    pub fn get_elements(&self) -> &Vec<Model> {
        match self.current_state {
            GuiState::Game => self.game_gui.get_elements(),
        }
    }

    pub fn get_listeners_mut(&mut self) -> &mut Vec<Listener> {
        match self.current_state {
            GuiState::Game => self.game_gui.get_listeners_mut(),
        }
    }

    pub fn build(&mut self, resource_manager: &ResourceManager, window_handler: &WindowHandler) {
        match self.current_state {
            GuiState::Game => self.game_gui.build(resource_manager, window_handler),
        }
    }

    pub fn update(
        &mut self,
        resource_manager: &ResourceManager,
        window_handler: &mut WindowHandler,
    ) {
        let dimensions = window_handler.get_dimensions();
        let input_handler = window_handler.get_input_handler_mut();
        let mouse_pos = input_handler.get_normalized_mouse_position(dimensions);
        let click_state = input_handler.handle_l_mouse();
        // Reverse iteration for hierarchy
        let mut rebuild = false;
        for listener in self.get_listeners_mut().iter_mut().rev() {
            if listener.bottom_left.x <= mouse_pos.x
                && mouse_pos.x <= listener.top_right.x
                && listener.top_right.y <= mouse_pos.y
                && mouse_pos.y <= listener.bottom_left.y
            {
                let mut callback = RefCell::borrow_mut(&mut listener.fun);
                if let Some(Action::Release) = click_state {
                    rebuild = rebuild || (*callback)();
                }
                break;
            }
        }
        if rebuild {
            self.build(resource_manager, window_handler);
        }
    }
}

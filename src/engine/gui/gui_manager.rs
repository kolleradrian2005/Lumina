use std::sync::Arc;

use crate::engine::{
    command_queue::CommandQueue, input_handler::InputHandler, model::model::Model,
    texture::resource_manager::ResourceManager,
};

use super::{game_gui::GameGui, gui::Gui, listener::Listener};

pub enum GuiState {
    Game,
}

pub struct GuiManager {
    current_state: GuiState,
    game_gui: GameGui,
    dimensions: (i32, i32),
}

impl GuiManager {
    pub fn new(dimensions: (i32, i32)) -> Self {
        GuiManager {
            current_state: GuiState::Game,
            // Could create later so that it is not constantly loaded into memory
            game_gui: GameGui::create(),
            dimensions,
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

    pub fn build(
        &mut self,
        command_queue: Arc<CommandQueue>,
        resource_manager: &ResourceManager,
        aspect_ratio: f32,
    ) {
        match self.current_state {
            GuiState::Game => self
                .game_gui
                .build(command_queue, resource_manager, aspect_ratio),
        }
    }

    pub fn resize(&mut self, dimensions: (i32, i32)) {
        self.dimensions = dimensions;
    }

    pub fn update(
        &mut self,
        command_queue: Arc<CommandQueue>,
        resource_manager: &ResourceManager,
        input_handler: &mut InputHandler,
    ) {
        let mouse_pos = input_handler.get_normalized_mouse_position(self.dimensions);
        let click_state = input_handler.handle_l_mouse();
        // Reverse iteration for hierarchy
        let mut rebuild = false;
        for listener in self.get_listeners_mut().iter_mut().rev() {
            if listener.bottom_left.x <= mouse_pos.x
                && mouse_pos.x <= listener.top_right.x
                && listener.top_right.y <= mouse_pos.y
                && mouse_pos.y <= listener.bottom_left.y
            {
                let mut callback = listener.fun.write().unwrap();

                if let Some(false) = click_state {
                    rebuild = rebuild || (*callback)();
                }
                break;
            }
        }
        if rebuild {
            self.build(
                command_queue,
                resource_manager,
                self.dimensions.0 as f32 / self.dimensions.1 as f32,
            );
        }
    }
}

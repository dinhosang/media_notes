use super::super::helpers;

#[non_exhaustive]
pub struct InputHandlers {
    ctrl_c_handler: fn(),
    escape_handler: fn(),
}

impl Default for InputHandlers {
    fn default() -> Self {
        Self {
            ctrl_c_handler: || helpers::handle_program_exit(),
            escape_handler: || helpers::handle_program_exit(),
        }
    }
}

impl InputHandlers {
    pub fn ctrl_c_handler(&self) {
        (self.ctrl_c_handler)()
    }

    pub fn escape_handler(&self) {
        (self.escape_handler)();
    }
}

pub struct InputHandlersBuilder {
    handlers: InputHandlers,
}

impl Default for InputHandlersBuilder {
    fn default() -> Self {
        InputHandlersBuilder::new()
    }
}

impl InputHandlersBuilder {
    pub fn new() -> Self {
        Self {
            handlers: InputHandlers::default(),
        }
    }

    pub fn build(self) -> InputHandlers {
        self.handlers
    }

    pub fn ctrl_c_handler(&mut self, handler: fn()) {
        self.handlers.ctrl_c_handler = handler;
    }

    pub fn escape_handler(&mut self, handler: fn()) {
        self.handlers.escape_handler = handler;
    }
}

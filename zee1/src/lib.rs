#![no_std]
extern crate alloc;
use alloc::rc::Rc;
use core::cell::RefCell;

////////////////////////////////////////////////////////////////////////////////////////////////////
/// Base trait for sub-engine traits.
////////////////////////////////////////////////////////////////////////////////////////////////////
trait Engine {
    /// Start the engine.
    fn lauch(&self) -> Result<(), ()>;
    /// Shutdown the engine.
    fn shutdown(&self) -> Result<(), ()>;
    /// Should be called in game loop to update the engine's state.
    fn update(&mut self, app: &dyn AppBase) -> Result<(), ()>;
}

trait VideoEngine: Engine {}
trait AudioEngine: Engine {}
trait ControlEngine: Engine {}
trait ScriptingEngine: Engine {}
trait AppBase {
    fn run(&mut self) -> Result<(), ()>;
    fn stop(&mut self);
}

////////////////////////////////////////////////////////////////////////////////////////////////////
/// Root for sub-engines.
////////////////////////////////////////////////////////////////////////////////////////////////////
struct App {
    video_engine: Rc<RefCell<dyn VideoEngine>>,
    audio_engine: Rc<RefCell<dyn AudioEngine>>,
    scripting_engine: Rc<RefCell<dyn ScriptingEngine>>,
    control_engine: Rc<RefCell<dyn ControlEngine>>,
    is_running: bool,
}

impl AppBase for App {
    ////////////////////////////////////////////////////////////////////////////////////////////////
    /// Start the game loop.
    ////////////////////////////////////////////////////////////////////////////////////////////////
    fn run(&mut self) -> Result<(), ()> {
        //------------------------------------------------------------------------------------------
        // Game loop
        // Update every sub-engines and forward errors if they should happen.
        //------------------------------------------------------------------------------------------
        while self.is_running {
            self.scripting_engine.borrow_mut().update(self)?;
            self.video_engine.borrow_mut().update(self)?;
            self.audio_engine.borrow_mut().update(self)?;
            self.control_engine.borrow_mut().update(self)?;
        }
        //------------------------------------------------------------------------------------------
        // Game ran and quitted without any problems.
        //------------------------------------------------------------------------------------------
        Ok(())
    }

    ////////////////////////////////////////////////////////////////////////////////////////////////
    /// Stop the game loop.
    ////////////////////////////////////////////////////////////////////////////////////////////////
    fn stop(&mut self) {
        self.is_running = false;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

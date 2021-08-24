// Copyright (c) 2021 Hai Vu
//
// This software is provided 'as-is', without any express or implied
// warranty.  In no event will the authors be held liable for any damages
// arising from the use of this software.
//
// Permission is granted to anyone to use this software for any purpose,
// including commercial applications, and to alter it and redistribute it
// freely, subject to the following restrictions:
//
// 1. The origin of this software must not be misrepresented; you must not
//    claim that you wrote the original software. If you use this software
//    in a product, an acknowledgment in the product documentation would be
//    appreciated but is not required.
// 2. Altered source versions must be plainly marked as such, and must not be
//    misrepresented as being the original software.
// 3. This notice may not be removed or altered from any source distribution.

#![no_std]

extern crate alloc;
use alloc::boxed::Box;
use alloc::rc::Rc;
use core::cell::RefCell;

//--------------------------------------------------------------------------------------------------
// "mockall" and "std" is only used for testing.
//--------------------------------------------------------------------------------------------------
#[cfg(test)]
use mockall::{automock, predicate::*};
#[cfg(test)]
extern crate std;

////////////////////////////////////////////////////////////////////////////////////////////////////
/// Base trait for sub-engine traits.
////////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg_attr(test, automock)]
trait Engine {
    ////////////////////////////////////////////////////////////////////////////////////////////////
    /// Start the engine.
    ////////////////////////////////////////////////////////////////////////////////////////////////
    fn lauch(&self) -> Result<(), ()>;

    ////////////////////////////////////////////////////////////////////////////////////////////////
    /// Shutdown the engine.
    ////////////////////////////////////////////////////////////////////////////////////////////////
    fn shutdown(&self) -> Result<(), ()>;

    ////////////////////////////////////////////////////////////////////////////////////////////////
    /// Should be called in game loop to update the engine's state.
    ////////////////////////////////////////////////////////////////////////////////////////////////
    fn update(&mut self, app: &mut dyn RootBase) -> Result<(), ()>;
}

////////////////////////////////////////////////////////////////////////////////////////////////////
/// Trait for sub-engine that handles rendering and displaying graphics.
////////////////////////////////////////////////////////////////////////////////////////////////////
trait VideoEngine: Engine {}

////////////////////////////////////////////////////////////////////////////////////////////////////
/// Trait for sub-engine that plays audio.
////////////////////////////////////////////////////////////////////////////////////////////////////
trait AudioEngine: Engine {}

////////////////////////////////////////////////////////////////////////////////////////////////////
/// Trait for sub-engine that handles keyboard, mouse and gamepad... input.
////////////////////////////////////////////////////////////////////////////////////////////////////
trait ControlEngine: Engine {}

////////////////////////////////////////////////////////////////////////////////////////////////////
/// Trait for sub-engine that handles game scripting.
////////////////////////////////////////////////////////////////////////////////////////////////////
trait ScriptingEngine: Engine {}

////////////////////////////////////////////////////////////////////////////////////////////////////
/// Trait for root for sub-engines.
////////////////////////////////////////////////////////////////////////////////////////////////////
trait RootBase {
    ////////////////////////////////////////////////////////////////////////////////////////////////
    /// Start the game loop.
    ////////////////////////////////////////////////////////////////////////////////////////////////
    fn run(&mut self) -> Result<(), ()>;

    ////////////////////////////////////////////////////////////////////////////////////////////////
    /// Stop the game loop.
    ////////////////////////////////////////////////////////////////////////////////////////////////
    fn stop(&mut self);
}

////////////////////////////////////////////////////////////////////////////////////////////////////
/// Root for sub-engines.
////////////////////////////////////////////////////////////////////////////////////////////////////
struct Root {
    video_engine: Rc<RefCell<Box<dyn VideoEngine>>>,
    audio_engine: Rc<RefCell<Box<dyn AudioEngine>>>,
    scripting_engine: Rc<RefCell<Box<dyn ScriptingEngine>>>,
    control_engine: Rc<RefCell<Box<dyn ControlEngine>>>,
    /// Is the game running?
    is_running: bool,
}

impl RootBase for Root {
    fn run(&mut self) -> Result<(), ()> {
        //------------------------------------------------------------------------------------------
        // Start all the sub-engines.
        //------------------------------------------------------------------------------------------
        self.start_engines()?;

        //------------------------------------------------------------------------------------------
        // Game loop.
        // Update every sub-engines and forward errors if they should happen.
        //------------------------------------------------------------------------------------------
        while self.is_running {
            //--------------------------------------------------------------------------------------
            // Update scripting engine.
            //--------------------------------------------------------------------------------------
            self.scripting_engine
                .clone()
                .as_ref()
                .borrow_mut()
                .update(self)?;

            //--------------------------------------------------------------------------------------
            // Update video engine.
            //--------------------------------------------------------------------------------------
            self.video_engine
                .clone()
                .as_ref()
                .borrow_mut()
                .update(self)?;

            //--------------------------------------------------------------------------------------
            // Update video engine.
            //--------------------------------------------------------------------------------------
            self.audio_engine
                .clone()
                .as_ref()
                .borrow_mut()
                .update(self)?;

            //--------------------------------------------------------------------------------------
            // Update control engine.
            //--------------------------------------------------------------------------------------
            self.control_engine
                .clone()
                .as_ref()
                .borrow_mut()
                .update(self)?;
        }

        //------------------------------------------------------------------------------------------
        // Game ran and exited on purpose without any problems.
        //------------------------------------------------------------------------------------------
        Ok(())
    }

    fn stop(&mut self) {
        //------------------------------------------------------------------------------------------
        //  Just changing is_running to false is enough to stop the game loop.
        //------------------------------------------------------------------------------------------
        self.is_running = false;
    }
}

impl Root {
    ////////////////////////////////////////////////////////////////////////////////////////////////
    /// Start all sub-engines.
    ////////////////////////////////////////////////////////////////////////////////////////////////
    fn start_engines(&mut self) -> Result<(), ()> {
        //------------------------------------------------------------------------------------------
        // Start all sub-engines and forward errors.
        //------------------------------------------------------------------------------------------
        self.scripting_engine.as_ref().borrow_mut().lauch()?;
        self.control_engine.as_ref().borrow_mut().lauch()?;
        self.audio_engine.as_ref().borrow_mut().lauch()?;
        self.video_engine.as_ref().borrow_mut().lauch()?;

        //------------------------------------------------------------------------------------------
        // All engines are started successfully.
        //------------------------------------------------------------------------------------------
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn works() {
        let mut mock = MockEngine::new();
        mock.expect_lauch().return_const(Err(()));
        assert!(mock.lauch().is_err());
    }
}

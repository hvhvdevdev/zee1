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
extern crate common;
extern crate spin;

use alloc::boxed::Box;
use alloc::sync::Arc;
use core::fmt::Debug;
use spin::mutex::*;

//--------------------------------------------------------------------------------------------------
// "mockall" and "std" is only used for testing.
//--------------------------------------------------------------------------------------------------
#[cfg(test)]
use mockall::{mock, predicate::*};
#[cfg(test)]
extern crate rspec;
#[cfg(test)]
extern crate std;

////////////////////////////////////////////////////////////////////////////////////////////////////
/// Base trait for engine traits.
////////////////////////////////////////////////////////////////////////////////////////////////////
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
/// Trait for engine that handles rendering and displaying graphics.
////////////////////////////////////////////////////////////////////////////////////////////////////
trait VideoEngine: Engine {}

////////////////////////////////////////////////////////////////////////////////////////////////////
/// Trait for engine that plays audio.
////////////////////////////////////////////////////////////////////////////////////////////////////
trait AudioEngine: Engine {}

////////////////////////////////////////////////////////////////////////////////////////////////////
/// Trait for engine that handles keyboard, mouse and gamepad... input.
////////////////////////////////////////////////////////////////////////////////////////////////////
trait ControlEngine: Engine {}

////////////////////////////////////////////////////////////////////////////////////////////////////
/// Trait for engine that handles game scripting.
////////////////////////////////////////////////////////////////////////////////////////////////////
trait ScriptEngine: Engine {}

////////////////////////////////////////////////////////////////////////////////////////////////////
/// Trait for root for engines.
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
/// Root for engines.
////////////////////////////////////////////////////////////////////////////////////////////////////
struct Root {
    video_engine: Arc<Mutex<Box<dyn VideoEngine + Send>>>,
    audio_engine: Arc<Mutex<Box<dyn AudioEngine + Send>>>,
    script_engine: Arc<Mutex<Box<dyn ScriptEngine + Send>>>,
    control_engine: Arc<Mutex<Box<dyn ControlEngine + Send>>>,
    /// Is the game running?
    is_running: bool,
}

impl Debug for Root {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Root")
            .field("is_running", &self.is_running)
            .finish()
    }
}

impl RootBase for Root {
    fn run(&mut self) -> Result<(), ()> {
        //------------------------------------------------------------------------------------------
        // Start all the engines.
        //------------------------------------------------------------------------------------------
        self.start_engines()?;

        //------------------------------------------------------------------------------------------
        // Start game loop.
        // Update every engines and forward errors if they should happen.
        //------------------------------------------------------------------------------------------
        self.is_running = true;
        while self.is_running {
            self.update_engines()?
        }

        //------------------------------------------------------------------------------------------
        // Loop stopped. we need to shutdown engines gratefully...
        //------------------------------------------------------------------------------------------
        self.shutdown_engines()?;

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
    /// Creates a new root.
    ////////////////////////////////////////////////////////////////////////////////////////////////
    #[allow(dead_code)]
    fn new(
        video_engine: Box<dyn VideoEngine + Send>,
        audio_engine: Box<dyn AudioEngine + Send>,
        script_engine: Box<dyn ScriptEngine + Send>,
        control_engine: Box<dyn ControlEngine + Send>,
    ) -> Root {
        Root {
            video_engine: Arc::new(Mutex::new(video_engine)),
            control_engine: Arc::new(Mutex::new(control_engine)),
            audio_engine: Arc::new(Mutex::new(audio_engine)),
            script_engine: Arc::new(Mutex::new(script_engine)),
            is_running: false,
        }
    }

    ////////////////////////////////////////////////////////////////////////////////////////////////
    /// Start all engines.
    ////////////////////////////////////////////////////////////////////////////////////////////////
    fn start_engines(&mut self) -> Result<(), ()> {
        //------------------------------------------------------------------------------------------
        // Start all engines and forward errors.
        //------------------------------------------------------------------------------------------
        self.script_engine.clone().lock().lauch()?;
        self.control_engine.clone().lock().lauch()?;
        self.audio_engine.clone().lock().lauch()?;
        self.video_engine.clone().lock().lauch()?;

        //------------------------------------------------------------------------------------------
        // All engines are started successfully.
        //------------------------------------------------------------------------------------------
        Ok(())
    }

    ////////////////////////////////////////////////////////////////////////////////////////////////
    /// Update all engines.
    ////////////////////////////////////////////////////////////////////////////////////////////////
    fn update_engines(&mut self) -> Result<(), ()> {
        //------------------------------------------------------------------------------------------
        // Update scripting engine.
        //------------------------------------------------------------------------------------------
        self.script_engine.clone().lock().update(self)?;

        //------------------------------------------------------------------------------------------
        // Update video engine.
        //------------------------------------------------------------------------------------------
        self.audio_engine.clone().lock().update(self)?;

        //------------------------------------------------------------------------------------------
        // Update control engine.
        //------------------------------------------------------------------------------------------
        self.control_engine.clone().lock().update(self)?;

        //------------------------------------------------------------------------------------------
        // Update video engine.
        //------------------------------------------------------------------------------------------
        self.video_engine.clone().lock().update(self)
    }

    ////////////////////////////////////////////////////////////////////////////////////////////////
    /// Shutdown all engines.
    ////////////////////////////////////////////////////////////////////////////////////////////////
    fn shutdown_engines(&mut self) -> Result<(), ()> {
        self.audio_engine.clone().lock().shutdown()?;
        self.control_engine.clone().lock().shutdown()?;
        self.script_engine.clone().lock().shutdown()?;
        self.video_engine.clone().lock().shutdown()?;

        //------------------------------------------------------------------------------------------
        // No error? Ok.
        //------------------------------------------------------------------------------------------
        Ok(())
    }
}

#[cfg(test)]
mod tests;

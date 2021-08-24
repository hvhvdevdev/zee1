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
/// Base trait for sub-engine traits.
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
trait ScriptEngine: Engine {}

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
        // Start all the sub-engines.
        //------------------------------------------------------------------------------------------
        self.start_engines()?;

        //------------------------------------------------------------------------------------------
        // Start game loop.
        // Update every sub-engines and forward errors if they should happen.
        //------------------------------------------------------------------------------------------
        self.is_running = true;
        while self.is_running {
            //--------------------------------------------------------------------------------------
            // Update scripting engine.
            //--------------------------------------------------------------------------------------
            self.script_engine.clone().lock().update(self)?;

            //--------------------------------------------------------------------------------------
            // Update video engine.
            //--------------------------------------------------------------------------------------
            self.video_engine.clone().lock().update(self)?;

            //--------------------------------------------------------------------------------------
            // Update video engine.
            //--------------------------------------------------------------------------------------
            self.audio_engine.clone().lock().update(self)?;

            //--------------------------------------------------------------------------------------
            // Update control engine.
            //--------------------------------------------------------------------------------------
            self.control_engine.clone().lock().update(self)?;
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
    /// Start all sub-engines.
    ////////////////////////////////////////////////////////////////////////////////////////////////
    fn start_engines(&mut self) -> Result<(), ()> {
        //------------------------------------------------------------------------------------------
        // Start all sub-engines and forward errors.
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    //----------------------------------------------------------------------------------------------
    // Mock preparation for video engine.
    //----------------------------------------------------------------------------------------------
    mock! {
        VideoEngineImpl  {}
        impl Engine for VideoEngineImpl {
            fn lauch(&self) -> Result<(), ()>;
            fn shutdown(&self) -> Result<(), ()>;
            fn update(&mut self, app: &mut dyn RootBase) -> Result<(), ()>;
        }
        impl VideoEngine for VideoEngineImpl {}
    }

    //----------------------------------------------------------------------------------------------
    // Mock preparation for audio engine.
    //----------------------------------------------------------------------------------------------
    mock! {
        AudioEngineImpl  {}
        impl Engine for AudioEngineImpl {
            fn lauch(&self) -> Result<(), ()>;
            fn shutdown(&self) -> Result<(), ()>;
            fn update(&mut self, app: &mut dyn RootBase) -> Result<(), ()>;
        }
        impl AudioEngine for AudioEngineImpl {}
    }

    //----------------------------------------------------------------------------------------------
    // Mock preparation for control engine.
    //----------------------------------------------------------------------------------------------
    mock! {
        ControlEngineImpl  {}
        impl Engine for ControlEngineImpl {
            fn lauch(&self) -> Result<(), ()>;
            fn shutdown(&self) -> Result<(), ()>;
            fn update(&mut self, app: &mut dyn RootBase) -> Result<(), ()>;
        }
        impl ControlEngine for ControlEngineImpl {}
    }

    //----------------------------------------------------------------------------------------------
    // Mock preparation for scripting engine.
    //----------------------------------------------------------------------------------------------
    mock! {
        ScriptEngineImpl  {}
        impl Engine for ScriptEngineImpl {
            fn lauch(&self) -> Result<(), ()>;
            fn shutdown(&self) -> Result<(), ()>;
            fn update(&mut self, app: &mut dyn RootBase) -> Result<(), ()>;
        }
        impl ScriptEngine for ScriptEngineImpl {}
    }

    #[test]
    fn bdd() {
        //------------------------------------------------------------------------------------------
        // Environment for rspec.
        //------------------------------------------------------------------------------------------
        #[derive(Debug, Clone)]
        struct Environment {
            root: Arc<Mutex<Box<Root>>>,
            result: Option<Result<(), ()>>,
        }

        impl Default for Environment {
            fn default() -> Self {
                //----------------------------------------------------------------------------------
                // Setup mock objects.
                //----------------------------------------------------------------------------------
                let mut video_engine = MockVideoEngineImpl::new();
                let mut audio_engine = MockAudioEngineImpl::new();
                let mut control_engine = MockControlEngineImpl::new();
                let mut script_engine = MockScriptEngineImpl::new();

                //----------------------------------------------------------------------------------
                // They should always launch successfully.
                //----------------------------------------------------------------------------------
                video_engine.expect_lauch().return_const(Ok(()));
                audio_engine.expect_lauch().return_const(Ok(()));
                control_engine.expect_lauch().return_const(Ok(()));
                script_engine.expect_lauch().return_const(Ok(()));

                //----------------------------------------------------------------------------------
                // Construct that default Enviroment.
                //----------------------------------------------------------------------------------
                Self {
                    root: Arc::new(Mutex::new(Box::new(Root::new(
                        Box::new(video_engine),
                        Box::new(audio_engine),
                        Box::new(script_engine),
                        Box::new(control_engine),
                    )))),
                    result: None,
                }
            }
        }

        //------------------------------------------------------------------------------------------
        // Run BDD tests.
        //------------------------------------------------------------------------------------------
        rspec::run(&rspec::given("An engine", Environment::default(), |ctx| {
            ctx.when("Just start the root with working sub-engines.", |ctx| {
                ctx.before_all(|env| {
                    env.result = Some(env.root.clone().lock().start_engines());
                });

                ctx.then("There is no error.", |env| {
                    assert!(env.result.unwrap().is_ok());
                })
            });

            ctx.when("Start the root with broken video engine...", |ctx| {
                ctx.before_all(|env| {
                    //------------------------------------------------------------------------------
                    // Create a mock VideoEngine that will fail to launch.
                    //------------------------------------------------------------------------------
                    let mut video_engine = MockVideoEngineImpl::new();
                    video_engine.expect_lauch().return_const(Err(()));
                    let video_engine: Box<dyn VideoEngine + Send> = Box::new(video_engine);
                    let video_engine = Arc::new(Mutex::new(video_engine));

                    //------------------------------------------------------------------------------
                    // Swap it in.
                    //------------------------------------------------------------------------------
                    env.root.lock().video_engine = video_engine;

                    env.result = Some(env.root.lock().start_engines());
                });

                ctx.then("It should fail.", |env| {
                    assert!(env.result.unwrap().is_err());
                })
            });
        }))
    }
}

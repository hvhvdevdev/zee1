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

use super::*;
use std::sync::Arc;

//--------------------------------------------------------------------------------------------------
// Mock preparation for video engine.
//--------------------------------------------------------------------------------------------------
mock! {
    VideoEngineImpl  {}
    impl Engine for VideoEngineImpl {
        fn lauch(&self) -> Result<(), ()>;
        fn shutdown(&self) -> Result<(), ()>;
        fn update(&mut self, app: &mut dyn RootBase) -> Result<(), ()>;
    }
    impl VideoEngine for VideoEngineImpl {}
}

//--------------------------------------------------------------------------------------------------
// Mock preparation for audio engine.
//--------------------------------------------------------------------------------------------------
mock! {
    AudioEngineImpl  {}
    impl Engine for AudioEngineImpl {
        fn lauch(&self) -> Result<(), ()>;
        fn shutdown(&self) -> Result<(), ()>;
        fn update(&mut self, app: &mut dyn RootBase) -> Result<(), ()>;
    }
    impl AudioEngine for AudioEngineImpl {}
}

//--------------------------------------------------------------------------------------------------
// Mock preparation for control engine.
//--------------------------------------------------------------------------------------------------
mock! {
    ControlEngineImpl  {}
    impl Engine for ControlEngineImpl {
        fn lauch(&self) -> Result<(), ()>;
        fn shutdown(&self) -> Result<(), ()>;
        fn update(&mut self, app: &mut dyn RootBase) -> Result<(), ()>;
    }
    impl ControlEngine for ControlEngineImpl {}
}

//--------------------------------------------------------------------------------------------------
// Mock preparation for scripting engine.
//--------------------------------------------------------------------------------------------------
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
    //----------------------------------------------------------------------------------------------
    // Environment for rspec.
    //----------------------------------------------------------------------------------------------
    #[derive(Debug, Clone)]
    struct Environment {
        root: Arc<Mutex<Box<Root>>>,
        result: Option<Result<(), ()>>,
    }

    impl Default for Environment {
        ////////////////////////////////////////////////////////////////////////////////////////////
        /// Construct a default Enviroment for testing.
        /// All engines in root will always launch and update successfully.
        ////////////////////////////////////////////////////////////////////////////////////////////
        fn default() -> Self {
            //--------------------------------------------------------------------------------------
            // Setup mock objects.
            //--------------------------------------------------------------------------------------
            let mut video_engine = MockVideoEngineImpl::new();
            let mut audio_engine = MockAudioEngineImpl::new();
            let mut control_engine = MockControlEngineImpl::new();
            let mut script_engine = MockScriptEngineImpl::new();

            //--------------------------------------------------------------------------------------
            // They should always launch successfully.
            //--------------------------------------------------------------------------------------
            video_engine.expect_lauch().return_const(Ok(()));
            audio_engine.expect_lauch().return_const(Ok(()));
            control_engine.expect_lauch().return_const(Ok(()));
            script_engine.expect_lauch().return_const(Ok(()));

            //--------------------------------------------------------------------------------------
            // They should always update successfully.
            //--------------------------------------------------------------------------------------
            video_engine.expect_update().return_const(Ok(()));
            audio_engine.expect_update().return_const(Ok(()));
            control_engine.expect_update().return_const(Ok(()));
            script_engine.expect_update().return_const(Ok(()));

            //--------------------------------------------------------------------------------------
            // Also shutdown peacefully...
            //--------------------------------------------------------------------------------------
            video_engine.expect_shutdown().return_const(Ok(()));
            audio_engine.expect_shutdown().return_const(Ok(()));
            control_engine.expect_shutdown().return_const(Ok(()));
            script_engine.expect_shutdown().return_const(Ok(()));

            //--------------------------------------------------------------------------------------
            // Construct that default Enviroment.
            //--------------------------------------------------------------------------------------
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

    //----------------------------------------------------------------------------------------------
    // Run BDD tests.
    //----------------------------------------------------------------------------------------------
    rspec::run(&rspec::given("An engine", Environment::default(), |ctx| {
        ctx.when("Just start the root with working sub-engines.", |ctx| {
            ctx.before_all(|env| {
                //----------------------------------------------------------------------------------
                // Start engines and save result.
                //----------------------------------------------------------------------------------
                env.result = Some(env.root.clone().lock().start_engines());
            });

            ctx.then("There is no error.", |env| {
                assert!(env.result.unwrap().is_ok());
            });

            ctx.when("Update all engines", |ctx| {
                ctx.before_all(|env| env.result = Some(env.root.clone().lock().update_engines()));

                ctx.then("There is no error.", |env| {
                    assert!(env.result.unwrap().is_ok());
                });
            });

            ctx.when("Shutdown all engines", |ctx| {
                ctx.before_all(|env| env.result = Some(env.root.clone().lock().shutdown_engines()));

                ctx.then("There is no error.", |env| {
                    assert!(env.result.unwrap().is_ok());
                });
            });
        });

        ctx.when("Start the root with broken video engine...", |ctx| {
            ctx.before_all(|env| {
                //----------------------------------------------------------------------------------
                // Create a mock VideoEngine that will fail to launch, shutdown and update.
                //----------------------------------------------------------------------------------
                let mut video_engine = MockVideoEngineImpl::new();
                video_engine.expect_lauch().return_const(Err(()));
                video_engine.expect_update().return_const(Err(()));
                video_engine.expect_shutdown().return_const(Err(()));

                //----------------------------------------------------------------------------------
                // Wrap it in a Box then Mutex then Arc.
                //----------------------------------------------------------------------------------
                let video_engine: Box<dyn VideoEngine + Send> = Box::new(video_engine);
                let video_engine = Arc::new(Mutex::new(video_engine));

                //----------------------------------------------------------------------------------
                // Swap it into root.
                //----------------------------------------------------------------------------------
                env.root.lock().video_engine = video_engine;

                //----------------------------------------------------------------------------------
                // Start engines and save result.
                //----------------------------------------------------------------------------------
                env.result = Some(env.root.lock().start_engines());
            });

            ctx.then("It should fail.", |env| {
                assert!(env.result.unwrap().is_err());
            });

            ctx.when("Update all engines", |ctx| {
                ctx.before_all(|env| env.result = Some(env.root.clone().lock().update_engines()));

                ctx.then("It should fail.", |env| {
                    assert!(env.result.unwrap().is_err());
                });
            });

            ctx.when("Shutdown all engines", |ctx| {
                ctx.before_all(|env| env.result = Some(env.root.clone().lock().shutdown_engines()));

                ctx.then("It should fail.", |env| {
                    assert!(env.result.unwrap().is_err());
                });
            });
        });

        //------------------------------------------------------------------------------------------
        // Running mean starting, updating until quit, and shuting down...
        //------------------------------------------------------------------------------------------
        ctx.when("Run root with working engines...", |ctx| {
            ctx.before_all(|env| {
                //----------------------------------------------------------------------------------
                // We need video engine to ask the root to stop after a while.
                // Otherwise, it will loop forever...
                //----------------------------------------------------------------------------------
                let mut video_engine = MockVideoEngineImpl::new();
                video_engine.expect_lauch().return_const(Ok(()));
                video_engine.expect_update().times(10).return_const(Ok(()));
                video_engine.expect_update().returning(|root| {
                    root.stop();
                    Ok(())
                });
                video_engine.expect_shutdown().return_const(Ok(()));

                //----------------------------------------------------------------------------------
                // Wrap it in a Box then Mutex then Arc.
                //----------------------------------------------------------------------------------
                let video_engine: Box<dyn VideoEngine + Send> = Box::new(video_engine);
                let video_engine = Arc::new(Mutex::new(video_engine));

                //----------------------------------------------------------------------------------
                // Swap it into root.
                //----------------------------------------------------------------------------------
                env.root.lock().video_engine = video_engine;

                env.result = Some(env.root.clone().lock().run())
            });

            ctx.then("Run and stop successfully.", |env| {
                assert!(env.result.unwrap().is_ok())
            })
        })
    }));
}

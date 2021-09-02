// ┌──────────────────────────────────────────────────────────────────────────────┐
// │                                                                              │
// │ This Source Code Form is subject to the terms of the Mozilla Public          │
// │                                                                              │
// │ License, v. 2.0. If a copy of the MPL was not distributed with this          │
// │                                                                              │
// │ file, You can obtain one at https://mozilla.org/MPL/2.0/.                    │
// │                                                                              │
// └──────────────────────────────────────────────────────────────────────────────┘

//
// ────────────────────────────────────────────────────── I ──────────
//   :::::: I M P O R T S : :  :   :    :     :        :          :
// ────────────────────────────────────────────────────────────────
//

use super::super::Service;
use super::OpenGlDriver;

//
// ────────────────────────────────────────────────────── II ──────────
//   :::::: S T R U C T S : :  :   :    :     :        :          :
// ────────────────────────────────────────────────────────────────
//

//
// ─── STRUCT SDL VIDEO ───────────────────────────────────────────────────────────
//

#[derive(new)]
pub struct SdlVideo {}

//
// ─── STRUCT SDL OPENGL DRIVER ───────────────────────────────────────────────────
//

#[derive(new)]
pub struct SdlOpenGlDriver {}

//
// ────────────────────────────────────────────────────────────────────── III ──────────
//   :::::: I M P L E M E N T A T I O N S : :  :   :    :     :        :          :
// ────────────────────────────────────────────────────────────────────────────────
//

//
// ─── IMPLEMENTS SERVICE FOR SDL VIDEO ───────────────────────────────────────────
//

impl Service for SdlVideo {
    fn start(&self) -> Result<(), alloc::string::String> {
        extern "C" {
            fn Zee1_InitVideo() -> bool;
        }

        unsafe { Zee1_InitVideo() };

        Ok(())
    }

    fn stop(&self) -> Result<(), alloc::string::String> {
        todo!()
    }

    fn update(&self, delta: f32) -> Result<(), alloc::string::String> {
        todo!()
    }
}

//
// ─── IMPLEMENTS OPENGL DRIVER FOR SDL OPENGL DRIVER ─────────────────────────────
//

impl OpenGlDriver for SdlOpenGlDriver {}

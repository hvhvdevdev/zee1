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
// ─── DISABLE STANDARD LIBRARY ───────────────────────────────────────────────────
//

#![no_std]

//
// ────────────────────────────────────────────────────── I ──────────
//   :::::: M O D U L E S : :  :   :    :     :        :          :
// ────────────────────────────────────────────────────────────────
//

//
// ─── AUDIO SERVICE ──────────────────────────────────────────────────────────────
//

pub mod audio;

//
// ─── INPUT SERVICE ──────────────────────────────────────────────────────────────
//

pub mod input;

//
// ─── SCRIPTING SERVICE ──────────────────────────────────────────────────────────
//

pub mod script;

//
// ─── GRAPHICS SERVICE ───────────────────────────────────────────────────────────
//

pub mod video;

//
// ────────────────────────────────────────────────────── II ──────────
//   :::::: I M P O R T S : :  :   :    :     :        :          :
// ────────────────────────────────────────────────────────────────
//

extern crate alloc;

use alloc::string::String;

//
// ──────────────────────────────────────────────────── III ──────────
//   :::::: T R A I T S : :  :   :    :     :        :          :
// ──────────────────────────────────────────────────────────────
//

//
// ─── TRAIT CORE ─────────────────────────────────────────────────────────────────
//

trait Core {
    //
    // ─── START THE CORE ─────────────────────────────────────────────────────────────
    //

    fn start() -> Result<(), String>;
}

//
// ─── TRAIT GAME ─────────────────────────────────────────────────────────────────
//

trait Game {
    fn play() -> Result<(), String>;
}

//
// ────────────────────────────────────────────────────── IV ──────────
//   :::::: S T R U C T S : :  :   :    :     :        :          :
// ────────────────────────────────────────────────────────────────
//

//
// ─── STRUCT CORE IMPL ───────────────────────────────────────────────────────────
//

struct CoreImpl {}

//
// ────────────────────────────────────────────────────────────────────── V ──────────
//   :::::: I M P L E M E N T A T I O N S : :  :   :    :     :        :          :
// ────────────────────────────────────────────────────────────────────────────────
//

//
// ─── IMPLEMENTS CORE FOR CORE IMPL ──────────────────────────────────────────────
//

impl Core for CoreImpl {
    fn start() -> Result<(), String> {
        todo!()
    }
}

//
// ─── UNIT TESTS ─────────────────────────────────────────────────────────────────
//

#[cfg(test)]
mod tests;

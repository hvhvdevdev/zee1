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

//! Module for reading command line arguments.

use common::config::RunConfig;

////////////////////////////////////////////////////////////////////////////////////////////////////
/// Struct for parsing command line arguments.
////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct CmdLineConfigReader {
    args: Vec<String>,
}

impl CmdLineConfigReader {
    ////////////////////////////////////////////////////////////////////////////////////////////////////
    /// Constructs a new `CmdLineConfigReader.
    ////////////////////////////////////////////////////////////////////////////////////////////////////
    pub fn new(args: Vec<String>) -> CmdLineConfigReader {
        CmdLineConfigReader { args }
    }

    ////////////////////////////////////////////////////////////////////////////////////////////////////
    /// Reads the command line arguments.
    ////////////////////////////////////////////////////////////////////////////////////////////////////
    pub fn read(&self) -> Result<RunConfig, String> {
        let mut window_width = 640u32;
        let mut window_height = 480u32;
        let mut mod_root = String::from("mods/zeroheat");

        //------------------------------------------------------------------------------------------
        // Loop through each command line argument.
        // Start with 1 because the first argument is the executable path.
        // Do not forget to increment i by one in some cases we need to look ahead for a value.
        //------------------------------------------------------------------------------------------
        let mut i = 1;
        while i < self.args.len() {
            match &self.args[i][..] {
                //----------------------------------------------------------------------------------
                // Window width
                //----------------------------------------------------------------------------------
                "--width" | "-w" => {
                    window_width = if i + 1 < self.args.len() {
                        self.args[i + 1].parse::<u32>().unwrap_or(1024)
                    } else {
                        return Err(String::from("Unexpected end."));
                    };
                    i += 1
                }

                //----------------------------------------------------------------------------------
                // Window height
                //----------------------------------------------------------------------------------
                "--height" | "-h" => {
                    window_height = if i + 1 < self.args.len() {
                        self.args[i + 1].parse::<u32>().unwrap_or(768)
                    } else {
                        return Err(String::from("Unexpected end."));
                    };
                    i += 1
                }

                //----------------------------------------------------------------------------------
                // Mod root
                //----------------------------------------------------------------------------------
                "--mod" => {
                    mod_root = if i + 1 < self.args.len() {
                        String::from(&self.args[i + 1])
                    } else {
                        return Err(String::from("Unexpected end."));
                    };
                    i += 1;
                }

                //----------------------------------------------------------------------------------
                // Unknown argument.
                //----------------------------------------------------------------------------------
                x => return Err(format!("Unexpected {}", x)),
            }
            i += 1;
        }

        //------------------------------------------------------------------------------------------
        // No problem? Ok
        //------------------------------------------------------------------------------------------
        Ok(RunConfig {
            window_width,
            window_height,
            mod_root,
        })
    }
}

#[cfg(test)]
mod tests {
    ////////////////////////////////////////////////////////////////////////////////////////////////
    /// Simple case.
    /// Just change window width
    ////////////////////////////////////////////////////////////////////////////////////////////////.
    #[test]
    fn simple() {
        let args = vec![String::from(""), String::from("-w"), String::from("1920")];
        let config = super::CmdLineConfigReader::new(args).read().unwrap();
        assert_eq!(config.window_width, 1920)
    }

    ////////////////////////////////////////////////////////////////////////////////////////////////
    /// Ok when there are many arguments.
    /// Later --width should replace earlier -width/-w
    ////////////////////////////////////////////////////////////////////////////////////////////////
    #[test]
    fn multi() {
        let args = vec![
            String::from(""),
            String::from("-w"),
            String::from("1920"),
            String::from("--width"),
            String::from("1600"),
            String::from("--mod"),
            String::from("mods/abc"),
            String::from("--height"),
            String::from("900"),
        ];
        let config = super::CmdLineConfigReader::new(args).read().unwrap();
        assert_eq!(config.window_width, 1600);
        assert_eq!(config.window_height, 900);
        assert_eq!(config.mod_root, String::from("mods/abc"))
    }

    ////////////////////////////////////////////////////////////////////////////////////////////////
    /// --width is there but no value provided.
    /// Should return Err.
    ////////////////////////////////////////////////////////////////////////////////////////////////
    #[test]
    fn no_width_value() {
        let args = vec![String::from(""), String::from("-w")];
        let config_result = super::CmdLineConfigReader::new(args).read();
        assert!(config_result.is_err());
    }

    ////////////////////////////////////////////////////////////////////////////////////////////////
    /// --height is there but no value provided.
    /// Should return Err.
    ////////////////////////////////////////////////////////////////////////////////////////////////
    #[test]
    fn no_height_value() {
        let args = vec![String::from(""), String::from("-h")];
        let config_result = super::CmdLineConfigReader::new(args).read();
        assert!(config_result.is_err());
    }

    ////////////////////////////////////////////////////////////////////////////////////////////////
    /// --mod is there but no value provided.
    /// Should return Err.
    ////////////////////////////////////////////////////////////////////////////////////////////////
    #[test]
    fn no_mod_value() {
        let args = vec![String::from(""), String::from("--mod")];
        let config_result = super::CmdLineConfigReader::new(args).read();
        assert!(config_result.is_err());
    }

    ////////////////////////////////////////////////////////////////////////////////////////////////
    /// What happens when an unknown argument is passed to the command line?
    /// Should return Err.
    ////////////////////////////////////////////////////////////////////////////////////////////////
    #[test]
    fn unknown() {
        let args = vec![String::from(""), String::from("-unknown")];
        let config_result = super::CmdLineConfigReader::new(args).read();
        assert!(config_result.is_err());
    }
}

// ┌──────────────────────────────────────────────────────────────────────────────┐
// │                                                                              │
// │ This Source Code Form is subject to the terms of the Mozilla Public          │
// │                                                                              │
// │ License, v. 2.0. If a copy of the MPL was not distributed with this          │
// │                                                                              │
// │ file, You can obtain one at https://mozilla.org/MPL/2.0/.                    │
// │                                                                              │
// └──────────────────────────────────────────────────────────────────────────────┘

fn main() {
    let dst = cmake::build("os/posix");

    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    println!("cargo:rustc-link-lib=static=Zee1_Posix");
    println!("cargo:rustc-link-lib=SDL2");
    println!("cargo:rustc-link-lib=GL");
}

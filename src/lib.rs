// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![crate_name = "alert"]
#![crate_type = "rlib"]

#[cfg(target_os="macos")]
extern crate core_foundation;
#[cfg(target_os="macos")]
extern crate cocoa;

#[cfg(target_os="linux")]
pub use linux::Alert;
#[cfg(target_os="macos")]
pub use macos::Alert;
#[cfg(target_os="android")]
pub use android::Alert;

#[cfg(target_os="linux")]
pub mod linux;
#[cfg(target_os="macos")]
pub mod macos;
#[cfg(target_os="android")]
pub mod android;

#[cfg(testalert)]
mod test;

pub trait AlertMethods {
    /// Creates a new alert with an OK and Cancel button.
    fn new(message_text: &str) -> Self;
    /// Adds a prompt to the alert.
    fn add_prompt(&mut self);
    /// Runs the alert modally.
    fn run(&self);
    /// Returns the value of the prompt.
    fn prompt_value(&self) -> String;
}


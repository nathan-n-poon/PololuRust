// Copyright 2021 UBC Bionics, Ltd.
//
// Licensed under the MIT license
// <LICENSE.md or https://opensource.org/licenses/MIT>.
// This file may not be copied, modified, or
// distributed except according to those terms.

// external crates

// external uses
use std::{
    thread,
    time::Duration,
};

use raestro::prelude::*;

// internal mods

// internal uses

fn main() -> () {
    let mut maestro: Maestro = Maestro::new();
    maestro.start(BaudRates::BR_115200).unwrap();

    let channel: Channels = Channels::C_0;

    let speed0 = 10u16;
    let speed1 = 140u16;

    let pos0 = 992u16;
    let pos1 = 2000u16;

    let sleep_time = Duration::from_millis(1000u64);

    loop {
        maestro.set_speed(channel, speed0).unwrap();
        maestro.set_target(channel, pos0).unwrap();
        thread::sleep(sleep_time);

        maestro.set_speed(channel, speed1).unwrap();
        maestro.set_target(channel, pos1).unwrap();
        thread::sleep(sleep_time);
    }
}

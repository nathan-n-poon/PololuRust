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

    let target_min = 3968u16;
    let target_max = 8000u16;

    let sleep_time = Duration::from_millis(1000u64);

    loop {
        maestro.set_speed(channel, speed0).unwrap();
        maestro.set_target(channel, target_min).unwrap();
        thread::sleep(sleep_time);

        maestro.set_speed(channel, speed1).unwrap();
        maestro.set_target(channel, target_max).unwrap();
        thread::sleep(sleep_time);
    }
}

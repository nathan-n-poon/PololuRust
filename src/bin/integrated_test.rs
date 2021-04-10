/* external crates */

/* external uses */
use std::thread;
use raestro::prelude::*;
use std::time::Duration;
// use std::num::Float;
use std::time::Instant;
use std::thread::sleep;

/* internal mods */

/* internal uses */
mod test_util;
use crate::test_util::test_util::within_tolerance;

const TIMEOUT:u32 = 2000;


fn wait(maestro: & mut Maestro, target:u16) -> () {
    let mut timeout_counter:u32 = 0;
    while !within_tolerance(maestro.get_position(Channels::C_0).unwrap(), target, 10) 
    { 
        if timeout_counter > TIMEOUT
        {
            panic!("took too long! \n");
        }
        thread::sleep(Duration::from_millis(100)); 
        timeout_counter += 100;
    }
}

fn main() -> () {
    let mut maestro: Maestro = Maestro::new();
    maestro.start(BaudRates::BR_115200).unwrap();

    let small: u16 = 500u16;
    let big: u16 = 60000u16;
    let sleep_time: u64 = 1000u64;
    let accel:u8 = 100;

    loop {
        //simple position
        maestro.set_target(Channels::C_0, small).unwrap();
        wait(& mut maestro, small);


        thread::sleep(Duration::from_millis(sleep_time));

        maestro.set_target(Channels::C_0, big).unwrap();
        wait(& mut maestro, big);

        thread::sleep(Duration::from_millis(sleep_time));

        //accel test
        maestro.set_target(Channels::C_0, small).unwrap();
        maestro.set_acceleration(Channels::C_0, accel).unwrap();
        wait(&mut maestro, small);
        let d:u16 = big - small;
        let expected_time: f64 = ((2 as f64)*(d as f64)/(accel as f64)).sqrt();
        let now = Instant::now();

        maestro.set_target(Channels::C_0, big).unwrap();
        wait(& mut maestro, big);
        assert!( within_tolerance(now.elapsed().as_millis() as u16, expected_time as u16, 100));

    }
}

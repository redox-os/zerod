extern crate syscall;

use syscall::data::Packet;
use syscall::scheme::Scheme;
use std::fs::File;
use std::io::{Read, Write};
use std::mem::size_of;
use scheme::ZeroScheme;

mod scheme;

fn main() {
    if unsafe { syscall::clone(0).unwrap() } == 0 {
        let mut socket = File::create(":zero").expect("zerod: failed to create zero scheme");
        let scheme = ZeroScheme;

        syscall::setrens(0, 0).expect("zerod: failed to enter null namespace");

        loop {
            let mut packet = Packet::default();
            socket.read(&mut packet).expect("zerod: failed to read events from zero scheme");

            scheme.handle(&mut packet);

            socket.write(&packet).expect("zerod: failed to write responses to zero scheme");
        }
    }
}

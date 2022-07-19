use std::fs::File;
use std::io::{Read, Write};

use syscall::data::Packet;
use syscall::scheme::Scheme;

use scheme::ZeroScheme;

mod scheme;

fn main() {
    redox_daemon::Daemon::new(move |daemon| {
        let mut socket = File::create(":zero").expect("zerod: failed to create zero scheme");
        let scheme = ZeroScheme;

        syscall::setrens(0, 0).expect("zerod: failed to enter null namespace");

        daemon.ready().expect("zerod: failed to notify parent");

        loop {
            let mut packet = Packet::default();
            if socket.read(&mut packet).expect("zerod: failed to read events from zero scheme") == 0 {
                std::process::exit(0);
            }

            scheme.handle(&mut packet);

            socket.write(&packet).expect("zerod: failed to write responses to zero scheme");
        }
    }).expect("zerod: failed to daemonize");
}

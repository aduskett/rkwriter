mod rk_devices;

use clap::{command, Arg, ArgAction, ArgMatches};
use crate::rk_devices::RKDevices;

fn parse_args() -> ArgMatches {
    let matches = command!() // requires `cargo` feature
        .arg(Arg::new("list-devices").short('l').long("list-devices").action(ArgAction::SetTrue))
        .get_matches();

    matches
}


fn main() {
    let args = parse_args();
    if args.get_flag("list-devices") {
        let mut devices = RKDevices::new();
        devices.list_devices();
    }
}

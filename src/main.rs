use std::{env::args, process};
use ysfs::ysf;

fn main() {
    let args: Vec<String> = args().collect();
    if args.len() < 4 {
        eprintln!("{} <callsign> <local_ip_bind> <server>\n{} M0ABC 192.168.0.2:4040 americalink.radiotechnology.xyz:42000", args[0], args[0]);
        process::exit(-1);
    }

    let config = ysfs::Config {
        callsign: args[1].to_string(),
        bind: args[2].to_string(),
        server: args[3].to_string(),
    };

    ysf::run(config);
}

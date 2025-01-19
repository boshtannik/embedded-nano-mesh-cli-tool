use std::time::Instant;

use super::constants;
use clap::Parser;
use embedded_nano_mesh::{ms, ExactAddressType, LifeTimeType, Node, NodeConfig, NodeString};
use embedded_nano_mesh_linux_io::*;

#[derive(Parser, Debug)]
pub struct BroadcastArgs {
    #[clap(
        short = 'f',
        long = "from-address",
        required = true,
        help = constants::FROM_ADDRESS_HELP_MSG
    )]
    pub from_address: ExactAddressType,

    #[clap(
        long = "listen-period",
        required = true,
        help = constants::LISTEN_PERIOD_HELP_MSG
    )]
    pub listen_period: ms,

    #[clap(
        short = 'c',
        long = "content",
        required = true,
        help = constants::SEND_CONTENT_HELP_MSG
    )]
    pub content: String,

    #[clap(short = 'o', long = "timeout", required = true, help = constants::SEND_TIMEOUT_HELP_MSG)]
    pub timeout: ms,

    #[clap(
        short = 'd',
        long = "filter-out-duplication",
        help = constants::FILTER_OUT_DUPLICATION_HELP_MSG
    )]
    pub filter_out_duplication: bool,

    #[clap(
        long = "lifetime",
        required = true,
        help = constants::LIFETIME_HELP_MSG
    )]
    pub lifetime: LifeTimeType,

    #[clap(
        short = 'p',
        long = "port",
        required = true,
        help = constants::PORT_HELP_MSG
    )]
    pub port: String,
}

pub fn process_broadcast(args: BroadcastArgs) {
    let program_start_time = Instant::now();

    let mut node = Node::new(NodeConfig {
        device_address: args.from_address as ExactAddressType,
        listen_period: args.listen_period as ms,
    });

    let mut serial = LinuxIO::new(
        serialport::new("/dev/ttyUSB0", 9600)
            .open_native()
            .expect("Fail to open serial port"),
    );

    let _ = node.broadcast(
        NodeString::from_iter(args.content.chars()).into_bytes(),
        args.lifetime as LifeTimeType,
    );

    let exit_time = program_start_time
        .duration_since(program_start_time)
        .as_millis() as ms
        + args.timeout as ms;

    loop {
        let current_time = Instant::now()
            .duration_since(program_start_time)
            .as_millis() as ms;
        if current_time >= exit_time {
            std::process::exit(0);
        }
        let _ = node.update(&mut serial, current_time);
    }
}

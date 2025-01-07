use std::time::Instant;

use crate::serial_interface::LinuxInterfaceDriver;

use super::constants;
use clap::Parser;
use embedded_nano_mesh::{ms, ExactAddressType, LifeTimeType, Node, NodeConfig, NodeString};

#[derive(Parser, Debug)]
pub struct PingArgs {
    #[clap(short = 'f', long = "from-address", required = true, help = constants::FROM_ADDRESS_HELP_MSG)]
    pub from_address: ExactAddressType,

    #[clap(short = 't', long = "to-address", required = true, help = constants::TO_ADDRESS_HELP_MSG)]
    pub to_address: ExactAddressType,

    #[clap(long = "listen-period", required = true, help = constants::LISTEN_PERIOD_HELP_MSG)]
    pub listen_period: ms,

    #[clap(short = 'c', long = "content", required = true, help = constants::PING_CONTENT_HELP_MSG)]
    pub content: String,

    #[clap(short = 'o', long = "timeout", required = true, help = constants::SPEC_OPERATION_TIMEOUT_HELP_MSG)]
    pub timeout: ms,

    #[clap(long = "lifetime", required = true, help = constants::LIFETIME_HELP_MSG)]
    pub lifetime: LifeTimeType,

    #[clap(short = 'p', long = "port", required = true, help = constants::PORT_HELP_MSG)]
    pub port: String,
}

pub fn process_ping(args: PingArgs) {
    let program_start_time = Instant::now();

    let mut node = Node::new(NodeConfig {
        device_address: args.from_address as ExactAddressType,
        listen_period: args.listen_period as ms,
    });

    let mut serial = LinuxInterfaceDriver::new(
        serialport::new("/dev/ttyUSB0", 9600)
            .open_native()
            .expect("Fail to open serial port"),
    );

    match node.send_ping_pong(
        NodeString::from_iter(args.content.chars()).into_bytes(),
        args.to_address as ExactAddressType,
        args.lifetime as LifeTimeType,
        args.timeout as ms,
        || {
            Instant::now()
                .duration_since(program_start_time)
                .as_millis() as ms
        },
        &mut serial,
    ) {
        Ok(_) => std::process::exit(0),
        Err(_) => std::process::exit(1),
    }
}

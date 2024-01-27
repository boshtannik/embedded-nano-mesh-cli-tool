use super::constants;
use clap::Parser;
use embedded_nano_mesh::{ms, AddressType, LifeTimeType, Node, NodeConfig, NodeString};
use platform_millis_linux::LinuxTime;
use platform_serial_linux::LinuxSerial;

#[derive(Parser, Debug)]
pub struct PingArgs {
    #[clap(
        short = 'f',
        long = "from-address",
        required = true,
        help = constants::FROM_ADDRESS_HELP_MSG
    )]
    pub from_address: AddressType,

    #[clap(short = 't', long = "to-address", required = true, help = constants::TO_ADDRESS_HELP_MSG)]
    pub to_address: AddressType,

    #[clap(
        short = 'l',
        long = "listen-period",
        required = true,
        help = constants::LISTEN_PERIOD_HELP_MSG
    )]
    pub listen_period: ms,

    #[clap(
        short = 'p',
        long = "content",
        required = true,
        help = constants::PING_CONTENT_HELP_MSG
    )]
    pub content: NodeString,

    #[clap(short = 'o', long = "timeout", required = true, help = constants::SPEC_OPERATION_TIMEOUT_HELP_MSG)]
    pub timeout: ms,

    #[clap(
        short = 'd',
        long = "filter-out-duplication",
        help = constants::FILTER_OUT_DUPLICATION_HELP_MSG
    )]
    pub filter_out_duplication: bool,

    #[clap(
        short = 'l',
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

pub fn process_ping(args: PingArgs) {
    let mut node = Node::new(NodeConfig {
        device_address: args.from_address as AddressType,
        listen_period: args.listen_period as ms,
    });

    match node.send_ping_pong::<LinuxTime, LinuxSerial>(
        NodeString::from(args.content.as_str()).into_bytes(),
        args.to_address as AddressType,
        args.lifetime as LifeTimeType,
        args.filter_out_duplication,
        args.timeout as ms,
    ) {
        Ok(_) => std::process::exit(0),
        Err(_) => std::process::exit(1),
    }
}

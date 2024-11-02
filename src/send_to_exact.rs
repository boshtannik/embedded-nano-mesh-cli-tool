use super::constants;
use clap::Parser;
use embedded_nano_mesh::{ms, ExactAddressType, LifeTimeType, Node, NodeConfig, NodeString};
use platform_millis_linux::{LinuxMillis, PlatformMillis};
use platform_serial_linux::LinuxSerial;

#[derive(Parser, Debug)]
pub struct SendToExactArgs {
    #[clap(
        short = 'f',
        long = "from-address",
        required = true,
        help = constants::FROM_ADDRESS_HELP_MSG
    )]
    pub from_address: ExactAddressType,

    #[clap(short = 't', long = "to-address", required = true, help = constants::TO_ADDRESS_HELP_MSG)]
    pub to_address: ExactAddressType,

    #[clap(
        long = "listen-period",
        required = true,
        help = constants::LISTEN_PERIOD_HELP_MSG
    )]
    pub listen_period: ms,

    #[clap(short = 'c', long = "content", required = true, help = constants::SEND_CONTENT_HELP_MSG)]
    pub content: NodeString,

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

pub fn process_send(args: SendToExactArgs) {
    let mut node = Node::new(NodeConfig {
        device_address: args.from_address as ExactAddressType,
        listen_period: args.listen_period as ms,
    });

    let _ = node.send_to_exact(
        NodeString::from(args.content.as_str()).into_bytes(),
        args.to_address as ExactAddressType,
        args.lifetime as LifeTimeType,
        args.filter_out_duplication,
    );

    let exit_time = LinuxMillis::millis() + args.timeout as ms;

    loop {
        let current_time = LinuxMillis::millis();
        if current_time >= exit_time {
            std::process::exit(0);
        }
        let _ = node.update::<LinuxMillis, LinuxSerial>();
    }
}

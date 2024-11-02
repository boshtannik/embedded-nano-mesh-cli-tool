use super::constants;
use clap::Parser;
use embedded_nano_mesh::{ms, ExactAddressType, GeneralAddressType, Node, NodeConfig};
use platform_millis_linux::{LinuxMillis, PlatformMillis};
use platform_serial_linux::LinuxSerial;

#[derive(Parser, Debug)]
pub struct ReceiveArgs {
    #[clap(
        short = 'a',
        long = "current-address",
        required = true,
        help = constants::CURRENT_ADDRESS_HELP_MSG
    )]
    pub current_address: ExactAddressType,

    #[clap(
        long = "listen-period",
        required = true,
        help = constants::LISTEN_PERIOD_HELP_MSG
    )]
    pub listen_period: ms,

    #[clap(short = 'o', long = "timeout", required = true, help = constants::RECEIVE_TIMEOUT_HELP_MSG)]
    pub timeout: ms,

    #[clap(
        short = 'p',
        long = "port",
        required = true,
        help = constants::PORT_HELP_MSG
    )]
    pub port: String,
}

pub fn process_receive(args: ReceiveArgs) {
    let mut node = Node::new(NodeConfig {
        device_address: args.current_address as ExactAddressType,
        listen_period: args.listen_period as ms,
    });

    let exit_time = LinuxMillis::millis() + args.timeout as ms;

    loop {
        let current_time = LinuxMillis::millis();
        if current_time >= exit_time {
            std::process::exit(1);
        }

        let _ = node.update::<LinuxMillis, LinuxSerial>();
        let received_message = node.receive();

        if let Some(packet) = received_message {
            let target_addr: String;
            println!(
                "from_address: {}, to_address: {}, content: {}",
                packet.source_device_identifier,
                match packet.destination_device_identifier {
                    GeneralAddressType::Broadcast => "Broadcast",
                    GeneralAddressType::Exact(addr) => {
                        target_addr = addr.get().to_string().clone();
                        target_addr.as_str()
                    }
                },
                packet
                    .data
                    .into_iter()
                    .map(|character| character as char)
                    .collect::<String>()
            );
            std::process::exit(0);
        }
    }
}

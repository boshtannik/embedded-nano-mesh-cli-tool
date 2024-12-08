use super::constants;
use clap::Parser;
use embedded_nano_mesh::{
    ms, ExactAddressType, GeneralAddressType, Node, NodeConfig, PacketDataBytes,
};
use platform_millis_linux::{LinuxMillis, PlatformMillis};
use platform_serial_linux::LinuxSerial;

#[derive(Parser, Debug, Clone)]
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
    pub work_mode: WorkMode,
    pub output_mode: OutputMode,
}

#[derive(clap::ValueEnum, Debug, Clone, PartialEq, Eq)]
pub enum OutputMode {
    FullData,
    DataOnly,
}

#[derive(clap::ValueEnum, Debug, Clone, PartialEq, Eq)]
pub enum WorkMode {
    ExitOnReceive,
    ExitOnTimeout,
    Forever,
}

pub fn process_receive(args: ReceiveArgs) {
    let mut node = Node::new(NodeConfig {
        device_address: args.current_address as ExactAddressType,
        listen_period: args.listen_period as ms,
    });

    let exit_time = LinuxMillis::millis() + args.timeout as ms;

    loop {
        let _ = node.update::<LinuxMillis, LinuxSerial>();
        let packet = node.receive();
        match packet {
            None => continue,
            Some(packet) => {
                let destination_address =
                    if packet.is_destination_reached(GeneralAddressType::Broadcast) {
                        &"Broadcast".to_string()
                    } else {
                        &args.current_address.to_string()
                    };
                match args.work_mode {
                    WorkMode::Forever => {
                        print_packet(
                            args.clone(),
                            packet.data,
                            packet.source_device_identifier.into(),
                            destination_address,
                        );
                    }
                    WorkMode::ExitOnReceive => {
                        print_packet(
                            args.clone(),
                            packet.data,
                            packet.source_device_identifier.into(),
                            destination_address,
                        );
                        std::process::exit(0);
                    }
                    WorkMode::ExitOnTimeout => {
                        print_packet(
                            args.clone(),
                            packet.data,
                            packet.source_device_identifier.into(),
                            destination_address,
                        );
                        let current_time = LinuxMillis::millis();
                        if current_time >= exit_time {
                            std::process::exit(1);
                        }
                    }
                }
            }
        }
    }
}

#[inline]
fn print_packet(
    args: ReceiveArgs,
    data: PacketDataBytes,
    source_device_identifier: GeneralAddressType,
    destination_address: &str,
) {
    print_out_data(
        data,
        source_device_identifier,
        destination_address,
        args.output_mode == OutputMode::FullData,
    )
}

fn print_out_data(
    data: PacketDataBytes,
    source_device_identifier: GeneralAddressType,
    destination_address: &str,
    full_data: bool,
) {
    if full_data {
        println!(
            "from_address: {}, to_address: {}, content: {}",
            <GeneralAddressType as Into<u8>>::into(source_device_identifier),
            destination_address,
            data.into_iter()
                .map(|character| character as char)
                .collect::<String>()
        );
    } else {
        println!(
            "{}",
            data.into_iter()
                .map(|character| character as char)
                .collect::<String>()
        );
    }
}

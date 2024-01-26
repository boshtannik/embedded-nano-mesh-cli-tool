use clap::Parser;
use embedded_nano_mesh::*;
use platform_millis_linux::*;
use platform_serial_linux::*;

const FILTER_OUT_DUPLICATION_HELP_MSG: &str = "Tells to all devices within the network to ignore this packet if it has already been received. The purpose of it is to reduce the amount of traffic.";
const LIFETIME_HELP_MSG: &str = "Tells how many nodes the packet will be able to pass.";
const SPEC_OPERATION_TIMEOUT_HELP_MSG: &str =
    "Tells how long the node will listen for the response.";
const SEND_TIMEOUT_HELP_MSG: &str =
    "Tells for how long the node will update itself to send the packet from it's internal queues.";
const SEND_CONTENT_HELP_MSG: &str = "Content to send.";
const TO_ADDRESS_HELP_MSG: &str = "To address.";
const FROM_ADDRESS_HELP_MSG: &str = "From address.";
const LISTEN_PERIOD_HELP_MSG: &str = "Each device listens for this period of time before speaking. This parameter configures for how long the device will listen.";
const RECEIVE_TIMEOUT_HELP_MSG: &str = "Each device listens for this period of time trying to receiving. This parameter configures for how long the device will listen.";
const PING_CONTENT_HELP_MSG: &str = "Content to send with ping packet.";
const TRANSACTION_CONTENT_HELP_MSG: &str = "Content to send with transaction packet.";
const CURRENT_ADDRESS_HELP_MSG: &str =
    "Address of current device, which has to receive the message.";

fn main() {
    let command = Command::parse();

    configure_serial(
        "/dev/ttyUSB0".to_string(),
        PortSettings {
            baud_rate: BaudRate::Baud9600,
            char_size: CharSize::Bits8,
            parity: Parity::ParityNone,
            stop_bits: StopBits::Stop1,
            flow_control: FlowControl::FlowNone,
        },
    );

    match command {
        Command::Send(args) => {
            let mut node = Node::new(NodeConfig {
                device_address: args.from_address as AddressType,
                listen_period: args.listen_period as ms,
            });

            let _ = node.send(
                NodeString::from(args.send_content.as_str()).into_bytes(),
                args.to_address as AddressType,
                args.lifetime as LifeTimeType,
                args.filter_out_duplication,
            );

            let exit_time = LinuxTime::millis() + args.timeout as ms;

            loop {
                let current_time = LinuxTime::millis();
                if current_time >= exit_time {
                    std::process::exit(0);
                }
                let _ = node.update::<LinuxTime, LinuxSerial>();
            }
        }
        Command::Receive(args) => {
            let mut node = Node::new(NodeConfig {
                device_address: args.current_address as AddressType,
                listen_period: args.listen_period as ms,
            });

            let exit_time = LinuxTime::millis() + args.timeout as ms;

            loop {
                let current_time = LinuxTime::millis();
                if current_time >= exit_time {
                    std::process::exit(1);
                }

                let _ = node.update::<LinuxTime, LinuxSerial>();
                let received_message = node.receive();

                if let Some(packet) = received_message {
                    println!(
                        "from_address: {}, to_address: {}, content: {}",
                        packet.source_device_identifier,
                        packet.destination_device_identifier,
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
        Command::Ping(args) => {
            let mut node = Node::new(NodeConfig {
                device_address: args.from_address as AddressType,
                listen_period: args.listen_period as ms,
            });

            match node.send_ping_pong::<LinuxTime, LinuxSerial>(
                NodeString::from(args.ping_content.as_str()).into_bytes(),
                args.to_address as AddressType,
                args.lifetime as LifeTimeType,
                args.filter_out_duplication,
                args.timeout as ms,
            ) {
                Ok(_) => std::process::exit(0),
                Err(_) => std::process::exit(1),
            }
        }
        Command::Transaction(args) => {
            let mut node = Node::new(NodeConfig {
                device_address: args.from_address as AddressType,
                listen_period: args.listen_period as ms,
            });

            match node.send_with_transaction::<LinuxTime, LinuxSerial>(
                NodeString::from(args.transaction_content.as_str()).into_bytes(),
                args.to_address as AddressType,
                args.lifetime as LifeTimeType,
                args.filter_out_duplication,
                args.timeout as ms,
            ) {
                Ok(_) => std::process::exit(0),
                Err(_) => std::process::exit(1),
            }
        }
    }
}

#[derive(Parser, Debug)]
#[clap(
    name = "Nano mesh CLI communication tool",
    version = "1.0",
    author = "Boshtannik <boshtannik@gmail.com>"
)]
enum Command {
    Send(SendArgs),
    Receive(ReceiveArgs),
    Ping(PingArgs),
    Transaction(TransactionArgs),
}

#[derive(Parser, Debug)]
struct SendArgs {
    #[clap(
        short = 'f',
        long = "from-address",
        required = true,
        help = FROM_ADDRESS_HELP_MSG
    )]
    from_address: AddressType,

    #[clap(short = 't', long = "to-address", required = true, help = TO_ADDRESS_HELP_MSG)]
    to_address: AddressType,

    #[clap(
        short = 'l',
        long = "listen-period",
        required = true,
        help = LISTEN_PERIOD_HELP_MSG
    )]
    listen_period: ms,

    #[clap(
        short = 's',
        long = "send-content",
        required = true,
        help = SEND_CONTENT_HELP_MSG
    )]
    send_content: NodeString,

    #[clap(short = 'o', long = "timeout", required = true, help = SEND_TIMEOUT_HELP_MSG)]
    timeout: ms,

    #[clap(
        short = 'd',
        long = "filter-out-duplication",
        required = true,
        help = FILTER_OUT_DUPLICATION_HELP_MSG
    )]
    filter_out_duplication: bool,

    #[clap(
        short = 'l',
        long = "lifetime",
        required = true,
        help = LIFETIME_HELP_MSG
    )]
    lifetime: LifeTimeType,
}

#[derive(Parser, Debug)]
struct ReceiveArgs {
    #[clap(
        short = 'a',
        long = "current-address",
        required = true,
        help = CURRENT_ADDRESS_HELP_MSG
    )]
    current_address: AddressType,

    #[clap(
        short = 'l',
        long = "listen-period",
        required = true,
        help = LISTEN_PERIOD_HELP_MSG
    )]
    listen_period: ms,

    #[clap(short = 'o', long = "timeout", required = true, help = RECEIVE_TIMEOUT_HELP_MSG)]
    timeout: ms,
}

#[derive(Parser, Debug)]
struct PingArgs {
    #[clap(
        short = 'f',
        long = "from-address",
        required = true,
        help = FROM_ADDRESS_HELP_MSG
    )]
    from_address: AddressType,

    #[clap(short = 't', long = "to-address", required = true, help = TO_ADDRESS_HELP_MSG)]
    to_address: AddressType,

    #[clap(
        short = 'l',
        long = "listen-period",
        required = true,
        help = LISTEN_PERIOD_HELP_MSG
    )]
    listen_period: ms,

    #[clap(
        short = 'p',
        long = "ping-content",
        required = true,
        help = PING_CONTENT_HELP_MSG
    )]
    ping_content: NodeString,

    #[clap(short = 'o', long = "timeout", required = true, help = SPEC_OPERATION_TIMEOUT_HELP_MSG)]
    timeout: ms,

    #[clap(
        short = 'd',
        long = "filter-out-duplication",
        required = true,
        help = FILTER_OUT_DUPLICATION_HELP_MSG
    )]
    filter_out_duplication: bool,

    #[clap(
        short = 'l',
        long = "lifetime",
        required = true,
        help = LIFETIME_HELP_MSG
    )]
    lifetime: LifeTimeType,
}

#[derive(Parser, Debug)]
struct TransactionArgs {
    #[clap(
        short = 'f',
        long = "from-address",
        required = true,
        help = FROM_ADDRESS_HELP_MSG
    )]
    from_address: AddressType,

    #[clap(short = 't', long = "to-address", required = true, help = TO_ADDRESS_HELP_MSG)]
    to_address: AddressType,

    #[clap(
        short = 'l',
        long = "listen-period",
        required = true,
        help = LISTEN_PERIOD_HELP_MSG
    )]
    listen_period: ms,

    #[clap(
        short = 'r',
        long = "transaction-content",
        required = true,
        help = TRANSACTION_CONTENT_HELP_MSG
    )]
    transaction_content: NodeString,

    #[clap(short = 'o', long = "timeout", required = true, help = SPEC_OPERATION_TIMEOUT_HELP_MSG)]
    timeout: ms,

    #[clap(
        short = 'd',
        long = "filter-out-duplication",
        required = true,
        help = FILTER_OUT_DUPLICATION_HELP_MSG
    )]
    filter_out_duplication: bool,

    #[clap(
        short = 'l',
        long = "lifetime",
        required = true,
        help = LIFETIME_HELP_MSG
    )]
    lifetime: LifeTimeType,
}

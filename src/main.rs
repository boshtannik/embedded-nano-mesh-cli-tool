use clap::Parser;
use embedded_nano_mesh::*;
use platform_millis_linux::*;
use platform_serial_linux::*;

// This program should do next.
// It can be called for one of next purposes:
// 1 - To simply send message, and hang for a timeout, to let the message to be sent.
// 2 - To receive message and hang for a timeout, to let the message to be received.
// 3 - To send message using ping-pong method, and wait for some timeout. Also this use case shall return operation code, that
//   indicates wether the response was received or not.
// 4 - To send message using transaction method, and wait for some timeout. Also this use case shall return operation code, that
//   indicates wether the transaction was successful or not.
// It shall be called as command line tool.
// Use cases:
//
// // simple message sending. Send message to address 2, and hang for 250ms in update loop.
// program \
// --current-address=1 \
// --to-address=2 \
// --listen-period=250 \
// --send-content="Hello, World!" \
// --timeout=250
// This will set node address to 1, listen period to 250ms, send "Hello, World!" to device address 2, and loop 250ms, and then exits with return code Ok.
//
// // ping-pong message sending. Send message to address 2, and hang for 2000 ms of timeout waiting
// for response. In case of pong packet was received - returns Ok code.
// program \
// --current-address=1 \
// --to-address=2
// --listen-period=250 \
// --timeout=2000 \
// --ping-content="Hello, World!" \
// This will set node address to 1, listen period to 250ms, send ping-pong type message "Hello, World!" to device address 2, and in case of pong packet was received - returns Ok code.
//
// // transaction message sending. Send message to address 2, and hang for 3000 ms of timeout waiting
// for transaction to be finished. In case of transaction being finished good - returns Ok code.
// program \
// --current-address=1 \
// --to-address=2 \
// --listen-period=250 \
// --transaction="Hello, World!" \
// This will set node address to 1, listen period to 250ms, send transaction type message "Hello, World!" to device address 2, and in case of transaction being finished good - returns Ok code.
//
// // Receive message. Receive message to this devide. Hang for 3000 ms of timeout waiting
// for message. In case of message was received - returns Ok code.
// program \
// --current-address=1 \
// --listen-period=3000 \
// --timeout=3000
// This will set node address to 1, listen period to 3000ms, and in case of message was received - returns Ok code.
//

fn main() {
    let command = Command::parse();

    // Configuration of serial port, which will be used for communication
    // with radio modem
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
            // Initialization of mesh node.
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
                    break;
                }
                let _ = node.update::<LinuxTime, LinuxSerial>();
            }
        }
        Command::Receive(args) => {
            // Handle receive command
            // Initialization of mesh node.
            let mut node = Node::new(NodeConfig {
                device_address: args.current_address as AddressType,
                listen_period: args.listen_period as ms,
            });

            let exit_time = LinuxTime::millis() + args.timeout as ms;

            loop {
                let current_time = LinuxTime::millis();
                if current_time >= exit_time {
                    break;
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
                }
            }
        }
        Command::Ping(args) => {
            // Initialization of mesh node.
            let mut node = Node::new(NodeConfig {
                device_address: args.from_address as AddressType,
                listen_period: args.listen_period as ms,
            });

            // Handle ping command
            // example of ping-pong like packet send. Similar to ping, but makes the receiver respond with
            // pong with same content. Successful response will tell, that receiver has received the message
            let _ = node.send_ping_pong::<LinuxTime, LinuxSerial>(
                NodeString::from(args.ping_content.as_str()).into_bytes(), // This is the message to be sent
                args.to_address as AddressType, // This is address of the device, which will receive the message
                args.lifetime as LifeTimeType, // This is life time of the message. Indicates how many hops the message will make
                args.filter_out_duplication, // This is flag, that indicates, if the devices of the network should ignore duplication of the message by other nodes.
                args.timeout as ms,          // This is the period of the ping-pong message
            );
        }
        Command::Transaction(args) => {
            // Handle transaction command
            // Initialization of mesh node.
            let mut node = Node::new(NodeConfig {
                device_address: args.from_address as AddressType,
                listen_period: args.listen_period as ms,
            });

            // example of transaction like packet send.
            // This will make the receiver respond with finish transaction with same content.
            // Successful response will tell, that receiver has received the message, and reacted on it
            // only once.
            let _ = node.send_with_transaction::<LinuxTime, LinuxSerial>(
                NodeString::from(args.transaction_content.as_str()).into_bytes(),
                args.to_address as AddressType,
                args.lifetime as LifeTimeType,
                args.filter_out_duplication,
                args.timeout as ms,
            );
        }
    }
}

#[derive(Parser, Debug)]
#[clap(
    name = "program",
    version = "1.0",
    author = "Your Name <you@example.com>"
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
        help = "From address"
    )]
    from_address: AddressType,

    #[clap(short = 't', long = "to-address", required = true, help = "To address")]
    to_address: AddressType,

    #[clap(
        short = 'l',
        long = "listen-period",
        required = true,
        help = "Listen period in ms, during which this device will be listening for packets before speaking"
    )]
    listen_period: ms,

    #[clap(
        short = 's',
        long = "send-content",
        required = true,
        help = "Content to send"
    )]
    send_content: NodeString,

    #[clap(short = 'o', long = "timeout", required = true, help = "Timeout in ms")]
    timeout: ms,

    #[clap(
        short = 'd',
        long = "filter-out-duplication",
        required = true,
        help = "Says if devices of the network should ignore duplication of the message by other nodes."
    )]
    filter_out_duplication: bool,

    #[clap(
        short = 'l',
        long = "lifetime",
        required = true,
        help = "Lifetime, tells how many hops the message will make"
    )]
    lifetime: LifeTimeType,
}

#[derive(Parser, Debug)]
struct ReceiveArgs {
    #[clap(
        short = 'a',
        long = "current-address",
        required = true,
        help = "Current address"
    )]
    current_address: AddressType,

    #[clap(
        short = 'l',
        long = "listen-period",
        required = true,
        help = "Listen period in ms, during which this device will be listening for packets before speaking"
    )]
    listen_period: ms,

    #[clap(short = 'o', long = "timeout", required = true, help = "Timeout in ms")]
    timeout: ms,
}

#[derive(Parser, Debug)]
struct PingArgs {
    #[clap(
        short = 'f',
        long = "from-address",
        required = true,
        help = "From address"
    )]
    from_address: AddressType,

    #[clap(short = 't', long = "to-address", required = true, help = "To address")]
    to_address: AddressType,

    #[clap(
        short = 'l',
        long = "listen-period",
        required = true,
        help = "Listen period in ms, during which this device will be listening for packets before speaking"
    )]
    listen_period: ms,

    #[clap(
        short = 'p',
        long = "ping-content",
        required = true,
        help = "Content to send"
    )]
    ping_content: NodeString,

    #[clap(short = 'o', long = "timeout", required = true, help = "Timeout in ms")]
    timeout: ms,

    #[clap(
        short = 'd',
        long = "filter-out-duplication",
        required = true,
        help = "Says if devices of the network should ignore duplication of the message by other nodes."
    )]
    filter_out_duplication: bool,

    #[clap(
        short = 'l',
        long = "lifetime",
        required = true,
        help = "Lifetime, tells how many hops the message will make"
    )]
    lifetime: LifeTimeType,
}

#[derive(Parser, Debug)]
struct TransactionArgs {
    #[clap(
        short = 'f',
        long = "from-address",
        required = true,
        help = "From address"
    )]
    from_address: AddressType,

    #[clap(short = 't', long = "to-address", required = true, help = "To address")]
    to_address: AddressType,

    #[clap(
        short = 'l',
        long = "listen-period",
        required = true,
        help = "Listen period in ms, during which this device will be listening for packets before speaking"
    )]
    listen_period: ms,

    #[clap(
        short = 'r',
        long = "transaction",
        required = true,
        help = "Content to send"
    )]
    transaction_content: NodeString,

    #[clap(short = 'o', long = "timeout", required = true, help = "Timeout in ms")]
    timeout: ms,

    #[clap(
        short = 'd',
        long = "filter-out-duplication",
        required = true,
        help = "Says if devices of the network should ignore duplication of the message by other nodes."
    )]
    filter_out_duplication: bool,

    #[clap(
        short = 'l',
        long = "lifetime",
        required = true,
        help = "Lifetime, tells how many hops the message will make"
    )]
    lifetime: LifeTimeType,
}

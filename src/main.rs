mod constants;
mod ping;
mod receive;
mod send;
mod transaction;

use clap::Parser;

pub use embedded_nano_mesh::*;
use ping::process_ping;
use platform_serial_linux::*;
use receive::process_receive;
use send::process_send;
use transaction::process_transaction;

#[derive(Parser, Debug)]
#[clap(
    name = "Nano mesh CLI communication tool",
    version = "2.0.0",
    author = "Boshtannik <boshtannik@gmail.com>"
)]
enum Command {
    Send(send::SendArgs),
    Receive(receive::ReceiveArgs),
    Ping(ping::PingArgs),
    Transaction(transaction::TransactionArgs),
}

fn _configure_serial(port_path: String) {
    configure_serial(
        port_path,
        PortSettings {
            baud_rate: BaudRate::Baud9600,
            char_size: CharSize::Bits8,
            parity: Parity::ParityNone,
            stop_bits: StopBits::Stop1,
            flow_control: FlowControl::FlowNone,
        },
    );
}

fn main() {
    let command = Command::parse();

    match command {
        Command::Send(args) => {
            _configure_serial(args.port.clone());
            process_send(args);
        }
        Command::Receive(args) => {
            _configure_serial(args.port.clone());
            process_receive(args);
        }
        Command::Ping(args) => {
            _configure_serial(args.port.clone());
            process_ping(args);
        }
        Command::Transaction(args) => {
            _configure_serial(args.port.clone());
            process_transaction(args);
        }
    }
}

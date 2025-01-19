mod broadcast;
mod constants;
mod ping;
mod receive;
mod send_to_exact;
mod transaction;

use broadcast::process_broadcast;
use clap::Parser;

pub use embedded_nano_mesh::*;
use ping::process_ping;
use receive::process_receive;
use send_to_exact::process_send;
use transaction::process_transaction;

#[derive(Parser, Debug)]
#[clap(
    name = "Nano mesh CLI communication tool",
    version = "1.1.4-Jump_back_from_version_2.0.0 to be in sync with embedded_nano_mesh library version.",
    author = "Boshtannik <boshtannik@gmail.com>"
)]
/// Command line utility to communicate wtih embedded_nano_mesh network
/// using command line interface. This utility uses serial port which has
/// radio module connected to interact with the network.
enum Command {
    /// Sends the message to exact device within the network.
    SendToExact(send_to_exact::SendToExactArgs),

    /// Sends the message to all devices of the network.
    Broadcast(broadcast::BroadcastArgs),

    /// Turns this computer into receiver of messages from netwrrk for specified period of time.
    Receive(receive::ReceiveArgs),

    /// Forces receiver to respond with exact message back with `pong` flag being set.
    Ping(ping::PingArgs),

    /// Ensures that both devices know about successfull data exchange.
    Transaction(transaction::TransactionArgs),
}

fn main() {
    let command = Command::parse();

    match command {
        Command::SendToExact(args) => {
            process_send(args);
        }
        Command::Broadcast(args) => {
            process_broadcast(args);
        }
        Command::Receive(args) => {
            process_receive(args);
        }
        Command::Ping(args) => {
            process_ping(args);
        }
        Command::Transaction(args) => {
            process_transaction(args);
        }
    }
}

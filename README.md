## CLI tool for embedded-nano-mesh protocol
This tool provides communication interface between linux CLI interface and mesh network

# Use cases
It allows to:
* Send message
* Ping device
* Send message with transaction
* Receive the message from network

## Usage
Usage:
### See general usage
```
jack@jack-debian:~/w/rust/arduino/nano_mesh_cli_tool$ ./nano_mesh_cli_tool 
Usage: nano_mesh_cli_tool <COMMAND>

Commands:
  send         
  receive      
  ping         
  transaction  
  help         Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```
### See documentation of `send` use case
```
jack@jack-debian:~/w/rust/arduino/nano_mesh_cli_tool$ ./nano_mesh_cli_tool send
error: the following required arguments were not provided:
  --from-address <FROM_ADDRESS>
  --to-address <TO_ADDRESS>
  --listen-period <LISTEN_PERIOD>
  --send-content <SEND_CONTENT>
  --timeout <TIMEOUT>
  --filter-out-duplication
  --lifetime <LIFETIME>

Usage: nano_mesh_cli_tool send --from-address <FROM_ADDRESS> --to-address <TO_ADDRESS> --listen-period <LISTEN_PERIOD> --send-content <SEND_CONTENT> --timeout <TIMEOUT> --filter-out-duplication --lifetime <LIFETIME>

For more information, try '--help'.
```
### Example how to send "beep" message from device 1 to device 2
```
jack@jack-debian:~/w/rust/arduino/nano_mesh_cli_tool$ ./nano_mesh_cli_tool send --from-address=1 --to-address=2 --listen-period=250 --send-content="beep" --timeout=400 --filter-out-duplication --lifetime=10
```
### Check for help of `send` use
```
jack@jack-debian:~/w/rust/arduino/nano_mesh_cli_tool$ ./nano_mesh_cli_tool send --help
Usage: nano_mesh_cli_tool send --from-address <FROM_ADDRESS> --to-address <TO_ADDRESS> --listen-period <LISTEN_PERIOD> --send-content <SEND_CONTENT> --timeout <TIMEOUT> --filter-out-duplication --lifetime <LIFETIME>

Options:
  -f, --from-address <FROM_ADDRESS>    From address
  -t, --to-address <TO_ADDRESS>        To address
  -l, --listen-period <LISTEN_PERIOD>  Listen period in ms, during which this device will be listening for packets before speaking
  -s, --send-content <SEND_CONTENT>    Content to send
  -o, --timeout <TIMEOUT>              Timeout in ms
  -d, --filter-out-duplication         Says if devices of the network should ignore duplication of the message by other nodes.
  -l, --lifetime <LIFETIME>            Lifetime, tells how many hops the message will make
  -h, --help                           Print help
```

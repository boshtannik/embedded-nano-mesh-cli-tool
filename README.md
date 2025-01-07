## CLI tool for embedded-nano-mesh protocol
This tool provides communication interface between linux CLI interface and embedded nano mesh network

# Use cases
It allows to:
* Send message to exact device, or to all devices via multicast reserved address.
* Ping device, or send the message with PING flag which forces receiver to reply.
* Send message with transaction, which forces receiver to reply, about the operation being done.
* Receive the message from network

# Installation
1. Get rust from official site. You can use this command: ```curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh```
2. Clone this repo with: ```git clone https://github.com/boshtannik/embedded-nano-mesh-cli-tool.git```
3. Enter into nano-mesh-cli-tool: ```cd embedded-nano-mesh-cli-tool```
4. Build it by run: ```cargo build --release && cp target/release/nano_mesh_cli_tool ./nano_mesh_cli_tool```
5. Now you can use it with ```./nano_mesh_cli_tool --help```

# Usage:
## Send message to device 3 from computer which pretends to be device 1. Send does not require a response from receiving device.
```
user@user-debian:~/w/rust/arduino/nano_mesh_cli_tool$ ./nano_mesh_cli_tool \
send_to_exact \
--from-address=1 \
--to-address=3 \
--listen-period=250 \
--content="beep" \
--timeout=1000 \
--filter-out-duplication \
--lifetime=1 \
--port=/dev/ttyUSB0 \
&& echo $?
```

## Send message to all devices from computer which pretends to be device 1. Broadcast does not require a response from receiving device.
```
user@user-debian:~/w/rust/arduino/nano_mesh_cli_tool$ ./nano_mesh_cli_tool \
broadcast \
--from-address=1 \
--listen-period=250 \
--content="beep" \
--timeout=1000 \
--filter-out-duplication \
--lifetime=1 \
--port=/dev/ttyUSB0 \
&& echo $?
```

## Send message to device 3 from computer which pretends to be device 1. Send does require a simple response from receiving device.
```
user@user-debian:~/w/rust/arduino/nano_mesh_cli_tool$ ./nano_mesh_cli_tool \
ping \
--from-address=1 \
--to-address=3 \
--listen-period=250 \
--content="beep" \
--timeout=1000 \
--lifetime=1 \
--port=/dev/ttyUSB0 \
&& echo $?
```

## Send message to device 3 from computer which pretends to be device 1. Send does require a strict proven response from receiving device.
```
user@user-debian:~/w/rust/arduino/nano_mesh_cli_tool$ ./nano_mesh_cli_tool \
transaction \
--from-address=1 \
--to-address=3 \
--listen-period=250 \
--content="beep" \
--timeout=1000 \
--lifetime=1 \
--port=/dev/ttyUSB0 \
&& echo $?
```

# Helping documentation
For more help of usage - use `--help` argument.
You can use `--help` for general program help, or for one of it's parts, such as `send_to_exact`, `broadcast` `ping` `transaction` `receive` functions

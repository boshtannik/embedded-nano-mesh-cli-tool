## CLI tool for embedded-nano-mesh protocol
This tool provides communication interface between linux CLI interface and embedded nano mesh network

# Use cases
It allows to:
* Send message to exact device, or to all devices via multicast reserved address.
* Ping device, or send the message with PING flag which forces receiver to reply.
* Send message with transaction, which forces receiver to reply, about the operation being done.
* Receive the message from network

# Usage:
## Send message to device 3 from computer which pretends to be device 1. Send does not require a response from receiving device.
```
user@user-debian:~/w/rust/arduino/nano_mesh_cli_tool$ ./nano_mesh_cli_tool \
send \
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

## Send message to device 3 from computer which pretends to be device 1. Send does require a simple response from receiving device.
```
user@user-debian:~/w/rust/arduino/nano_mesh_cli_tool$ ./nano_mesh_cli_tool \
ping \
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

## Send message to device 3 from computer which pretends to be device 1. Send does require a strict proven response from receiving device.
```
user@user-debian:~/w/rust/arduino/nano_mesh_cli_tool$ ./nano_mesh_cli_tool \
transaction \
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

# Helping documentation
For more help of usage - use `--help` argument.
You can use `--help` for general program help, or for one of it's parts, such as `send` `ping` `transaction` `receive` functions

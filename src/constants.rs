pub const FILTER_OUT_DUPLICATION_HELP_MSG: &str = concat!(
    "Flag that tells to all devices within the network to ignore duplication\n",
    "of this exact same packet caught if it previously been received.\n",
    "The purpose of it is to reduce the amount of unnecessary traffic.\n"
);
pub const LIFETIME_HELP_MSG: &str = "Tells how many nodes the packet will be able to pass.\n";
pub const SPEC_OPERATION_TIMEOUT_HELP_MSG: &str =
    "Tells how long the node will listen for the response.\n";
pub const SEND_TIMEOUT_HELP_MSG: &str = concat!(
    "Tells for how long this node will be live since command start running. \n",
    "This continious update is needed to let the packet be sent from \n",
    "node's internal queues.\n"
);
pub const SEND_CONTENT_HELP_MSG: &str = "Content to send.\n";
pub const TO_ADDRESS_HELP_MSG: &str = "To address.\n";
pub const FROM_ADDRESS_HELP_MSG: &str = "From address.\n";
pub const LISTEN_PERIOD_HELP_MSG: &str = concat!(
    "Each device listens for this period of time before speaking.\n",
    "This parameter configures for how long the device will listen.\n",
    "Measured in milliseconds. This is made in order to prevent device\n",
    "to send messages one by one sequentially with no time gaps between them.\n"
);
pub const RECEIVE_TIMEOUT_HELP_MSG: &str = concat!(
    "Each device listens ether for it's own specified period of time.\n",
    "This parameter configures for how long the device will listen before\n",
    "speak into ether back. The purpose of this value - is to provide some\n",
    "sord of politeness of devices and to prevent the ether being spammed\n",
    "via packets."
);

pub const PING_CONTENT_HELP_MSG: &str = "Content to be sent with ping packet.\n";
pub const TRANSACTION_CONTENT_HELP_MSG: &str = "Content to be sent with transaction packet.\n";
pub const CURRENT_ADDRESS_HELP_MSG: &str =
    "Address of current device, which has to receive the message.\n";
pub const PORT_HELP_MSG: &str = "Port of current device, which is connected to radio module.\n";

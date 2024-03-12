use bitcoin::p2p::message;

// Message format as per documentation https://en.bitcoin.it/wiki/Protocol_documentation#verack
pub fn verack_msg() -> message::NetworkMessage {
    message::NetworkMessage::Verack
}

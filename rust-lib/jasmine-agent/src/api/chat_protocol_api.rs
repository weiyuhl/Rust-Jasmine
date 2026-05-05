use crate::chat_protocol::simple;

#[flutter_rust_bridge::frb(sync)]
pub fn chat_protocol_greet(name: String) -> String {
    simple::greet(name)
}

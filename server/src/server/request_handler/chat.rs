use crate::server::model::request::Request;
use crate::server::request_handler::atcommand::handle_atcommand;
use crate::server::Server;
use packets::packets::PacketCzPlayerChat;

pub fn handle_chat(server: &Server, context: Request) {
    let packet_player_chat = cast!(context.packet(), PacketCzPlayerChat);
    let char_id = context.session().char_id();
    let character = server.state().get_character_unsafe(char_id);
    debug!("Received chat from {}: {:?}", character.name, packet_player_chat.msg);
    if packet_player_chat.msg.starts_with(format!("{} : @", character.name).as_str()) { // TODO make symbol configurable
        handle_atcommand(server, context, packet_player_chat);
    }
}
pub mod clutter_filter_map;
pub mod digital_radar_data;
pub mod message_header;
pub mod rda_status_data;

mod message_type;
pub use message_type::MessageType;

mod message;
pub use message::{Message, MessageWithHeader};

mod definitions;
mod primitive_aliases;

use crate::messages::clutter_filter_map::decode_clutter_filter_map;
use crate::messages::digital_radar_data::decode_digital_radar_data;
use crate::messages::message_header::MessageHeader;
use crate::messages::rda_status_data::decode_rda_status_message;
use crate::result::Result;
use crate::util::deserialize;
use std::io::{Read, Seek};

/// Decode a NEXRAD Level II message from a reader.
pub fn decode_message_header<R: Read>(reader: &mut R) -> Result<MessageHeader> {
    deserialize(reader)
}

/// Decode a series of NEXRAD Level II messages from a reader.
pub fn decode_messages<R: Read + Seek>(reader: &mut R) -> Result<Vec<MessageWithHeader>> {
    let mut messages = Vec::new();
    while let Ok(header) = decode_message_header(reader) {
        let message = decode_message(reader, header.message_type())?;
        messages.push(MessageWithHeader { header, message });
    }
    Ok(messages)
}

/// Decode a NEXRAD Level II message of the specified type from a reader.
pub fn decode_message<R: Read + Seek>(
    reader: &mut R,
    message_type: MessageType,
) -> Result<Message> {
    if message_type == MessageType::RDADigitalRadarDataGenericFormat {
        let decoded_message = decode_digital_radar_data(reader)?;
        return Ok(Message::DigitalRadarData(Box::new(decoded_message)));
    }

    let mut message_buffer = [0; 2432 - size_of::<MessageHeader>()];
    reader.read_exact(&mut message_buffer)?;

    let message_reader = &mut message_buffer.as_ref();
    Ok(match message_type {
        MessageType::RDAStatusData => {
            Message::RDAStatusData(Box::new(decode_rda_status_message(message_reader)?))
        }
        MessageType::RDAClutterFilterMap => {
            Message::ClutterFilterMap(Box::new(decode_clutter_filter_map(message_reader)?))
        }
        _ => Message::Other,
    })
}

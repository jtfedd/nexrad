use crate::model::messages::MessageWithHeader;
use crate::model::Archive2Header;

/// An Archive II file containing decoded NEXRAD Level II data.
#[derive(Debug)]
pub struct Archive2File {
    /// The volume scan header.
    pub header: Archive2Header,

    /// The decoded messages.
    pub records: Vec<MessageWithHeader>,
}

use serde::{Deserialize, Serialize};

use crate::{Frame, NULL_FRAME};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConnectionStatus {
    pub disconnected: bool,
    pub last_frame: Frame,
}

impl Default for ConnectionStatus {
    fn default() -> Self {
        Self {
            disconnected: false,
            last_frame: NULL_FRAME,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct SyncRequest {
    pub random_request: u32, // please reply back with this random data
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct SyncReply {
    pub random_reply: u32, // here's your random data back
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Input {
    pub peer_connect_status: Vec<ConnectionStatus>,
    pub disconnect_requested: bool,
    pub start_frame: Frame,
    pub ack_frame: Frame,
    pub bytes: Vec<u8>,
}

impl Default for Input {
    fn default() -> Self {
        Self {
            peer_connect_status: Vec::new(),
            disconnect_requested: false,
            start_frame: NULL_FRAME,
            ack_frame: NULL_FRAME,
            bytes: Vec::new(),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct InputAck {
    pub ack_frame: Frame,
}

impl Default for InputAck {
    fn default() -> Self {
        Self {
            ack_frame: NULL_FRAME,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct QualityReport {
    pub frame_advantage: i8, // frame advantage of other player
    pub ping: u128,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct QualityReply {
    pub pong: u128,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct MessageHeader {
    pub magic: u16,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum MessageBody {
    SyncRequest(SyncRequest),
    SyncReply(SyncReply),
    Input(Input),
    InputAck(InputAck),
    QualityReport(QualityReport),
    QualityReply(QualityReply),
    KeepAlive,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct UdpMessage {
    pub header: MessageHeader,
    // pub(crate) header: MessageHeader,
    pub body: MessageBody,
    // pub(crate) body: MessageBody,
}

impl UdpMessage {
    pub fn dummy() -> Self {
        Self {
            header: MessageHeader { magic: 123 },
            body: MessageBody::KeepAlive,
        }
    }
}

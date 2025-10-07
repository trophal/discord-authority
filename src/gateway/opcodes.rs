/// Discord Gateway Opcodes
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OpCode {
    /// Dispatch - An event was dispatched
    Dispatch = 0,
    /// Heartbeat - Fired periodically by the client to keep the connection alive
    Heartbeat = 1,
    /// Identify - Starts a new session during the initial handshake
    Identify = 2,
    /// Presence Update - Update the client's presence
    PresenceUpdate = 3,
    /// Voice State Update - Join/leave or move between voice channels
    VoiceStateUpdate = 4,
    /// Resume - Resume a previous session that was disconnected
    Resume = 6,
    /// Reconnect - You should attempt to reconnect and resume immediately
    Reconnect = 7,
    /// Request Guild Members - Request information about guild members
    RequestGuildMembers = 8,
    /// Invalid Session - The session has been invalidated. You should reconnect and identify/resume accordingly
    InvalidSession = 9,
    /// Hello - Sent immediately after connecting, contains heartbeat interval
    Hello = 10,
    /// Heartbeat ACK - Sent in response to receiving a heartbeat to acknowledge that it was received
    HeartbeatAck = 11,
}

impl OpCode {
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(OpCode::Dispatch),
            1 => Some(OpCode::Heartbeat),
            2 => Some(OpCode::Identify),
            3 => Some(OpCode::PresenceUpdate),
            4 => Some(OpCode::VoiceStateUpdate),
            6 => Some(OpCode::Resume),
            7 => Some(OpCode::Reconnect),
            8 => Some(OpCode::RequestGuildMembers),
            9 => Some(OpCode::InvalidSession),
            10 => Some(OpCode::Hello),
            11 => Some(OpCode::HeartbeatAck),
            _ => None,
        }
    }
}


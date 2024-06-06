use crate::{Address, Capability, LazyLoadBlob, ProcessId};
use serde::{Deserialize, Serialize};

/// The basic message type. A message is either a request or a response. Best
/// practice when handling a message is to do this:
/// 1. Match on whether it's a request or a response
/// 2. Match on who the message is from (the `source`)
/// 3. Parse and interpret the `body`, `metadata`, and/or `context` based on
/// who the message is from and what your process expects from them.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Message {
    Request {
        source: Address,
        expects_response: Option<u64>,
        body: Vec<u8>,
        metadata: Option<String>,
        capabilities: Vec<Capability>,
    },
    Response {
        source: Address,
        body: Vec<u8>,
        metadata: Option<String>,
        context: Option<Vec<u8>>,
        capabilities: Vec<Capability>,
    },
}

impl Message {
    /// Get the source of a message.
    pub fn source(&self) -> &Address {
        match self {
            Message::Request { source, .. } => source,
            Message::Response { source, .. } => source,
        }
    }
    /// Get the IPC body of a message.
    pub fn body(&self) -> &[u8] {
        match self {
            Message::Request { body, .. } => body,
            Message::Response { body, .. } => body,
        }
    }
    /// Get the metadata of a message.
    pub fn metadata(&self) -> Option<&str> {
        match self {
            Message::Request { metadata, .. } => metadata.as_ref().map(|s| s.as_str()),
            Message::Response { metadata, .. } => metadata.as_ref().map(|s| s.as_str()),
        }
    }
    /// Get the context of a message. Always `None` for requests.
    pub fn context(&self) -> Option<&[u8]> {
        match self {
            Message::Request { .. } => None,
            Message::Response { context, .. } => context.as_ref().map(|s| s.as_slice()),
        }
    }
    /// Get the blob of a message, if any. This function must be called
    /// by the process that received the message **before** receiving another
    /// message! The blob can only be consumed immediately after receiving a message.
    pub fn blob(&self) -> Option<LazyLoadBlob> {
        crate::get_blob()
    }
    /// Get the capabilities of a message.
    pub fn capabilities(&self) -> &Vec<Capability> {
        match self {
            Message::Request { capabilities, .. } => capabilities,
            Message::Response { capabilities, .. } => capabilities,
        }
    }
    /// Check if a message is a request. Returns `false` if it's a response.
    pub fn is_request(&self) -> bool {
        matches!(self, Message::Request { .. })
    }
    /// Check if a message was sent by a local process. Returns `false` if the
    /// source node is not our local node.
    pub fn is_local(&self, our: &Address) -> bool {
        match self {
            Message::Request { source, .. } => source.node == our.node,
            Message::Response { source, .. } => source.node == our.node,
        }
    }
    /// Check the `ProcessId` of a message source against a given `ProcessId` or
    /// something that can be checked for equality against a `ProcessId`.
    pub fn is_process<T>(&self, process: T) -> bool
    where
        ProcessId: PartialEq<T>,
    {
        match self {
            Message::Request { source, .. } => source.process == process,
            Message::Response { source, .. } => source.process == process,
        }
    }
}

pub fn _wit_message_to_message(
    source: Address,
    message: crate::kinode::process::standard::Message,
) -> Message {
    match message {
        crate::kinode::process::standard::Message::Request(req) => Message::Request {
            source,
            expects_response: req.expects_response,
            body: req.body,
            metadata: req.metadata,
            capabilities: req.capabilities,
        },
        crate::kinode::process::standard::Message::Response((resp, context)) => Message::Response {
            source,
            body: resp.body,
            metadata: resp.metadata,
            context,
            capabilities: resp.capabilities,
        },
    }
}

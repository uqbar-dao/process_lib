use crate::uqbar::process::standard as wit;
pub use crate::uqbar::process::standard::*;
/// Uqbar process standard library for Rust compiled to WASM
/// Must be used in context of bindings generated by uqbar.wit
use serde::{Deserialize, Serialize};

wit_bindgen::generate!({
    path: "wit",
    world: "lib",
});

pub mod kernel_types;

/// Override the println! macro to print to the terminal
#[macro_export]
macro_rules! println {
    () => {
        $crate::print_to_terminal(0, "\n");
    };
    ($($arg:tt)*) => {{
        $crate::print_to_terminal(0, &format!($($arg)*));
    }};
}

/// PackageId is like a ProcessId, but for a package. Only contains the name
/// of the package and the name of the publisher.
#[derive(Hash, Eq, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct PackageId {
    package_name: String,
    publisher_node: String,
}

impl PackageId {
    pub fn new(package_name: &str, publisher_node: &str) -> Self {
        PackageId {
            package_name: package_name.into(),
            publisher_node: publisher_node.into(),
        }
    }
    pub fn from_str(input: &str) -> Result<Self, ProcessIdParseError> {
        // split string on colons into 2 segments
        let mut segments = input.split(':');
        let package_name = segments
            .next()
            .ok_or(ProcessIdParseError::MissingField)?
            .to_string();
        let publisher_node = segments
            .next()
            .ok_or(ProcessIdParseError::MissingField)?
            .to_string();
        if segments.next().is_some() {
            return Err(ProcessIdParseError::TooManyColons);
        }
        Ok(PackageId {
            package_name,
            publisher_node,
        })
    }
    pub fn to_string(&self) -> String {
        [self.package_name.as_str(), self.publisher_node.as_str()].join(":")
    }
    pub fn package(&self) -> &str {
        &self.package_name
    }
    pub fn publisher_node(&self) -> &str {
        &self.publisher_node
    }
}

/// ProcessId is defined in the wit bindings, but constructors and methods
/// are defined here.
impl ProcessId {
    /// generates a random u64 number if process_name is not declared
    pub fn new(process_name: &str, package_name: &str, publisher_node: &str) -> Self {
        ProcessId {
            process_name: process_name.into(),
            package_name: package_name.into(),
            publisher_node: publisher_node.into(),
        }
    }
    pub fn from_str(input: &str) -> Result<Self, ProcessIdParseError> {
        // split string on colons into 3 segments
        let mut segments = input.split(':');
        let process_name = segments
            .next()
            .ok_or(ProcessIdParseError::MissingField)?
            .to_string();
        let package_name = segments
            .next()
            .ok_or(ProcessIdParseError::MissingField)?
            .to_string();
        let publisher_node = segments
            .next()
            .ok_or(ProcessIdParseError::MissingField)?
            .to_string();
        if segments.next().is_some() {
            return Err(ProcessIdParseError::TooManyColons);
        }
        Ok(ProcessId {
            process_name,
            package_name,
            publisher_node,
        })
    }
    pub fn to_string(&self) -> String {
        [
            self.process_name.as_str(),
            self.package_name.as_str(),
            self.publisher_node.as_str(),
        ]
        .join(":")
    }
    pub fn process(&self) -> &str {
        &self.process_name
    }
    pub fn package(&self) -> &str {
        &self.package_name
    }
    pub fn publisher_node(&self) -> &str {
        &self.publisher_node
    }
}

pub trait IntoProcessId {
    fn into_process_id(self) -> Result<ProcessId, ProcessIdParseError>;
}

impl IntoProcessId for ProcessId {
    fn into_process_id(self) -> Result<ProcessId, ProcessIdParseError> {
        Ok(self)
    }
}

impl IntoProcessId for &str {
    fn into_process_id(self) -> Result<ProcessId, ProcessIdParseError> {
        ProcessId::from_str(self)
    }
}

impl std::fmt::Display for ProcessId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}:{}:{}",
            self.process_name, self.package_name, self.publisher_node
        )
    }
}

impl PartialEq for ProcessId {
    fn eq(&self, other: &Self) -> bool {
        self.process_name == other.process_name
            && self.package_name == other.package_name
            && self.publisher_node == other.publisher_node
    }
}

impl PartialEq<&str> for ProcessId {
    fn eq(&self, other: &&str) -> bool {
        &self.to_string() == other
    }
}

impl PartialEq<ProcessId> for &str {
    fn eq(&self, other: &ProcessId) -> bool {
        self == &other.to_string()
    }
}

#[derive(Debug)]
pub enum ProcessIdParseError {
    TooManyColons,
    MissingField,
}

impl std::fmt::Display for ProcessIdParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ProcessIdParseError::TooManyColons => "Too many colons in ProcessId string",
                ProcessIdParseError::MissingField => "Missing field in ProcessId string",
            }
        )
    }
}

impl std::error::Error for ProcessIdParseError {
    fn description(&self) -> &str {
        match self {
            ProcessIdParseError::TooManyColons => "Too many colons in ProcessId string",
            ProcessIdParseError::MissingField => "Missing field in ProcessId string",
        }
    }
}

/// Address is defined in the wit bindings, but constructors and methods here.
impl Address {
    pub fn new<T: IntoProcessId>(node: &str, process: T) -> Result<Address, ProcessIdParseError> {
        Ok(Address {
            node: node.to_string(),
            process: process.into_process_id()?,
        })
    }
    pub fn from_str(input: &str) -> Result<Self, AddressParseError> {
        // split string on colons into 4 segments,
        // first one with @, next 3 with :
        let mut name_rest = input.split('@');
        let node = name_rest
            .next()
            .ok_or(AddressParseError::MissingField)?
            .to_string();
        let mut segments = name_rest
            .next()
            .ok_or(AddressParseError::MissingNodeId)?
            .split(':');
        let process_name = segments
            .next()
            .ok_or(AddressParseError::MissingField)?
            .to_string();
        let package_name = segments
            .next()
            .ok_or(AddressParseError::MissingField)?
            .to_string();
        let publisher_node = segments
            .next()
            .ok_or(AddressParseError::MissingField)?
            .to_string();
        if segments.next().is_some() {
            return Err(AddressParseError::TooManyColons);
        }
        Ok(Address {
            node,
            process: ProcessId {
                process_name,
                package_name,
                publisher_node,
            },
        })
    }
    pub fn to_string(&self) -> String {
        [self.node.as_str(), &self.process.to_string()].join("@")
    }
}

pub trait IntoAddress {
    fn into_address(self) -> Result<Address, AddressParseError>;
}

impl IntoAddress for Address {
    fn into_address(self) -> Result<Address, AddressParseError> {
        Ok(self)
    }
}

impl IntoAddress for &str {
    fn into_address(self) -> Result<Address, AddressParseError> {
        Address::from_str(self)
    }
}

#[derive(Debug)]
pub enum AddressParseError {
    TooManyColons,
    MissingNodeId,
    MissingField,
}

impl std::fmt::Display for AddressParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                AddressParseError::TooManyColons => "Too many colons in ProcessId string",
                AddressParseError::MissingNodeId => "Node ID missing",
                AddressParseError::MissingField => "Missing field in ProcessId string",
            }
        )
    }
}

impl std::error::Error for AddressParseError {
    fn description(&self) -> &str {
        match self {
            AddressParseError::TooManyColons => "Too many colons in ProcessId string",
            AddressParseError::MissingNodeId => "Node ID missing",
            AddressParseError::MissingField => "Missing field in ProcessId string",
        }
    }
}

///
/// Here, we define wrappers over the wit bindings to make them easier to use.
/// This library prescribes the use of IPC and metadata types serialized and
/// deserialized to JSON, which is far from optimal for performance, but useful
/// for applications that want to maximize composability and introspectability.
/// For payloads, we use bincode to serialize and deserialize to bytes.
///

pub struct Request {
    target: Option<Address>,
    inherit: bool,
    timeout: Option<u64>,
    ipc: Option<Vec<u8>>,
    metadata: Option<String>,
    payload: Option<Payload>,
    context: Option<Vec<u8>>,
}

impl Request {
    pub fn new() -> Self {
        Request {
            target: None,
            inherit: false,
            timeout: None,
            ipc: None,
            metadata: None,
            payload: None,
            context: None,
        }
    }

    pub fn target<T: IntoAddress>(mut self, target: T) -> Result<Self, AddressParseError> {
        self.target = Some(target.into_address()?);
        Ok(self)
    }

    pub fn inherit(mut self, inherit: bool) -> Self {
        self.inherit = inherit;
        self
    }

    pub fn expects_response(mut self, timeout: u64) -> Self {
        self.timeout = Some(timeout);
        self
    }

    pub fn ipc_bytes(mut self, ipc: Vec<u8>) -> Self {
        self.ipc = Some(ipc);
        self
    }

    pub fn ipc<T, F>(mut self, ipc: &T, serializer: F) -> anyhow::Result<Self>
    where
        F: Fn(&T) -> anyhow::Result<Vec<u8>>,
    {
        self.ipc = Some(serializer(ipc)?);
        Ok(self)
    }

    pub fn ipc_serde<T>(mut self, ipc: T) -> anyhow::Result<Self>
    where
        T: serde::Serialize,
    {
        self.ipc = Some(serde_json::to_vec(&ipc)?);
        Ok(self)
    }

    pub fn metadata(mut self, metadata: String) -> Self {
        self.metadata = Some(metadata);
        self
    }

    pub fn metadata_serde<T>(mut self, metadata: T) -> anyhow::Result<Self>
    where
        T: serde::Serialize,
    {
        self.metadata = Some(serde_json::to_string(&metadata)?);
        Ok(self)
    }

    pub fn payload(mut self, payload: Payload) -> Self {
        self.payload = Some(payload);
        self
    }

    pub fn payload_mime(mut self, mime: String) -> Self {
        if self.payload.is_none() {
            self.payload = Some(Payload {
                mime: Some(mime),
                bytes: vec![],
            });
            self
        } else {
            self.payload = Some(Payload {
                mime: Some(mime),
                bytes: self.payload.unwrap().bytes,
            });
            self
        }
    }

    pub fn payload_bytes(mut self, bytes: Vec<u8>) -> Self {
        if self.payload.is_none() {
            self.payload = Some(Payload { mime: None, bytes });
            self
        } else {
            self.payload = Some(Payload {
                mime: self.payload.unwrap().mime,
                bytes,
            });
            self
        }
    }

    pub fn payload_serde<T>(mut self, bytes: T) -> anyhow::Result<Self>
    where
        T: serde::Serialize,
    {
        if self.payload.is_none() {
            self.payload = Some(Payload { mime: None, bytes: serde_json::to_vec(&bytes)? });
            Ok(self)
        } else {
            self.payload = Some(Payload {
                mime: self.payload.unwrap().mime,
                bytes: serde_json::to_vec(&bytes)?,
            });
            Ok(self)
        }
    }

    pub fn context_bytes(mut self, context: Vec<u8>) -> Self {
        self.context = Some(context);
        self
    }

    pub fn context<T, F>(mut self, context: &T, serializer: F) -> anyhow::Result<Self>
    where
        F: Fn(&T) -> anyhow::Result<Vec<u8>>,
    {
        self.context = Some(serializer(context)?);
        Ok(self)
    }

    pub fn context_serde<T>(mut self, context: T) -> anyhow::Result<Self>
    where
        T: serde::Serialize,
    {
        self.context = Some(serde_json::to_vec(&context)?);
        Ok(self)
    }

    pub fn send(self) -> anyhow::Result<()> {
        if let (Some(target), Some(ipc)) = (self.target, self.ipc) {
            crate::send_request(
                &target,
                &wit::Request {
                    inherit: self.inherit,
                    expects_response: self.timeout,
                    ipc,
                    metadata: self.metadata,
                },
                self.context.as_ref(),
                self.payload.as_ref(),
            );
            Ok(())
        } else {
            Err(anyhow::anyhow!("missing fields"))
        }
    }

    pub fn send_and_await_response(self, timeout: u64) -> anyhow::Result<Result<(Address, Message), SendError>> {
        if let (Some(target), Some(ipc)) = (self.target, self.ipc) {
            Ok(crate::send_and_await_response(
                &target,
                &wit::Request {
                    inherit: self.inherit,
                    expects_response: Some(timeout),
                    ipc,
                    metadata: self.metadata,
                },
                self.payload.as_ref(),
            ))
        } else {
            Err(anyhow::anyhow!("missing fields"))
        }
    }

    pub fn send_and_await_response_unpack(self, timeout: u64) -> anyhow::Result<Result<(Address, wit::Response, Option<Context>), SendError>> {
        match self.send_and_await_response(timeout)? {
            Err(e) => Ok(Err(e)),
            Ok((source, message)) => {
                let Message::Response((response, context)) = message else {
                    return Err(anyhow::anyhow!("did not receive Response"));
                };
                Ok(Ok((source, response, context)))
            },
        }
    }
}

pub struct Response {
    inherit: bool,
    ipc: Option<Vec<u8>>,
    metadata: Option<String>,
    payload: Option<Payload>,
}

impl Response {
    pub fn new() -> Self {
        Response {
            inherit: false,
            ipc: None,
            metadata: None,
            payload: None,
        }
    }

    pub fn inherit(mut self, inherit: bool) -> Self {
        self.inherit = inherit;
        self
    }

    pub fn ipc_bytes(mut self, ipc: Vec<u8>) -> Self {
        self.ipc = Some(ipc);
        self
    }

    pub fn ipc<T, F>(mut self, ipc: &T, serializer: F) -> anyhow::Result<Self>
    where
        F: Fn(&T) -> anyhow::Result<Vec<u8>>,
    {
        self.ipc = Some(serializer(ipc)?);
        Ok(self)
    }

    pub fn ipc_serde<T>(mut self, ipc: T) -> anyhow::Result<Self>
    where
        T: serde::Serialize,
    {
        self.ipc = Some(serde_json::to_vec(&ipc)?);
        Ok(self)
    }

    pub fn metadata(mut self, metadata: Option<String>) -> Self {
        self.metadata = metadata;
        self
    }

    pub fn payload(mut self, payload: Payload) -> Self {
        self.payload = Some(payload);
        self
    }

    pub fn payload_mime(mut self, mime: String) -> Self {
        if self.payload.is_none() {
            self.payload = Some(Payload {
                mime: Some(mime),
                bytes: vec![],
            });
            self
        } else {
            self.payload = Some(Payload {
                mime: Some(mime),
                bytes: self.payload.unwrap().bytes,
            });
            self
        }
    }

    pub fn payload_bytes(mut self, bytes: Vec<u8>) -> Self {
        if self.payload.is_none() {
            self.payload = Some(Payload { mime: None, bytes });
            self
        } else {
            self.payload = Some(Payload {
                mime: self.payload.unwrap().mime,
                bytes,
            });
            self
        }
    }

    pub fn payload_serde<T>(mut self, bytes: T) -> anyhow::Result<Self>
    where
        T: serde::Serialize,
    {
        if self.payload.is_none() {
            self.payload = Some(Payload { mime: None, bytes: serde_json::to_vec(&bytes)? });
            Ok(self)
        } else {
            self.payload = Some(Payload {
                mime: self.payload.unwrap().mime,
                bytes: serde_json::to_vec(&bytes)?,
            });
            Ok(self)
        }
    }

    pub fn send(self) -> anyhow::Result<()> {
        if let Some(ipc) = self.ipc {
            crate::send_response(
                &wit::Response {
                    inherit: self.inherit,
                    ipc,
                    metadata: self.metadata,
                },
                self.payload.as_ref(),
            );
            Ok(())
        } else {
            Err(anyhow::anyhow!("missing IPC"))
        }
    }
}

pub fn make_payload<T, F>(payload: &T, serializer: F) -> anyhow::Result<Payload>
where
    F: Fn(&T) -> anyhow::Result<Vec<u8>>,
{
    Ok(Payload {
        mime: None,
        bytes: serializer(payload)?,
    })
}

pub fn make_payload_serde<T>(payload: &T) -> anyhow::Result<Payload>
where
    T: serde::Serialize,
{
    Ok(Payload {
        mime: None,
        bytes: serde_json::to_vec(payload)?,
    })
}

pub fn get_typed_payload<T, F>(deserializer: F) -> Option<T>
where
    F: Fn(&[u8]) -> anyhow::Result<T>,
{
    match crate::get_payload() {
        Some(payload) => match deserializer(&payload.bytes) {
            Ok(thing) => Some(thing),
            Err(_) => None,
        },
        None => None,
    }
}

pub fn get_typed_payload_serde<T>() -> Option<T>
where
    T: serde::de::DeserializeOwned,
{
    match crate::get_payload() {
        Some(payload) => match serde_json::from_slice(&payload.bytes) {
            Ok(thing) => Some(thing),
            Err(_) => None,
        },
        None => None,
    }
}

pub fn get_typed_state<T, F>(deserializer: F) -> Option<T>
where
    F: Fn(&[u8]) -> anyhow::Result<T>,
{
    match crate::get_state() {
        Some(bytes) => match deserializer(&bytes) {
            Ok(thing) => Some(thing),
            Err(_) => None,
        },
        None => None,
    }
}

pub fn get_typed_state_serde<T>() -> Option<T>
where
    T: serde::de::DeserializeOwned,
{
    match crate::get_state() {
        Some(bytes) => match serde_json::from_slice(&bytes) {
            Ok(thing) => Some(thing),
            Err(_) => None,
        },
        None => None,
    }
}

pub fn grant_messaging(our: &Address, grant_to: &Vec<ProcessId>) -> anyhow::Result<()> {
    let Some(our_messaging_cap) = crate::get_capability(our, &"\"messaging\"".into()) else {
        // the kernel will always give us this capability, so this should never happen
        return Err(anyhow::anyhow!(
            "failed to get our own messaging capability!"
        ));
    };
    for process in grant_to {
        crate::share_capability(&process, &our_messaging_cap);
    }
    Ok(())
}

pub fn can_message(address: &Address) -> bool {
    crate::get_capability(address, &"\"messaging\"".into()).is_some()
}

///
/// Here, we define types used by various Uqbar runtime components. Use these
/// to interface directly with the kernel, filesystem, virtual filesystem,
/// and other components -- if you have the capability to do so.
///

#[derive(Serialize, Deserialize, Debug)]
pub enum FsAction {
    Write,
    Replace(u128),
    Append(Option<u128>),
    Read(u128),
    ReadChunk(ReadChunkRequest),
    Delete(u128),
    Length(u128),
    //  process state management
    GetState,
    SetState,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReadChunkRequest {
    pub file_uuid: u128,
    pub start: u64,
    pub length: u64,
}

use super::{FileMetadata, SeekFrom, VfsAction, VfsRequest, VfsResponse};
use crate::{get_payload, Message, PackageId, Request};

/// Vfs helper struct for a file.
/// Opening or creating a file will give you a Result<File>.
/// You can call it's impl functions to interact with it.
pub struct File {
    pub path: String,
}

impl File {
    /// Reads the entire file, from start position.
    /// Returns a vector of bytes.
    pub fn read(&self) -> anyhow::Result<Vec<u8>> {
        let request = VfsRequest {
            path: self.path.clone(),
            action: VfsAction::Read,
        };
        let message = Request::new()
            .target(("our", "vfs", "sys", "uqbar"))
            .ipc(serde_json::to_vec(&request)?)
            .send_and_await_response(5)?;

        match message {
            Ok(Message::Response { ipc, .. }) => {
                let response = serde_json::from_slice::<VfsResponse>(&ipc)?;
                match response {
                    VfsResponse::Read => {
                        let data = match get_payload() {
                            Some(bytes) => bytes.bytes,
                            None => return Err(anyhow::anyhow!("vfs: no read payload")),
                        };
                        Ok(data)
                    }
                    VfsResponse::Err(e) => Err(e.into()),
                    _ => Err(anyhow::anyhow!("vfs: unexpected response: {:?}", response)),
                }
            }
            _ => Err(anyhow::anyhow!("vfs: unexpected message: {:?}", message)),
        }
    }

    /// Reads the entire file, from start position, into buffer.
    /// Returns the amount of bytes read.
    pub fn read_into(&self, buffer: &mut [u8]) -> anyhow::Result<usize> {
        let request = VfsRequest {
            path: self.path.clone(),
            action: VfsAction::Read,
        };
        let message = Request::new()
            .target(("our", "vfs", "sys", "uqbar"))
            .ipc(serde_json::to_vec(&request)?)
            .send_and_await_response(5)?;

        match message {
            Ok(Message::Response { ipc, .. }) => {
                let response = serde_json::from_slice::<VfsResponse>(&ipc)?;
                match response {
                    VfsResponse::Read => {
                        let data = match get_payload() {
                            Some(bytes) => bytes.bytes,
                            None => return Err(anyhow::anyhow!("vfs: no read payload")),
                        };
                        let len = std::cmp::min(data.len(), buffer.len());
                        buffer[..len].copy_from_slice(&data[..len]);
                        Ok(len)
                    }
                    VfsResponse::Err(e) => Err(e.into()),
                    _ => Err(anyhow::anyhow!("vfs: unexpected response: {:?}", response)),
                }
            }
            _ => Err(anyhow::anyhow!("vfs: unexpected message: {:?}", message)),
        }
    }

    /// Read into buffer from current cursor position
    /// Returns the amount of bytes read.
    pub fn read_at(&self, buffer: &mut [u8]) -> anyhow::Result<usize> {
        let length = buffer.len();
        let request = VfsRequest {
            path: self.path.clone(),
            action: VfsAction::ReadExact(length as u64),
        };
        let message = Request::new()
            .target(("our", "vfs", "sys", "uqbar"))
            .ipc(serde_json::to_vec(&request)?)
            .send_and_await_response(5)?;

        match message {
            Ok(Message::Response { ipc, .. }) => {
                let response = serde_json::from_slice::<VfsResponse>(&ipc)?;
                match response {
                    VfsResponse::Read => {
                        let data = match get_payload() {
                            Some(bytes) => bytes.bytes,
                            None => return Err(anyhow::anyhow!("vfs: no read payload")),
                        };
                        let len = std::cmp::min(data.len(), buffer.len());
                        buffer[..len].copy_from_slice(&data[..len]);
                        Ok(len)
                    }
                    VfsResponse::Err(e) => Err(e.into()),
                    _ => Err(anyhow::anyhow!("vfs: unexpected response: {:?}", response)),
                }
            }
            _ => Err(anyhow::anyhow!("vfs: unexpected message: {:?}", message)),
        }
    }

    /// Write entire slice as the new file.
    /// Truncates anything that existed at path before.
    pub fn write(&self, buffer: &[u8]) -> anyhow::Result<()> {
        let request = VfsRequest {
            path: self.path.clone(),
            action: VfsAction::Write,
        };

        let message = Request::new()
            .target(("our", "vfs", "sys", "uqbar"))
            .ipc(serde_json::to_vec(&request)?)
            .payload_bytes(buffer)
            .send_and_await_response(5)?;

        match message {
            Ok(Message::Response { ipc, .. }) => {
                let response = serde_json::from_slice::<VfsResponse>(&ipc)?;
                match response {
                    VfsResponse::Ok => Ok(()),
                    VfsResponse::Err(e) => Err(e.into()),
                    _ => Err(anyhow::anyhow!("vfs: unexpected response: {:?}", response)),
                }
            }
            _ => Err(anyhow::anyhow!("vfs: unexpected message: {:?}", message)),
        }
    }

    /// Write buffer to file at current position, overwriting any existing data.
    pub fn write_at(&mut self, buffer: &[u8]) -> anyhow::Result<()> {
        let request = VfsRequest {
            path: self.path.clone(),
            action: VfsAction::WriteAt,
        };
        let message = Request::new()
            .target(("our", "vfs", "sys", "uqbar"))
            .ipc(serde_json::to_vec(&request)?)
            .payload_bytes(buffer)
            .send_and_await_response(5)?;

        match message {
            Ok(Message::Response { ipc, .. }) => {
                let response = serde_json::from_slice::<VfsResponse>(&ipc)?;
                match response {
                    VfsResponse::Ok => Ok(()),
                    VfsResponse::Err(e) => Err(e.into()),
                    _ => Err(anyhow::anyhow!("vfs: unexpected response: {:?}", response)),
                }
            }
            _ => Err(anyhow::anyhow!("vfs: unexpected message: {:?}", message)),
        }
    }

    /// Seek file to position.
    /// Returns the new position.
    pub fn seek(&mut self, pos: SeekFrom) -> anyhow::Result<u64> {
        let request = VfsRequest {
            path: self.path.clone(),
            action: VfsAction::Seek { seek_from: pos },
        };
        let message = Request::new()
            .target(("our", "vfs", "sys", "uqbar"))
            .ipc(serde_json::to_vec(&request)?)
            .send_and_await_response(5)?;

        match message {
            Ok(Message::Response { ipc, .. }) => {
                let response = serde_json::from_slice::<VfsResponse>(&ipc)?;
                match response {
                    VfsResponse::SeekFrom(new_pos) => Ok(new_pos),
                    VfsResponse::Err(e) => Err(e.into()),
                    _ => Err(anyhow::anyhow!("vfs: unexpected response: {:?}", response)),
                }
            }
            _ => Err(anyhow::anyhow!("vfs: unexpected message: {:?}", message)),
        }
    }

    /// Set file length, if given size > underlying file, fills it with 0s.
    pub fn set_len(&mut self, size: u64) -> anyhow::Result<()> {
        let request = VfsRequest {
            path: self.path.clone(),
            action: VfsAction::SetLen(size),
        };
        let message = Request::new()
            .target(("our", "vfs", "sys", "uqbar"))
            .ipc(serde_json::to_vec(&request)?)
            .send_and_await_response(5)?;

        match message {
            Ok(Message::Response { ipc, .. }) => {
                let response = serde_json::from_slice::<VfsResponse>(&ipc)?;
                match response {
                    VfsResponse::Ok => Ok(()),
                    VfsResponse::Err(e) => Err(e.into()),
                    _ => Err(anyhow::anyhow!("vfs: unexpected response: {:?}", response)),
                }
            }
            _ => Err(anyhow::anyhow!("vfs: unexpected message: {:?}", message)),
        }
    }

    /// Metadata of a path, returns file type and length.
    pub fn metadata(&self) -> anyhow::Result<FileMetadata> {
        let request = VfsRequest {
            path: self.path.clone(),
            action: VfsAction::Metadata,
        };
        let message = Request::new()
            .target(("our", "vfs", "sys", "uqbar"))
            .ipc(serde_json::to_vec(&request)?)
            .send_and_await_response(5)?;

        match message {
            Ok(Message::Response { ipc, .. }) => {
                let response = serde_json::from_slice::<VfsResponse>(&ipc)?;
                match response {
                    VfsResponse::Metadata(metadata) => Ok(metadata),
                    VfsResponse::Err(e) => Err(e.into()),
                    _ => Err(anyhow::anyhow!("vfs: unexpected response: {:?}", response)),
                }
            }
            _ => Err(anyhow::anyhow!("vfs: unexpected message: {:?}", message)),
        }
    }

    /// Syncs path file buffers to disk.
    pub fn sync_all(&self) -> anyhow::Result<()> {
        let request = VfsRequest {
            path: self.path.clone(),
            action: VfsAction::SyncAll,
        };
        let message = Request::new()
            .target(("our", "vfs", "sys", "uqbar"))
            .ipc(serde_json::to_vec(&request)?)
            .send_and_await_response(5)?;

        match message {
            Ok(Message::Response { ipc, .. }) => {
                let response = serde_json::from_slice::<VfsResponse>(&ipc)?;
                match response {
                    VfsResponse::Ok => Ok(()),
                    VfsResponse::Err(e) => Err(e.into()),
                    _ => Err(anyhow::anyhow!("vfs: unexpected response: {:?}", response)),
                }
            }
            _ => Err(anyhow::anyhow!("vfs: unexpected message: {:?}", message)),
        }
    }
}

/// Creates a drive with path "/package_id/drive", gives you read and write caps.
/// Will only work on the same package_id as you're calling it from, unless you
/// have root capabilities.
pub fn create_drive(package_id: PackageId, drive: &str) -> anyhow::Result<String> {
    let path = format!("/{}/{}", package_id, drive);
    let res = Request::new()
        .target(("our", "vfs", "sys", "uqbar"))
        .ipc(serde_json::to_vec(&VfsRequest {
            path: path.clone(),
            action: VfsAction::CreateDrive,
        })?)
        .send_and_await_response(5)?;

    match res {
        Ok(Message::Response { ipc, .. }) => {
            let response = serde_json::from_slice::<VfsResponse>(&ipc)?;
            match response {
                VfsResponse::Ok => Ok(path),
                VfsResponse::Err(e) => Err(e.into()),
                _ => Err(anyhow::anyhow!("vfs: unexpected response: {:?}", response)),
            }
        }
        _ => return Err(anyhow::anyhow!("vfs: unexpected message: {:?}", res)),
    }
}

/// Opens a file at path, if no file at path, creates one if boolean create is true.
pub fn open_file(path: &str, create: bool) -> anyhow::Result<File> {
    let request = VfsRequest {
        path: path.to_string(),
        action: VfsAction::OpenFile { create },
    };

    let message = Request::new()
        .target(("our", "vfs", "sys", "uqbar"))
        .ipc(serde_json::to_vec(&request)?)
        .send_and_await_response(5)?;

    match message {
        Ok(Message::Response { ipc, .. }) => {
            let response = serde_json::from_slice::<VfsResponse>(&ipc)?;
            match response {
                VfsResponse::Ok => Ok(File {
                    path: path.to_string(),
                }),
                VfsResponse::Err(e) => Err(e.into()),
                _ => Err(anyhow::anyhow!("vfs: unexpected response: {:?}", response)),
            }
        }
        _ => Err(anyhow::anyhow!("vfs: unexpected message: {:?}", message)),
    }
}

/// Creates a file at path, if file found at path, truncates it to 0.
pub fn create_file(path: &str) -> anyhow::Result<File> {
    let request = VfsRequest {
        path: path.to_string(),
        action: VfsAction::CreateFile,
    };

    let message = Request::new()
        .target(("our", "vfs", "sys", "uqbar"))
        .ipc(serde_json::to_vec(&request)?)
        .send_and_await_response(5)?;

    match message {
        Ok(Message::Response { ipc, .. }) => {
            let response = serde_json::from_slice::<VfsResponse>(&ipc)?;
            match response {
                VfsResponse::Ok => Ok(File {
                    path: path.to_string(),
                }),
                VfsResponse::Err(e) => Err(e.into()),
                _ => Err(anyhow::anyhow!("vfs: unexpected response: {:?}", response)),
            }
        }
        _ => Err(anyhow::anyhow!("vfs: unexpected message: {:?}", message)),
    }
}
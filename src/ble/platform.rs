//! Platform-independent BLE transport interface.
//!
//! The BSC200 protocol layer only needs a small part of the BLE stack:
//! write to one of four data characteristics, write to the control
//! characteristic, subscribe to notifications and optionally read the
//! standard battery level. Each OS backend implements [`Link`].

use std::sync::Arc;

use tokio::sync::{mpsc, oneshot, Mutex};

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
pub use linux::{connect, pairing_hint, platform_name, scan, PairingAgent};

#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
pub use windows::{connect, pairing_hint, platform_name, scan, PairingAgent};

#[cfg(not(any(target_os = "linux", target_os = "windows")))]
compile_error!("The BSC200 BLE backend is implemented for Linux and Windows only.");

/// Completion callback for a pairing confirmation prompt.
pub type PairingPending = Arc<Mutex<Option<oneshot::Sender<bool>>>>;

/// A connected BLE link with the four BSC200 UART channels.
#[async_trait::async_trait]
pub trait Link: Send + Sync {
    /// Write a chunk to a data channel (1, 3 or 4).
    async fn write_data(&self, channel: u8, data: &[u8]) -> Result<(), String>;
    /// Write to the control channel (2).
    async fn write_control(&self, data: &[u8]) -> Result<(), String>;
    /// Read the standard Battery Level characteristic, if present.
    async fn read_battery(&self) -> Result<Option<u8>, String>;
    /// Disconnect the peripheral.
    async fn disconnect(&self) -> Result<(), String>;
    /// Maximum payload size for one BLE write.
    fn max_chunk(&self) -> usize;
    /// Whether a data channel is available.
    fn has_channel(&self, channel: u8) -> bool;
}

pub struct ConnectedLink {
    pub link: Box<dyn Link>,
    pub name: String,
    pub notifications: mpsc::UnboundedReceiver<(u8, Vec<u8>)>,
}

/// Human-readable backend name for logs and the GUI.
pub fn backend_name() -> &'static str {
    platform_name()
}

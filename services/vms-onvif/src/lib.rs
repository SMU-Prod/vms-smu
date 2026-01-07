//! VMS ONVIF Library
//! Cliente ONVIF com HTTP Digest Authentication

pub mod digest_auth;
pub mod client;
pub mod discovery;
pub mod device;
pub mod camera;
pub mod xml_utils;

pub use client::{OnvifClient, OnvifError};
pub use discovery::OnvifDiscovery;
pub use device::OnvifDevice;
pub use camera::{Camera, CameraProfile};

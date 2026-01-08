//! VMS Core - Shared types, DTOs, and utilities
//! 
//! This crate contains all shared types used across the VMS Enterprise system:
//! - Control Plane (vms_server)
//! - Data Plane (vms_node)
//! - Desktop Apps (vms_viewer, vms_admin)

pub mod auth;
pub mod camera;
pub mod node;
pub mod session;
pub mod error;
pub mod webrtc_types;

pub use auth::*;
pub use camera::*;
pub use node::*;
pub use session::*;
pub use error::*;
pub use webrtc_types::*;


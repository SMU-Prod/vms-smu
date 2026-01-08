//! Orchestrator - commands to nodes

use vms_core::{NodeCommand, LiveSession};
use uuid::Uuid;

pub struct Orchestrator;

impl Orchestrator {
    /// Send command to node
    pub async fn send_command(&self, _node_id: Uuid, _cmd: NodeCommand) -> anyhow::Result<()> {
        // TODO: Send via WebSocket/gRPC to node
        Ok(())
    }

    /// Start live session
    pub async fn start_live(&self, _session: &LiveSession) -> anyhow::Result<String> {
        // TODO: 
        // 1. Get node for camera
        // 2. Send START_LIVE command
        // 3. Wait for node response with stream URL
        // 4. Return signed URL
        Ok("http://node:8080/live/session-id/index.m3u8".to_string())
    }

    /// Stop live session
    pub async fn stop_live(&self, session_id: Uuid) -> anyhow::Result<()> {
        // TODO: Send STOP_LIVE to node
        tracing::info!("Stopping session {}", session_id);
        Ok(())
    }
}

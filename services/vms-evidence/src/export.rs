//! Evidence export functionality

use super::evidence::Evidence;
use anyhow::{Context, Result};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::io::Write;
use uuid::Uuid;

/// Export format
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ExportFormat {
    /// ZIP archive with files + metadata
    Zip,
    /// JSON metadata only
    Json,
    /// PDF report
    Pdf,
}

/// Export request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportRequest {
    /// Evidence ID
    pub evidence_id: Uuid,
    /// Export format
    pub format: ExportFormat,
    /// Include attachments
    pub include_attachments: bool,
    /// Include custody chain
    pub include_custody_chain: bool,
    /// Password protect (optional)
    pub password: Option<String>,
}

/// Export result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportResult {
    /// Export ID
    pub export_id: Uuid,
    /// File path
    pub file_path: String,
    /// File size (bytes)
    pub file_size: u64,
    /// SHA256 hash
    pub sha256: String,
    /// Created at
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Expires at
    pub expires_at: chrono::DateTime<chrono::Utc>,
}

/// Evidence exporter
pub struct EvidenceExporter {
    export_dir: String,
}

impl EvidenceExporter {
    /// Create new exporter
    pub fn new(export_dir: String) -> Self {
        Self { export_dir }
    }

    /// Export evidence
    pub async fn export(&self, evidence: &Evidence, request: &ExportRequest) -> Result<ExportResult> {
        match request.format {
            ExportFormat::Zip => self.export_zip(evidence, request).await,
            ExportFormat::Json => self.export_json(evidence, request).await,
            ExportFormat::Pdf => self.export_pdf(evidence, request).await,
        }
    }

    /// Export as ZIP
    async fn export_zip(&self, evidence: &Evidence, request: &ExportRequest) -> Result<ExportResult> {
        let export_id = Uuid::new_v4();
        let file_name = format!("{}-{}.zip", evidence.case_number, export_id);
        let file_path = format!("{}/{}", self.export_dir, file_name);

        // Create ZIP file
        let file = std::fs::File::create(&file_path)
            .context("Failed to create ZIP file")?;
        let mut zip = zip::ZipWriter::new(file);

        // Add metadata.json
        let options = zip::write::FileOptions::default()
            .compression_method(zip::CompressionMethod::Deflated);

        zip.start_file("metadata.json", options)?;
        let metadata = self.create_metadata_json(evidence, request.include_custody_chain);
        zip.write_all(metadata.as_bytes())?;

        // Add README.txt
        zip.start_file("README.txt", options)?;
        let readme = self.create_readme(evidence);
        zip.write_all(readme.as_bytes())?;

        // Add attachments (if requested)
        if request.include_attachments {
            for attachment in &evidence.attachments {
                // TODO: Copy actual files
                // For now, just add placeholder
                let attach_path = format!("attachments/{}", attachment.file_name);
                zip.start_file(&attach_path, options)?;
                zip.write_all(b"[File content would be here]")?;
            }
        }

        zip.finish()?;

        // Calculate file size and hash
        let metadata = std::fs::metadata(&file_path)?;
        let file_size = metadata.len();
        let sha256 = self.calculate_sha256(&file_path)?;

        Ok(ExportResult {
            export_id,
            file_path,
            file_size,
            sha256,
            created_at: Utc::now(),
            expires_at: Utc::now() + chrono::Duration::days(7),
        })
    }

    /// Export as JSON
    async fn export_json(&self, evidence: &Evidence, request: &ExportRequest) -> Result<ExportResult> {
        let export_id = Uuid::new_v4();
        let file_name = format!("{}-{}.json", evidence.case_number, export_id);
        let file_path = format!("{}/{}", self.export_dir, file_name);

        let metadata = self.create_metadata_json(evidence, request.include_custody_chain);
        std::fs::write(&file_path, metadata)?;

        let metadata = std::fs::metadata(&file_path)?;
        let file_size = metadata.len();
        let sha256 = self.calculate_sha256(&file_path)?;

        Ok(ExportResult {
            export_id,
            file_path,
            file_size,
            sha256,
            created_at: Utc::now(),
            expires_at: Utc::now() + chrono::Duration::days(7),
        })
    }

    /// Export as PDF
    async fn export_pdf(&self, evidence: &Evidence, _request: &ExportRequest) -> Result<ExportResult> {
        let export_id = Uuid::new_v4();
        let file_name = format!("{}-{}.pdf", evidence.case_number, export_id);
        let file_path = format!("{}/{}", self.export_dir, file_name);

        // TODO: Generate PDF report
        // For now, create a placeholder
        std::fs::write(&file_path, b"[PDF report would be here]")?;

        let metadata = std::fs::metadata(&file_path)?;
        let file_size = metadata.len();
        let sha256 = self.calculate_sha256(&file_path)?;

        Ok(ExportResult {
            export_id,
            file_path,
            file_size,
            sha256,
            created_at: Utc::now(),
            expires_at: Utc::now() + chrono::Duration::days(7),
        })
    }

    /// Create metadata JSON
    fn create_metadata_json(&self, evidence: &Evidence, include_custody: bool) -> String {
        let mut metadata = serde_json::json!({
            "case_number": evidence.case_number,
            "title": evidence.title,
            "description": evidence.description,
            "evidence_type": evidence.evidence_type,
            "priority": evidence.priority,
            "status": evidence.status,
            "tags": evidence.tags,
            "created_at": evidence.created_at,
            "updated_at": evidence.updated_at,
            "attachments": evidence.attachments.iter().map(|a| serde_json::json!({
                "file_name": a.file_name,
                "type": a.attachment_type,
                "size": a.file_size,
                "sha256": a.sha256,
                "camera_id": a.camera_id,
                "start_time": a.start_time,
                "end_time": a.end_time,
            })).collect::<Vec<_>>(),
        });

        if include_custody {
            metadata["custody_chain"] = serde_json::json!(evidence.custody_chain);
        }

        serde_json::to_string_pretty(&metadata).unwrap()
    }

    /// Create README
    fn create_readme(&self, evidence: &Evidence) -> String {
        format!(
            r#"EVIDENCE EXPORT
==============

Case Number: {}
Title: {}
Description: {}
Status: {:?}
Priority: {:?}

Created: {}
Exported: {}

This export contains evidence from the VMS Evidence Management System.
All files included in this archive are part of the official evidence record.

Chain of custody has been maintained and is included in metadata.json.

For questions or verification, please contact the system administrator.
"#,
            evidence.case_number,
            evidence.title,
            evidence.description,
            evidence.status,
            evidence.priority,
            evidence.created_at.format("%Y-%m-%d %H:%M:%S UTC"),
            Utc::now().format("%Y-%m-%d %H:%M:%S UTC"),
        )
    }

    /// Calculate SHA256 hash of file
    fn calculate_sha256(&self, file_path: &str) -> Result<String> {
        use sha2::{Digest, Sha256};

        let contents = std::fs::read(file_path)?;
        let hash = Sha256::digest(&contents);
        Ok(format!("{:x}", hash))
    }
}

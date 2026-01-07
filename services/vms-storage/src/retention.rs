//! Sistema de retenção de arquivos

use anyhow::Result;
use chrono::{Duration, Utc};
use std::fs;
use std::path::Path;
use tracing::{info, warn};

pub struct RetentionManager {
    base_path: String,
    retention_days: u32,
}

impl RetentionManager {
    pub fn new(base_path: String, retention_days: u32) -> Self {
        Self {
            base_path,
            retention_days,
        }
    }

    /// Executa limpeza de arquivos antigos
    pub async fn cleanup(&self) -> Result<()> {
        let cutoff_date = Utc::now() - Duration::days(self.retention_days as i64);
        let cameras_path = Path::new(&self.base_path).join("cameras");

        if !cameras_path.exists() {
            return Ok(());
        }

        let mut total_deleted = 0;
        let mut total_bytes_freed = 0u64;

        // Iterar por câmeras
        for camera_entry in fs::read_dir(&cameras_path)? {
            let camera_dir = camera_entry?;
            if !camera_dir.file_type()?.is_dir() {
                continue;
            }

            // Iterar por datas
            for date_entry in fs::read_dir(camera_dir.path())? {
                let date_dir = date_entry?;
                let date_name = date_dir.file_name();
                let date_str = date_name.to_string_lossy();

                // Parse data do nome do diretório (YYYY-MM-DD)
                if let Ok(dir_date) = chrono::NaiveDate::parse_from_str(&date_str, "%Y-%m-%d") {
                    let dir_datetime = dir_date.and_hms_opt(0, 0, 0).unwrap().and_utc();

                    if dir_datetime < cutoff_date {
                        // Calcular tamanho do diretório
                        let dir_size = Self::calculate_dir_size(&date_dir.path())?;

                        // Deletar diretório
                        fs::remove_dir_all(&date_dir.path())?;
                        total_deleted += 1;
                        total_bytes_freed += dir_size;

                        info!(
                            "Deleted old recordings: {} ({} MB)",
                            date_dir.path().display(),
                            dir_size / 1_000_000
                        );
                    }
                }
            }
        }

        if total_deleted > 0 {
            info!(
                "Retention cleanup: deleted {} directories, freed {} MB",
                total_deleted,
                total_bytes_freed / 1_000_000
            );
        }

        Ok(())
    }

    fn calculate_dir_size(path: &Path) -> Result<u64> {
        let mut total = 0;

        if path.is_dir() {
            for entry in fs::read_dir(path)? {
                let entry = entry?;
                let path = entry.path();

                if path.is_file() {
                    total += entry.metadata()?.len();
                } else if path.is_dir() {
                    total += Self::calculate_dir_size(&path)?;
                }
            }
        }

        Ok(total)
    }
}

# FASE 1: CORE FUNCIONAL - Plano de Implementação Detalhado

**Duração Estimada:** 4-6 semanas
**Objetivo:** Sistema VMS funcional para monitoramento e reprodução
**Meta de Completude:** 65% vs Digifort

---

## 📋 OVERVIEW DA FASE 1

A Fase 1 está dividida em 3 componentes principais que podem ser desenvolvidos em paralelo:

1. **Playback System** (vms-storage) - 2 semanas
2. **Multi-Streaming** (vms-ingest) - 2 semanas
3. **Frontend Básico** (clients/web) - 2 semanas

---

## 🎬 COMPONENTE 1: PLAYBACK SYSTEM (vms-storage)

### Objetivo
Permitir reprodução de gravações com controles completos de timeline, seek, export e bookmarks.

### Arquitetura Proposta

```
vms-storage/
├── src/
│   ├── playback/
│   │   ├── mod.rs           # Módulo principal
│   │   ├── timeline.rs      # Geração de timeline
│   │   ├── streamer.rs      # Streaming de gravação
│   │   ├── export.rs        # Exportação MP4/AVI
│   │   └── bookmark.rs      # Gerenciamento de bookmarks
│   ├── index/
│   │   ├── mod.rs           # Índice de gravações
│   │   └── seek.rs          # Seek rápido
│   └── routes/
│       └── playback.rs      # Rotas HTTP
```

---

### 1.1 Timeline API

**Endpoint:** `GET /api/v1/recordings/:camera_id/timeline`

**Query Params:**
- `start`: ISO 8601 timestamp início
- `end`: ISO 8601 timestamp fim
- `resolution`: Resolução da timeline (1s, 10s, 1m, 10m, 1h)

**Response:**
```json
{
  "camera_id": "uuid",
  "start": "2025-12-13T00:00:00Z",
  "end": "2025-12-13T23:59:59Z",
  "resolution": "1m",
  "segments": [
    {
      "start": "2025-12-13T00:00:00Z",
      "end": "2025-12-13T00:59:59Z",
      "file_path": "/storage/cam123/2025-12-13/video_00.mkv",
      "size_bytes": 1048576,
      "has_motion": true,
      "has_events": ["alarm_01", "motion_02"]
    }
  ],
  "motion_zones": [
    {
      "timestamp": "2025-12-13T00:15:30Z",
      "duration_ms": 5000,
      "confidence": 0.95
    }
  ],
  "events": [
    {
      "id": "evt_123",
      "timestamp": "2025-12-13T00:15:32Z",
      "type": "motion_detected",
      "priority": "high"
    }
  ],
  "bookmarks": [
    {
      "id": "bm_456",
      "timestamp": "2025-12-13T00:15:35Z",
      "user": "operator01",
      "note": "Pessoa suspeita detectada"
    }
  ]
}
```

**Implementação:**

```rust
// vms-storage/src/playback/timeline.rs

use chrono::{DateTime, Utc, Duration};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct Timeline {
    pub camera_id: String,
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
    pub resolution: TimelineResolution,
    pub segments: Vec<RecordingSegment>,
    pub motion_zones: Vec<MotionZone>,
    pub events: Vec<TimelineEvent>,
    pub bookmarks: Vec<Bookmark>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TimelineResolution {
    #[serde(rename = "1s")]
    OneSecond,
    #[serde(rename = "10s")]
    TenSeconds,
    #[serde(rename = "1m")]
    OneMinute,
    #[serde(rename = "10m")]
    TenMinutes,
    #[serde(rename = "1h")]
    OneHour,
}

#[derive(Debug, Serialize)]
pub struct RecordingSegment {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
    pub file_path: String,
    pub size_bytes: u64,
    pub has_motion: bool,
    pub has_events: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct MotionZone {
    pub timestamp: DateTime<Utc>,
    pub duration_ms: u64,
    pub confidence: f32,
}

#[derive(Debug, Serialize)]
pub struct TimelineEvent {
    pub id: String,
    pub timestamp: DateTime<Utc>,
    pub event_type: String,
    pub priority: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Bookmark {
    pub id: String,
    pub timestamp: DateTime<Utc>,
    pub user: String,
    pub note: String,
}

pub struct TimelineBuilder {
    db: rocksdb::DB,
}

impl TimelineBuilder {
    pub async fn build_timeline(
        &self,
        camera_id: &str,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
        resolution: TimelineResolution,
    ) -> anyhow::Result<Timeline> {
        // 1. Buscar segmentos de gravação no RocksDB
        let segments = self.find_segments(camera_id, start, end).await?;

        // 2. Buscar zonas de movimento
        let motion_zones = self.find_motion_zones(camera_id, start, end).await?;

        // 3. Buscar eventos
        let events = self.find_events(camera_id, start, end).await?;

        // 4. Buscar bookmarks
        let bookmarks = self.find_bookmarks(camera_id, start, end).await?;

        Ok(Timeline {
            camera_id: camera_id.to_string(),
            start,
            end,
            resolution,
            segments,
            motion_zones,
            events,
            bookmarks,
        })
    }

    async fn find_segments(
        &self,
        camera_id: &str,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> anyhow::Result<Vec<RecordingSegment>> {
        // Query RocksDB para encontrar arquivos de vídeo
        // Formato da chave: recordings:{camera_id}:{timestamp}
        let prefix = format!("recordings:{}:", camera_id);

        let mut segments = Vec::new();
        let iter = self.db.prefix_iterator(prefix.as_bytes());

        for item in iter {
            let (key, value) = item?;
            // Parse key/value e adiciona ao segments se estiver no range
            // TODO: implementar parsing
        }

        Ok(segments)
    }

    async fn find_motion_zones(
        &self,
        camera_id: &str,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> anyhow::Result<Vec<MotionZone>> {
        // Query RocksDB para motion events
        // Formato da chave: motion:{camera_id}:{timestamp}
        Ok(vec![])
    }

    async fn find_events(
        &self,
        camera_id: &str,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> anyhow::Result<Vec<TimelineEvent>> {
        // Query RocksDB para eventos
        Ok(vec![])
    }

    async fn find_bookmarks(
        &self,
        camera_id: &str,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> anyhow::Result<Vec<Bookmark>> {
        // Query RocksDB para bookmarks
        let prefix = format!("bookmarks:{}:", camera_id);
        Ok(vec![])
    }
}
```

**Rota Axum:**

```rust
// vms-storage/src/routes/playback.rs

use axum::{
    extract::{Path, Query},
    Json,
    response::IntoResponse,
};
use chrono::{DateTime, Utc};

#[derive(Deserialize)]
pub struct TimelineQuery {
    start: DateTime<Utc>,
    end: DateTime<Utc>,
    #[serde(default = "default_resolution")]
    resolution: TimelineResolution,
}

fn default_resolution() -> TimelineResolution {
    TimelineResolution::OneMinute
}

pub async fn get_timeline(
    Path(camera_id): Path<String>,
    Query(query): Query<TimelineQuery>,
) -> impl IntoResponse {
    // TODO: implementar
    Json(Timeline {
        camera_id,
        start: query.start,
        end: query.end,
        resolution: query.resolution,
        segments: vec![],
        motion_zones: vec![],
        events: vec![],
        bookmarks: vec![],
    })
}
```

---

### 1.2 Playback Streaming

**Endpoint:** `GET /api/v1/recordings/:camera_id/stream`

**Query Params:**
- `start`: ISO 8601 timestamp
- `end`: ISO 8601 timestamp (opcional)
- `speed`: Velocidade de reprodução (0.25, 0.5, 1.0, 2.0, 4.0)

**Response:** HTTP Streaming (chunked transfer)

**Content-Type:** `video/mp4` ou `application/octet-stream`

**Implementação:**

```rust
// vms-storage/src/playback/streamer.rs

use tokio::fs::File;
use tokio::io::AsyncReadExt;
use axum::body::StreamBody;
use futures::stream::Stream;

pub struct PlaybackStreamer {
    storage_path: String,
}

impl PlaybackStreamer {
    pub async fn stream_recording(
        &self,
        camera_id: &str,
        start: DateTime<Utc>,
        end: Option<DateTime<Utc>>,
        speed: f32,
    ) -> anyhow::Result<impl Stream<Item = Result<bytes::Bytes, std::io::Error>>> {
        // 1. Encontrar arquivos de vídeo que cobrem o período
        let files = self.find_recording_files(camera_id, start, end).await?;

        // 2. Criar stream que lê múltiplos arquivos sequencialmente
        let stream = self.create_multi_file_stream(files, start, end, speed).await?;

        Ok(stream)
    }

    async fn find_recording_files(
        &self,
        camera_id: &str,
        start: DateTime<Utc>,
        end: Option<DateTime<Utc>>,
    ) -> anyhow::Result<Vec<String>> {
        // Busca no filesystem
        // Formato: /storage/{camera_id}/{date}/video_{hour}.mkv
        Ok(vec![])
    }

    async fn create_multi_file_stream(
        &self,
        files: Vec<String>,
        start: DateTime<Utc>,
        end: Option<DateTime<Utc>>,
        speed: f32,
    ) -> anyhow::Result<impl Stream<Item = Result<bytes::Bytes, std::io::Error>>> {
        // Cria stream que:
        // 1. Abre cada arquivo sequencialmente
        // 2. Faz seek para o timestamp correto
        // 3. Lê chunks e envia para o cliente
        // 4. Aplica velocidade (se speed != 1.0)

        // TODO: implementar com async-stream
        todo!()
    }
}
```

**Rota Axum:**

```rust
use axum::response::Response;
use axum::body::Body;

#[derive(Deserialize)]
pub struct StreamQuery {
    start: DateTime<Utc>,
    end: Option<DateTime<Utc>>,
    #[serde(default = "default_speed")]
    speed: f32,
}

fn default_speed() -> f32 { 1.0 }

pub async fn stream_recording(
    Path(camera_id): Path<String>,
    Query(query): Query<StreamQuery>,
) -> Result<Response<Body>, StatusCode> {
    let streamer = PlaybackStreamer::new();

    let stream = streamer
        .stream_recording(&camera_id, query.start, query.end, query.speed)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let body = Body::from_stream(stream);

    Ok(Response::builder()
        .header("Content-Type", "video/mp4")
        .header("Cache-Control", "no-cache")
        .body(body)
        .unwrap())
}
```

---

### 1.3 Exportação de Vídeo

**Endpoint:** `POST /api/v1/recordings/export`

**Request Body:**
```json
{
  "camera_id": "uuid",
  "start": "2025-12-13T00:00:00Z",
  "end": "2025-12-13T01:00:00Z",
  "format": "mp4",
  "include_watermark": true,
  "include_metadata": true,
  "quality": "high"
}
```

**Response:**
```json
{
  "export_id": "exp_123",
  "status": "processing",
  "progress": 0,
  "estimated_size_mb": 250
}
```

**Status Endpoint:** `GET /api/v1/recordings/export/:export_id`

**Download:** `GET /api/v1/recordings/export/:export_id/download`

**Implementação:**

```rust
// vms-storage/src/playback/export.rs

use std::path::PathBuf;
use tokio::process::Command;

pub struct VideoExporter {
    export_path: PathBuf,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExportRequest {
    pub camera_id: String,
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
    pub format: ExportFormat,
    pub include_watermark: bool,
    pub include_metadata: bool,
    pub quality: ExportQuality,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ExportFormat {
    Mp4,
    Avi,
    Mkv,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ExportQuality {
    Low,
    Medium,
    High,
}

#[derive(Debug, Serialize)]
pub struct ExportJob {
    pub export_id: String,
    pub status: ExportStatus,
    pub progress: u8,
    pub estimated_size_mb: u64,
    pub output_path: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ExportStatus {
    Queued,
    Processing,
    Completed,
    Failed,
}

impl VideoExporter {
    pub async fn export_video(
        &self,
        request: ExportRequest,
    ) -> anyhow::Result<ExportJob> {
        let export_id = uuid::Uuid::new_v4().to_string();

        // Criar job
        let job = ExportJob {
            export_id: export_id.clone(),
            status: ExportStatus::Queued,
            progress: 0,
            estimated_size_mb: 0,
            output_path: None,
        };

        // Spawn tarefa em background
        tokio::spawn(async move {
            // Processar export usando FFmpeg
            self.process_export(export_id, request).await
        });

        Ok(job)
    }

    async fn process_export(
        &self,
        export_id: String,
        request: ExportRequest,
    ) -> anyhow::Result<()> {
        // 1. Encontrar arquivos de vídeo
        let input_files = self.find_files(&request).await?;

        // 2. Criar arquivo de concatenação para FFmpeg
        let concat_file = self.create_concat_file(&input_files)?;

        // 3. Construir comando FFmpeg
        let output_path = self.export_path.join(format!("{}.mp4", export_id));

        let mut cmd = Command::new("ffmpeg");
        cmd.arg("-f").arg("concat")
           .arg("-safe").arg("0")
           .arg("-i").arg(&concat_file)
           .arg("-c").arg("copy"); // Copy codec (rápido)

        // Adicionar watermark se solicitado
        if request.include_watermark {
            cmd.arg("-vf")
               .arg("drawtext=text='VMS Enterprise':x=10:y=10:fontsize=24:fontcolor=white");
        }

        // Qualidade
        match request.quality {
            ExportQuality::High => {
                cmd.arg("-crf").arg("18");
            }
            ExportQuality::Medium => {
                cmd.arg("-crf").arg("23");
            }
            ExportQuality::Low => {
                cmd.arg("-crf").arg("28");
            }
        }

        cmd.arg(&output_path);

        // 4. Executar FFmpeg
        let output = cmd.output().await?;

        if !output.status.success() {
            anyhow::bail!("FFmpeg failed: {:?}", output.stderr);
        }

        // 5. Atualizar status do job
        // TODO: salvar no RocksDB

        Ok(())
    }
}
```

---

### 1.4 Bookmarks

**Endpoints:**

- `POST /api/v1/bookmarks` - Criar bookmark
- `GET /api/v1/bookmarks?camera_id=&start=&end=` - Listar bookmarks
- `GET /api/v1/bookmarks/:id` - Obter bookmark
- `PUT /api/v1/bookmarks/:id` - Atualizar bookmark
- `DELETE /api/v1/bookmarks/:id` - Deletar bookmark

**Implementação:**

```rust
// vms-storage/src/playback/bookmark.rs

use rocksdb::DB;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Bookmark {
    pub id: String,
    pub camera_id: String,
    pub timestamp: DateTime<Utc>,
    pub user_id: String,
    pub note: String,
    pub tags: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub struct BookmarkManager {
    db: DB,
}

impl BookmarkManager {
    pub async fn create_bookmark(
        &self,
        camera_id: String,
        timestamp: DateTime<Utc>,
        user_id: String,
        note: String,
        tags: Vec<String>,
    ) -> anyhow::Result<Bookmark> {
        let bookmark = Bookmark {
            id: uuid::Uuid::new_v4().to_string(),
            camera_id: camera_id.clone(),
            timestamp,
            user_id,
            note,
            tags,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        // Salvar no RocksDB
        // Chave: bookmarks:{camera_id}:{timestamp}:{id}
        let key = format!("bookmarks:{}:{}:{}",
            camera_id,
            timestamp.timestamp(),
            bookmark.id
        );

        let value = serde_json::to_vec(&bookmark)?;
        self.db.put(key.as_bytes(), &value)?;

        Ok(bookmark)
    }

    pub async fn list_bookmarks(
        &self,
        camera_id: &str,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> anyhow::Result<Vec<Bookmark>> {
        let prefix = format!("bookmarks:{}:", camera_id);
        let mut bookmarks = Vec::new();

        let iter = self.db.prefix_iterator(prefix.as_bytes());
        for item in iter {
            let (_key, value) = item?;
            let bookmark: Bookmark = serde_json::from_slice(&value)?;

            if bookmark.timestamp >= start && bookmark.timestamp <= end {
                bookmarks.push(bookmark);
            }
        }

        Ok(bookmarks)
    }
}
```

---

## 🎥 COMPONENTE 2: MULTI-STREAMING (vms-ingest)

### Objetivo
Suportar múltiplos perfis de mídia (HD/SD/Mobile) com seleção automática baseada em banda e eventos.

### Arquitetura Proposta

```
vms-ingest/
├── src/
│   ├── profiles/
│   │   ├── mod.rs           # Gerenciamento de perfis
│   │   ├── profile.rs       # Definição de perfil
│   │   └── selector.rs      # Seleção automática
│   └── pipeline_multi.rs    # Pipeline com múltiplos streams
```

---

### 2.1 Definição de Perfis

```rust
// vms-ingest/src/profiles/profile.rs

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaProfile {
    pub id: String,
    pub name: String,
    pub purpose: ProfilePurpose,
    pub video_config: VideoConfig,
    pub audio_config: Option<AudioConfig>,
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ProfilePurpose {
    Recording,    // Gravação (alta qualidade)
    Viewing,      // Visualização desktop
    Mobile,       // Mobile (baixa banda)
    Analytics,    // Analytics (apenas I-frames)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoConfig {
    pub width: u32,
    pub height: u32,
    pub fps: u32,
    pub bitrate_kbps: u32,
    pub codec: VideoCodec,
    pub keyframe_interval: u32,
    pub quality: u8, // 1-100
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum VideoCodec {
    H264,
    H265,
    VP9,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioConfig {
    pub enabled: bool,
    pub codec: AudioCodec,
    pub sample_rate: u32,
    pub bitrate_kbps: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AudioCodec {
    Aac,
    Opus,
    G711,
}

// Perfis padrão
impl MediaProfile {
    pub fn high_quality() -> Self {
        Self {
            id: "high".to_string(),
            name: "High Quality".to_string(),
            purpose: ProfilePurpose::Recording,
            video_config: VideoConfig {
                width: 1920,
                height: 1080,
                fps: 30,
                bitrate_kbps: 4000,
                codec: VideoCodec::H264,
                keyframe_interval: 30,
                quality: 90,
            },
            audio_config: Some(AudioConfig {
                enabled: true,
                codec: AudioCodec::Aac,
                sample_rate: 48000,
                bitrate_kbps: 128,
            }),
            enabled: true,
        }
    }

    pub fn medium_quality() -> Self {
        Self {
            id: "medium".to_string(),
            name: "Medium Quality".to_string(),
            purpose: ProfilePurpose::Viewing,
            video_config: VideoConfig {
                width: 1280,
                height: 720,
                fps: 20,
                bitrate_kbps: 1500,
                codec: VideoCodec::H264,
                keyframe_interval: 20,
                quality: 75,
            },
            audio_config: Some(AudioConfig {
                enabled: true,
                codec: AudioCodec::Aac,
                sample_rate: 44100,
                bitrate_kbps: 96,
            }),
            enabled: true,
        }
    }

    pub fn mobile() -> Self {
        Self {
            id: "mobile".to_string(),
            name: "Mobile".to_string(),
            purpose: ProfilePurpose::Mobile,
            video_config: VideoConfig {
                width: 640,
                height: 360,
                fps: 15,
                bitrate_kbps: 500,
                codec: VideoCodec::H264,
                keyframe_interval: 15,
                quality: 60,
            },
            audio_config: Some(AudioConfig {
                enabled: true,
                codec: AudioCodec::Opus,
                sample_rate: 16000,
                bitrate_kbps: 32,
            }),
            enabled: true,
        }
    }
}
```

---

### 2.2 Seleção Automática de Perfil

```rust
// vms-ingest/src/profiles/selector.rs

pub struct ProfileSelector {
    profiles: Vec<MediaProfile>,
}

#[derive(Debug)]
pub struct SelectionCriteria {
    pub client_type: ClientType,
    pub bandwidth_kbps: Option<u32>,
    pub has_motion: bool,
    pub has_alarm: bool,
}

#[derive(Debug)]
pub enum ClientType {
    Desktop,
    Mobile,
    Analytics,
}

impl ProfileSelector {
    pub fn select_profile(&self, criteria: &SelectionCriteria) -> &MediaProfile {
        // 1. Filtrar por tipo de cliente
        let candidates: Vec<&MediaProfile> = match criteria.client_type {
            ClientType::Mobile => {
                self.profiles.iter()
                    .filter(|p| p.purpose == ProfilePurpose::Mobile)
                    .collect()
            }
            ClientType::Analytics => {
                self.profiles.iter()
                    .filter(|p| p.purpose == ProfilePurpose::Analytics)
                    .collect()
            }
            ClientType::Desktop => {
                // 2. Se há alarme, usar perfil de alta qualidade
                if criteria.has_alarm {
                    return self.profiles.iter()
                        .find(|p| p.purpose == ProfilePurpose::Recording)
                        .unwrap();
                }

                // 3. Se há movimento, usar perfil médio
                if criteria.has_motion {
                    return self.profiles.iter()
                        .find(|p| p.purpose == ProfilePurpose::Viewing)
                        .unwrap();
                }

                // 4. Considerar banda disponível
                if let Some(bandwidth) = criteria.bandwidth_kbps {
                    self.profiles.iter()
                        .filter(|p| p.enabled)
                        .filter(|p| p.video_config.bitrate_kbps <= bandwidth)
                        .max_by_key(|p| p.video_config.bitrate_kbps)
                        .unwrap()
                } else {
                    self.profiles.iter()
                        .find(|p| p.purpose == ProfilePurpose::Viewing)
                        .unwrap()
                }
            }
        };

        candidates.first().unwrap()
    }
}
```

---

### 2.3 Pipeline Multi-Stream

```rust
// vms-ingest/src/pipeline_multi.rs

use gstreamer as gst;
use std::collections::HashMap;

pub struct MultiStreamPipeline {
    source: gst::Element,
    tee: gst::Element,
    branches: HashMap<String, StreamBranch>,
}

struct StreamBranch {
    profile: MediaProfile,
    encoder: gst::Element,
    sink: gst::Element,
}

impl MultiStreamPipeline {
    pub fn new(rtsp_url: &str, profiles: Vec<MediaProfile>) -> anyhow::Result<Self> {
        gst::init()?;

        // Criar source RTSP
        let source = gst::ElementFactory::make("rtspsrc")
            .property("location", rtsp_url)
            .build()?;

        // Criar tee para dividir stream em múltiplos branches
        let tee = gst::ElementFactory::make("tee").build()?;

        let mut branches = HashMap::new();

        for profile in profiles {
            let branch = Self::create_branch(&profile)?;
            branches.insert(profile.id.clone(), branch);
        }

        Ok(Self {
            source,
            tee,
            branches,
        })
    }

    fn create_branch(profile: &MediaProfile) -> anyhow::Result<StreamBranch> {
        // Criar encoder com configuração do perfil
        let encoder = gst::ElementFactory::make("x264enc")
            .property("bitrate", profile.video_config.bitrate_kbps)
            .property("key-int-max", profile.video_config.keyframe_interval)
            .build()?;

        // Criar sink (NATS publisher)
        let sink = gst::ElementFactory::make("appsink")
            .property("emit-signals", true)
            .build()?;

        Ok(StreamBranch {
            profile: profile.clone(),
            encoder,
            sink,
        })
    }

    pub fn start(&mut self) -> anyhow::Result<()> {
        // TODO: Conectar todos os elementos e iniciar pipeline
        Ok(())
    }
}
```

---

## 🖥️ COMPONENTE 3: FRONTEND BÁSICO (clients/web)

### Objetivo
Interface web funcional com grid de câmeras, playback, alarmes e PTZ.

### Stack Tecnológico

- **Framework:** SolidJS (reatividade granular, bundle ~7KB)
- **Streaming:** WebRTC + HLS.js (fallback)
- **UI:** TailwindCSS + ShadCN-Solid
- **Build:** Vite
- **State:** Solid Store
- **Router:** @solidjs/router

---

### Estrutura de Projeto

```
clients/web/
├── src/
│   ├── main.tsx
│   ├── App.tsx
│   ├── components/
│   │   ├── Camera/
│   │   │   ├── CameraGrid.tsx
│   │   │   ├── CameraPlayer.tsx
│   │   │   └── CameraControls.tsx
│   │   ├── Playback/
│   │   │   ├── Timeline.tsx
│   │   │   ├── PlaybackPlayer.tsx
│   │   │   └── PlaybackControls.tsx
│   │   ├── Alarms/
│   │   │   ├── AlarmList.tsx
│   │   │   └── AlarmPopup.tsx
│   │   └── PTZ/
│   │       ├── PTZControl.tsx
│   │       └── PTZPresets.tsx
│   ├── stores/
│   │   ├── cameraStore.ts
│   │   ├── alarmStore.ts
│   │   └── authStore.ts
│   ├── services/
│   │   ├── api.ts
│   │   ├── webrtc.ts
│   │   └── websocket.ts
│   └── routes/
│       ├── Live.tsx
│       └── Playback.tsx
├── package.json
├── vite.config.ts
└── tsconfig.json
```

---

### 3.1 Setup Inicial

**package.json:**

```json
{
  "name": "vms-web-client",
  "version": "1.0.0",
  "type": "module",
  "scripts": {
    "dev": "vite",
    "build": "vite build",
    "preview": "vite preview"
  },
  "dependencies": {
    "solid-js": "^1.8.0",
    "@solidjs/router": "^0.10.0",
    "webrtc-adapter": "^8.2.3",
    "hls.js": "^1.4.14",
    "dayjs": "^1.11.10"
  },
  "devDependencies": {
    "vite": "^5.0.0",
    "vite-plugin-solid": "^2.8.0",
    "typescript": "^5.3.0",
    "tailwindcss": "^3.4.0",
    "autoprefixer": "^10.4.16",
    "postcss": "^8.4.32"
  }
}
```

---

### 3.2 Camera Grid (Live View)

```tsx
// src/components/Camera/CameraGrid.tsx

import { For, createSignal } from 'solid-js';
import CameraPlayer from './CameraPlayer';

interface Camera {
  id: string;
  name: string;
  stream_url: string;
}

export default function CameraGrid() {
  const [cameras, setCameras] = createSignal<Camera[]>([]);
  const [layout, setLayout] = createSignal<'1x1' | '2x2' | '3x3' | '4x4'>('2x2');

  // Fetch cameras from API
  async function loadCameras() {
    const response = await fetch('http://localhost:9095/api/v1/cameras');
    const data = await response.json();
    setCameras(data);
  }

  onMount(() => {
    loadCameras();
  });

  const gridClass = () => {
    switch (layout()) {
      case '1x1': return 'grid-cols-1 grid-rows-1';
      case '2x2': return 'grid-cols-2 grid-rows-2';
      case '3x3': return 'grid-cols-3 grid-rows-3';
      case '4x4': return 'grid-cols-4 grid-rows-4';
    }
  };

  return (
    <div class="flex flex-col h-full">
      {/* Layout selector */}
      <div class="p-2 bg-gray-800 flex gap-2">
        <button
          class="px-3 py-1 bg-blue-600 rounded"
          onClick={() => setLayout('2x2')}
        >
          2x2
        </button>
        <button
          class="px-3 py-1 bg-blue-600 rounded"
          onClick={() => setLayout('3x3')}
        >
          3x3
        </button>
        <button
          class="px-3 py-1 bg-blue-600 rounded"
          onClick={() => setLayout('4x4')}
        >
          4x4
        </button>
      </div>

      {/* Camera grid */}
      <div class={`grid ${gridClass()} gap-1 flex-1 bg-black p-1`}>
        <For each={cameras()}>
          {(camera) => (
            <CameraPlayer
              camera_id={camera.id}
              name={camera.name}
              stream_url={camera.stream_url}
            />
          )}
        </For>
      </div>
    </div>
  );
}
```

---

### 3.3 Camera Player (WebRTC)

```tsx
// src/components/Camera/CameraPlayer.tsx

import { onMount, createSignal, onCleanup } from 'solid-js';

interface Props {
  camera_id: string;
  name: string;
  stream_url: string;
}

export default function CameraPlayer(props: Props) {
  let videoRef: HTMLVideoElement;
  const [status, setStatus] = createSignal<'connecting' | 'connected' | 'error'>('connecting');
  let peerConnection: RTCPeerConnection;

  onMount(async () => {
    try {
      // Criar WebRTC connection
      peerConnection = new RTCPeerConnection({
        iceServers: [{ urls: 'stun:stun.l.google.com:19302' }]
      });

      // Adicionar track listener
      peerConnection.addEventListener('track', (event) => {
        videoRef.srcObject = event.streams[0];
        setStatus('connected');
      });

      // Criar offer
      const offer = await peerConnection.createOffer();
      await peerConnection.setLocalDescription(offer);

      // Enviar offer para servidor via API
      const response = await fetch(`http://localhost:9094/stream/${props.camera_id}/offer`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ sdp: offer.sdp })
      });

      const { answer } = await response.json();

      // Configurar answer
      await peerConnection.setRemoteDescription(
        new RTCSessionDescription({ type: 'answer', sdp: answer })
      );

    } catch (err) {
      console.error('WebRTC error:', err);
      setStatus('error');
    }
  });

  onCleanup(() => {
    peerConnection?.close();
  });

  return (
    <div class="relative bg-gray-900 flex items-center justify-center">
      {/* Video */}
      <video
        ref={videoRef!}
        autoplay
        muted
        class="w-full h-full object-contain"
      />

      {/* Overlay with camera name */}
      <div class="absolute top-0 left-0 right-0 bg-gradient-to-b from-black/70 to-transparent p-2">
        <div class="text-white text-sm font-medium">{props.name}</div>
        <div class="text-white/70 text-xs">
          {status() === 'connected' && '● Live'}
          {status() === 'connecting' && '○ Connecting...'}
          {status() === 'error' && '✕ Error'}
        </div>
      </div>
    </div>
  );
}
```

---

## 📊 CRONOGRAMA DETALHADO

### Semana 1: Playback System - Backend
- Dias 1-2: Timeline API + RocksDB schema
- Dias 3-4: Playback streaming
- Dia 5: Testes e ajustes

### Semana 2: Playback System - Export & Bookmarks
- Dias 1-2: Sistema de exportação com FFmpeg
- Dias 3-4: Bookmarks (CRUD completo)
- Dia 5: Integração e testes

### Semana 3: Multi-Streaming
- Dias 1-2: Definição de perfis + Configuração
- Dias 3-4: Pipeline multi-stream com GStreamer
- Dia 5: Seleção automática + testes

### Semana 4: Frontend - Setup & Live View
- Dias 1-2: Setup projeto + estrutura
- Dias 3-5: Camera Grid + WebRTC player

### Semana 5: Frontend - Playback & UI
- Dias 1-3: Playback player + Timeline
- Dias 4-5: Integração com backend

### Semana 6: Frontend - Alarmes & PTZ
- Dias 1-2: Lista de alarmes + WebSocket
- Dias 3-4: Controle PTZ
- Dia 5: Testes finais + ajustes

---

## ✅ CRITÉRIOS DE SUCESSO

Ao final da Fase 1, o sistema deve:

1. ✅ **Reproduzir gravações** com timeline visual
2. ✅ **Exportar vídeos** em MP4 com marca d'água
3. ✅ **Criar/listar bookmarks** para referência rápida
4. ✅ **Transmitir ao vivo** com múltiplos perfis (HD/SD/Mobile)
5. ✅ **Interface funcional** para operadores
6. ✅ **Controlar câmeras PTZ** via interface web
7. ✅ **Exibir alarmes** em tempo real

---

## 🔄 PRÓXIMOS PASSOS

Qual componente você quer que eu comece a implementar primeiro?

1. **Playback System** (vms-storage) - Mais crítico
2. **Multi-Streaming** (vms-ingest) - Otimização de banda
3. **Frontend** (clients/web) - Interface visual

Ou prefere que eu implemente os 3 em paralelo (vou criar PRs separados)?

//! Object tracking using ByteTrack algorithm

use crate::detector::{BoundingBox, Detection};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Track {
    pub id: u64,
    pub bbox: BoundingBox,
    pub class_id: usize,
    pub confidence: f32,
    pub age: u32,
    pub hits: u32,
}

pub struct Tracker {
    tracks: HashMap<u64, Track>,
    next_id: u64,
    max_age: u32,
    min_hits: u32,
    iou_threshold: f32,
}

impl Tracker {
    pub fn new(max_age: u32, min_hits: u32, iou_threshold: f32) -> Self {
        Self {
            tracks: HashMap::new(),
            next_id: 1,
            max_age,
            min_hits,
            iou_threshold,
        }
    }

    /// Atualiza tracks com novas detecções
    pub fn update(&mut self, detections: Vec<Detection>) -> Vec<Track> {
        // Associar detecções com tracks existentes usando IoU
        let mut matched_detections = Vec::new();
        let mut unmatched_detections = detections;

        for (_track_id, track) in self.tracks.iter_mut() {
            let mut best_match: Option<(usize, f32)> = None;

            for (i, det) in unmatched_detections.iter().enumerate() {
                let iou = Self::calculate_iou_static(&track.bbox, &det.bbox);

                if iou > self.iou_threshold {
                    if let Some((_, best_iou)) = best_match {
                        if iou > best_iou {
                            best_match = Some((i, iou));
                        }
                    } else {
                        best_match = Some((i, iou));
                    }
                }
            }

            if let Some((idx, _)) = best_match {
                let det = unmatched_detections.remove(idx);
                track.bbox = det.bbox;
                track.confidence = det.confidence;
                track.hits += 1;
                track.age = 0;
                matched_detections.push(track.clone());
            } else {
                track.age += 1;
            }
        }

        // Criar novos tracks para detecções não associadas
        for det in unmatched_detections {
            let track = Track {
                id: self.next_id,
                bbox: det.bbox,
                class_id: det.class_id,
                confidence: det.confidence,
                age: 0,
                hits: 1,
            };
            self.tracks.insert(self.next_id, track);
            self.next_id += 1;
        }

        // Remover tracks antigos
        self.tracks.retain(|_, track| track.age < self.max_age);

        // Retornar apenas tracks confirmados
        self.tracks
            .values()
            .filter(|t| t.hits >= self.min_hits)
            .cloned()
            .collect()
    }

    fn calculate_iou_static(bbox1: &BoundingBox, bbox2: &BoundingBox) -> f32 {
        let x1 = bbox1.x.max(bbox2.x);
        let y1 = bbox1.y.max(bbox2.y);
        let x2 = (bbox1.x + bbox1.width).min(bbox2.x + bbox2.width);
        let y2 = (bbox1.y + bbox1.height).min(bbox2.y + bbox2.height);

        if x2 < x1 || y2 < y1 {
            return 0.0;
        }

        let intersection = (x2 - x1) * (y2 - y1);
        let area1 = bbox1.width * bbox1.height;
        let area2 = bbox2.width * bbox2.height;
        let union = area1 + area2 - intersection;

        if union > 0.0 {
            intersection / union
        } else {
            0.0
        }
    }

    pub fn get_active_tracks(&self) -> Vec<Track> {
        self.tracks
            .values()
            .filter(|t| t.hits >= self.min_hits)
            .cloned()
            .collect()
    }
}

//! Playback Module
//! Módulo responsável por reprodução de gravações

pub mod timeline;
pub mod streamer;
pub mod bookmark;

pub use timeline::{Timeline, TimelineBuilder, TimelineResolution, RecordingSegment, MotionZone, TimelineEvent};
pub use streamer::PlaybackStreamer;
pub use bookmark::{Bookmark, BookmarkManager, CreateBookmarkRequest, UpdateBookmarkRequest};

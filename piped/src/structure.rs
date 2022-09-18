use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Channel {
    pub id: String,
    pub name: String,
    pub avatar_url: String,
    pub banner_url: Option<String>,
    pub description: String,
    pub nextpage: Option<String>,
    pub nextbody: Option<String>,
    pub related_streams: Vec<RelatedStream>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Playlist {
    pub name: String,
    pub thumbnail_url: String,
    pub banner_url: Option<String>,
    pub uploader: String,
    pub uploader_url: String,
    pub uploader_avatar: Option<String>,
    pub videos: i32,
    pub nextpage: Option<String>,
    pub nextbody: Option<String>,
    pub related_streams: Vec<RelatedStream>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StreamsPage {
    pub nextpage: Option<String>,
    pub nextbody: Option<String>,
    pub related_streams: Vec<RelatedStream>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RelatedStream {
    pub url: String,
    pub title: String,
    pub thumbnail: String,
    pub uploader_avatar: Option<String>,
    pub uploader_name: String,
    pub uploader_url: String,
    pub uploaded_date: Option<String>,
    pub uploader_verified: bool,
    pub duration: i32,
    pub views: i64,
    pub uploaded: i64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoInfo {
    pub title: String,
    pub description: String,
    pub dash: Option<String>,
    pub upload_date: String,
    pub uploader: String,
    pub uploader_url: String,
    pub uploader_avatar: String,
    pub thumbnail_url: String,
    pub hls: String,
    pub duration: i32,
    pub views: i64,
    pub likes: i64,
    pub lbry_id: Option<String>,
    pub dislikes: i64,
    pub audio_streams: Vec<Stream>,
    pub video_streams: Vec<Stream>,
    pub related_streams: Vec<RelatedStream>,
    pub subtitles: Vec<Subtitle>,
    pub livestream: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Stream {
    pub url: String,
    pub format: String,
    pub quality: String,
    pub mime_type: String,
    pub codec: Option<String>,
    pub video_only: bool,
    pub bitrate: i32,
    pub init_start: i32,
    pub init_end: i32,
    pub index_start: i32,
    pub index_end: i32,
    pub width: i32,
    pub height: i32,
    pub fps: i32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Subtitle {
    pub url: String,
    pub mime_type: String,
    pub name: String,
    pub code: String,
    pub auto_generated: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommentsInfo {
    pub comments: Vec<Comment>,
    pub nextpage: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Comment {
    pub author: String,
    pub thumbnail: String,
    pub comment_id: String,
    pub comment_text: String,
    pub commented_time: String,
    pub commentor_url: String,
    pub like_count: i64,
    pub hearted: bool,
    pub pinned: bool,
    pub verified: bool,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ChannelSearch {
    pub items: Vec<ChannelSearchItem>,
    pub nextpage: Option<String>,
    pub suggestion: Option<String>,
    pub corrected: bool,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ChannelSearchItem {
    pub name: String,
    pub thumbnail: String,
    pub url: String,
    pub description: Option<String>,
    pub subscribers: i32,
    pub videos: i32,
    pub verified: bool,
}

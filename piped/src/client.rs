use reqwest::{Client, Url};

use crate::{
    Channel, ChannelSearch, CommentsInfo, Playlist, RelatedStream, Result, StreamsPage, VideoInfo,
};

pub struct PipedClient {
    pub httpclient: Client,
    pub instance: String,
}

const USER_AGENT: &str = concat!("piped-rust-sdk/{}", env!("CARGO_PKG_VERSION"));

impl PipedClient {
    pub fn new<S: AsRef<str>>(httpclient: &Client, instance: S) -> PipedClient {
        // Format url to always have http(s) in the beginning and no ending /
        let mut url = instance.as_ref().to_owned();
        if !url.starts_with("http") {
            url = format!("https://{}", url);
        }
        if url.ends_with('/') {
            url.pop();
        }
        PipedClient {
            httpclient: httpclient.clone(),
            instance: url,
        }
    }

    pub async fn trending<S: AsRef<str>>(&self, region: S) -> Result<Vec<RelatedStream>> {
        let mut url = Url::parse(format!("{}/trending", &self.instance).as_str())?;
        url.query_pairs_mut().append_pair("region", region.as_ref());

        let resp = &self
            .httpclient
            .get(url)
            .header("User-Agent", USER_AGENT)
            .send()
            .await?
            .text()
            .await?;

        let streams: Vec<RelatedStream> = serde_json::from_str(resp.as_ref())?;

        Ok(streams)
    }

    pub async fn channel_from_id<S: AsRef<str>>(&self, id: S) -> Result<Channel> {
        let resp = &self
            .httpclient
            .get(format!("{}/channel/{}", &self.instance, id.as_ref()))
            .header("User-Agent", USER_AGENT)
            .send()
            .await?
            .text()
            .await?;

        let channel: Channel = serde_json::from_str(resp.as_str())?;

        Ok(channel)
    }

    pub async fn bulk_feed<S: AsRef<str>, I: IntoIterator<Item = S>>(
        &self,
        ids: I,
    ) -> Result<Vec<RelatedStream>> {
        let resp = &self
            .httpclient
            .get(format!(
                "{}/feed/unauthenticated?channels={}",
                &self.instance,
                ids.into_iter()
                    .map(|s| s.as_ref().to_owned())
                    .collect::<Vec<_>>()
                    .join(",")
            ))
            .header("User-Agent", USER_AGENT)
            .send()
            .await?
            .text()
            .await?;

        let videos: Vec<RelatedStream> = serde_json::from_str(resp.as_str())?;

        Ok(videos)
    }

    pub async fn channel_continuation<S: AsRef<str>>(
        &self,
        id: S,
        nexturl: S,
        nextbody: S,
    ) -> Result<StreamsPage> {
        let mut url =
            Url::parse(format!("{}/nextpage/channels/{}", &self.instance, id.as_ref()).as_str())?;
        url.query_pairs_mut()
            .append_pair("url", nexturl.as_ref())
            .append_pair("id", nextbody.as_ref());

        let resp = &self
            .httpclient
            .get(url)
            .header("User-Agent", USER_AGENT)
            .send()
            .await?
            .text()
            .await?;

        let streams: StreamsPage = serde_json::from_str(resp.as_ref())?;

        Ok(streams)
    }

    pub async fn playlist_from_id<S: AsRef<str>>(&self, id: S) -> Result<Playlist> {
        let resp = &self
            .httpclient
            .get(format!("{}/playlists/{}", &self.instance, id.as_ref()))
            .header("User-Agent", USER_AGENT)
            .send()
            .await?
            .text()
            .await?;

        let playlist: Playlist = serde_json::from_str(resp.as_ref())?;

        Ok(playlist)
    }

    pub async fn playlist_continuation<S: AsRef<str>>(
        &self,
        id: S,
        nexturl: S,
        nextbody: S,
    ) -> Result<StreamsPage> {
        let mut url =
            Url::parse(format!("{}/nextpage/playlists/{}", &self.instance, id.as_ref()).as_str())?;
        url.query_pairs_mut()
            .append_pair("url", nexturl.as_ref())
            .append_pair("id", nextbody.as_ref());

        let resp = &self
            .httpclient
            .get(url)
            .header("User-Agent", USER_AGENT)
            .send()
            .await?
            .text()
            .await?;

        let streams: StreamsPage = serde_json::from_str(resp.as_str())?;

        Ok(streams)
    }

    pub async fn video_from_id<S: AsRef<str>>(&self, id: S) -> Result<VideoInfo> {
        let resp = &self
            .httpclient
            .get(format!("{}/streams/{}", &self.instance, id.as_ref()))
            .header("User-Agent", USER_AGENT)
            .send()
            .await?
            .text()
            .await?;

        let video: VideoInfo = serde_json::from_str(resp.as_str())?;

        Ok(video)
    }

    pub async fn search_suggestions<S: AsRef<str>>(&self, q: S) -> Result<Vec<String>> {
        let mut url = Url::parse(format!("{}/suggestions", &self.instance).as_str())?;
        url.query_pairs_mut().append_pair("query", q.as_ref());

        let resp = &self
            .httpclient
            .get(url)
            .header("User-Agent", USER_AGENT)
            .send()
            .await?
            .text()
            .await?;

        let suggestions: Vec<String> = serde_json::from_str(resp.as_ref())?;

        Ok(suggestions)
    }

    pub async fn comments_from_id<S: AsRef<str>>(&self, id: S) -> Result<CommentsInfo> {
        let resp = &self
            .httpclient
            .get(format!("{}/comments/{}", &self.instance, id.as_ref()))
            .header("User-Agent", USER_AGENT)
            .send()
            .await?
            .text()
            .await?;

        let comments: CommentsInfo = serde_json::from_str(resp.as_ref())?;

        Ok(comments)
    }

    pub async fn comments_continuation<S: AsRef<str>>(
        &self,
        id: S,
        nexturl: S,
    ) -> Result<CommentsInfo> {
        let mut url =
            Url::parse(format!("{}/nextpage/comments/{}", &self.instance, id.as_ref()).as_str())?;
        url.query_pairs_mut().append_pair("url", nexturl.as_ref());

        let resp = &self
            .httpclient
            .get(url)
            .header("User-Agent", USER_AGENT)
            .send()
            .await?
            .text()
            .await?;

        let comments: CommentsInfo = serde_json::from_str(resp.as_ref())?;

        Ok(comments)
    }

    pub async fn search_channel<S: AsRef<str>>(&self, name: S) -> Result<ChannelSearch> {
        let mut url = Url::parse(format!("{}/search", &self.instance).as_str())?;
        url.query_pairs_mut().append_pair("q", name.as_ref());
        url.query_pairs_mut().append_pair("filter", "channels");

        let resp = &self
            .httpclient
            .get(url)
            .header("User-Agent", USER_AGENT)
            .send()
            .await?
            .text()
            .await?;

        let suggestions: ChannelSearch = serde_json::from_str(resp.as_ref())?;

        Ok(suggestions)
    }
}

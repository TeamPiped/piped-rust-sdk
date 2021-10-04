use reqwest::{Client, Url};

use crate::{Channel, CommentsInfo, Playlist, RelatedStream, StreamsPage, VideoInfo};

pub struct PipedClient {
    pub httpclient: Client,
    pub instance: String,
}

const USER_AGENT: &'static str = "Mozilla/5.0 (Windows NT 10.0; rv:78.0) Gecko/20100101 Firefox/78.0";

impl PipedClient {
    pub fn new<S: AsRef<str>>(httpclient: &Client, instance: S) -> PipedClient {
        PipedClient {
            httpclient: httpclient.clone(),
            instance: instance.as_ref().to_string(),
        }
    }

    pub async fn trending<S: AsRef<str>>(
        &self,
        region: S,
    ) -> Result<Vec<RelatedStream>, Box<dyn std::error::Error>> {
        let mut url = Url::parse(format!("{}/trending", &self.instance).as_str())?;
        url.query_pairs_mut().append_pair("region", region.as_ref());

        let resp = &self.httpclient.get(url)
            .header("User-Agent", USER_AGENT)
            .send().await?.text().await?;

        let streams: Vec<RelatedStream> = serde_json::from_str(resp.as_ref())?;

        Ok(streams)
    }

    pub async fn channel_from_id<S: AsRef<str>>(&self, id: S) -> Result<Channel, Box<dyn std::error::Error>> {
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

    pub async fn channel_continuation<S: AsRef<str>>(
        &self,
        id: S,
        nexturl: S,
        nextbody: S,
    ) -> Result<StreamsPage, Box<dyn std::error::Error>> {
        let mut url = Url::parse(format!("{}/nextpage/channels/{}", &self.instance, id.as_ref()).as_str())?;
        url.query_pairs_mut()
            .append_pair("url", nexturl.as_ref())
            .append_pair("id", nextbody.as_ref());

        let resp = &self.httpclient.get(url)
            .header("User-Agent", USER_AGENT)
            .send().await?.text().await?;

        let streams: StreamsPage = serde_json::from_str(resp.as_ref())?;

        Ok(streams)
    }

    pub async fn playlist_from_id<S: AsRef<str>>(
        &self,
        id: S,
    ) -> Result<Playlist, Box<dyn std::error::Error>> {
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
    ) -> Result<StreamsPage, Box<dyn std::error::Error>> {
        let mut url = Url::parse(format!("{}/nextpage/playlists/{}", &self.instance, id.as_ref()).as_str())?;
        url.query_pairs_mut()
            .append_pair("url", nexturl.as_ref())
            .append_pair("id", nextbody.as_ref());

        let resp = &self.httpclient.get(url)
            .header("User-Agent", USER_AGENT)
            .send().await?.text().await?;

        let streams: StreamsPage = serde_json::from_str(resp.as_str())?;

        Ok(streams)
    }

    pub async fn video_from_id<S: AsRef<str>>(&self, id: S) -> Result<VideoInfo, Box<dyn std::error::Error>> {
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

    pub async fn search_suggestions<S: AsRef<str>>(
        &self,
        q: S,
    ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let mut url = Url::parse(format!("{}/suggestions", &self.instance).as_str())?;
        url.query_pairs_mut().append_pair("query", q.as_ref());

        let resp = &self.httpclient.get(url)
            .header("User-Agent", USER_AGENT)
            .send().await?.text().await?;

        let suggestions: Vec<String> = serde_json::from_str(resp.as_ref())?;

        Ok(suggestions)
    }

    pub async fn comments_from_id<S: AsRef<str>>(
        &self,
        id: S,
    ) -> Result<CommentsInfo, Box<dyn std::error::Error>> {
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
    ) -> Result<CommentsInfo, Box<dyn std::error::Error>> {
        let mut url = Url::parse(format!("{}/nextpage/comments/{}", &self.instance, id.as_ref()).as_str())?;
        url.query_pairs_mut().append_pair("url", nexturl.as_ref());

        let resp = &self.httpclient.get(url)
            .header("User-Agent", USER_AGENT)
            .send().await?.text().await?;

        let comments: CommentsInfo = serde_json::from_str(resp.as_ref())?;

        Ok(comments)
    }
}

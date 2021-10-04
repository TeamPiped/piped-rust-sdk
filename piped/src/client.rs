use reqwest::{Client, Url};

use crate::{Channel, CommentsInfo, Playlist, RelatedStream, StreamsPage, VideoInfo};

pub struct PipedClient {
    pub httpclient: Client,
    pub instance: String,
}

impl PipedClient {
    pub fn new(httpclient: Client, instance: String) -> PipedClient {
        PipedClient {
            httpclient,
            instance,
        }
    }

    pub async fn trending(
        &self,
        region: String,
    ) -> Result<Vec<RelatedStream>, Box<dyn std::error::Error>> {
        let mut url = Url::parse(format!("{}/trending", &self.instance).as_str())?;
        url.query_pairs_mut().append_pair("region", region.as_str());

        let resp = &self.httpclient.get(url).send().await?.text().await?;

        let streams: Vec<RelatedStream> = serde_json::from_str(resp.as_str())?;

        Ok(streams)
    }

    pub async fn channel_from_id(&self, id: String) -> Result<Channel, Box<dyn std::error::Error>> {
        let resp = &self
            .httpclient
            .get(format!("{}/channel/{}", &self.instance, id))
            .send()
            .await?
            .text()
            .await?;

        let channel: Channel = serde_json::from_str(resp.as_str())?;

        Ok(channel)
    }

    pub async fn channel_continuation(
        &self,
        id: String,
        nexturl: String,
        nextbody: String,
    ) -> Result<StreamsPage, Box<dyn std::error::Error>> {
        let mut url = Url::parse(format!("{}/nextpage/channels/{}", &self.instance, id).as_str())?;
        url.query_pairs_mut()
            .append_pair("url", nexturl.as_str())
            .append_pair("id", nextbody.as_str());

        let resp = &self.httpclient.get(url).send().await?.text().await?;

        let streams: StreamsPage = serde_json::from_str(resp.as_str())?;

        Ok(streams)
    }

    pub async fn playlist_from_id(
        &self,
        id: String,
    ) -> Result<Playlist, Box<dyn std::error::Error>> {
        let resp = &self
            .httpclient
            .get(format!("{}/playlists/{}", &self.instance, id))
            .send()
            .await?
            .text()
            .await?;

        let playlist: Playlist = serde_json::from_str(resp.as_str())?;

        Ok(playlist)
    }

    pub async fn playlist_continuation(
        &self,
        id: String,
        nexturl: String,
        nextbody: String,
    ) -> Result<StreamsPage, Box<dyn std::error::Error>> {
        let mut url = Url::parse(format!("{}/nextpage/playlists/{}", &self.instance, id).as_str())?;
        url.query_pairs_mut()
            .append_pair("url", nexturl.as_str())
            .append_pair("id", nextbody.as_str());

        let resp = &self.httpclient.get(url).send().await?.text().await?;

        let streams: StreamsPage = serde_json::from_str(resp.as_str())?;

        Ok(streams)
    }

    pub async fn video_from_id(&self, id: String) -> Result<VideoInfo, Box<dyn std::error::Error>> {
        let resp = &self
            .httpclient
            .get(format!("{}/streams/{}", &self.instance, id))
            .send()
            .await?
            .text()
            .await?;

        let video: VideoInfo = serde_json::from_str(resp.as_str())?;

        Ok(video)
    }

    pub async fn search_suggestions(
        &self,
        q: String,
    ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let mut url = Url::parse(format!("{}/suggestions", &self.instance).as_str())?;
        url.query_pairs_mut().append_pair("query", q.as_str());

        let resp = &self.httpclient.get(url).send().await?.text().await?;

        let suggestions: Vec<String> = serde_json::from_str(resp.as_str())?;

        Ok(suggestions)
    }

    pub async fn comments_from_id(
        &self,
        id: String,
    ) -> Result<CommentsInfo, Box<dyn std::error::Error>> {
        let resp = &self
            .httpclient
            .get(format!("{}/comments/{}", &self.instance, id))
            .send()
            .await?
            .text()
            .await?;

        let comments: CommentsInfo = serde_json::from_str(resp.as_str())?;

        Ok(comments)
    }

    pub async fn comments_continuation(
        &self,
        id: String,
        nexturl: String,
    ) -> Result<CommentsInfo, Box<dyn std::error::Error>> {
        let mut url = Url::parse(format!("{}/nextpage/comments/{}", &self.instance, id).as_str())?;
        url.query_pairs_mut().append_pair("url", nexturl.as_str());

        let resp = &self.httpclient.get(url).send().await?.text().await?;

        let comments: CommentsInfo = serde_json::from_str(resp.as_str())?;

        Ok(comments)
    }
}

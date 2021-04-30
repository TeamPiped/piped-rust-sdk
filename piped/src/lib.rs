pub mod piped {
    use reqwest::{Client, Url};
    use serde::Deserialize;

    pub struct PipedClient {
        pub httpclient: Client,
        pub instance: String,
    }

    impl PipedClient {
        pub fn new(httpclient: Client, instance: String) -> PipedClient {
            PipedClient {
                httpclient: httpclient,
                instance: instance,
            }
        }

        pub async fn get_trending(&self) -> Result<Vec<RelatedStream>, Box<dyn std::error::Error>> {
            let resp = &self
                .httpclient
                .get(format!("{}/trending", &self.instance))
                .send()
                .await?
                .text()
                .await?;

            let streams: Vec<RelatedStream> = serde_json::from_str(resp.as_str())?;

            Ok(streams)
        }

        pub async fn get_channel_from_id(
            &self,
            id: String,
        ) -> Result<Channel, Box<dyn std::error::Error>> {
            let resp = &self
                .httpclient
                .get(format!("{}/channels/{}", &self.instance, id))
                .send()
                .await?
                .text()
                .await?;

            let channel: Channel = serde_json::from_str(resp.as_str())?;

            Ok(channel)
        }

        pub async fn get_channel_continuation(
            &self,
            id: String,
            nexturl: String,
            nextbody: String,
        ) -> Result<StreamsPage, Box<dyn std::error::Error>> {
            let mut url =
                Url::parse(format!("{}/nextpage/channels/{}", &self.instance, id).as_str())?;
            url.query_pairs_mut()
                .append_pair("url", nexturl.as_str())
                .append_pair("id", nextbody.as_str());

            let resp = &self
                .httpclient
                .get(url.as_str())
                .send()
                .await?
                .text()
                .await?;

            let streams: StreamsPage = serde_json::from_str(resp.as_str())?;

            Ok(streams)
        }

        pub async fn get_playlist_from_id(
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

        pub async fn get_playlist_continuation(
            &self,
            id: String,
            nexturl: String,
            nextbody: String,
        ) -> Result<StreamsPage, Box<dyn std::error::Error>> {
            let mut url =
                Url::parse(format!("{}/nextpage/playlists/{}", &self.instance, id).as_str())?;
            url.query_pairs_mut()
                .append_pair("url", nexturl.as_str())
                .append_pair("id", nextbody.as_str());

            let resp = &self
                .httpclient
                .get(url.as_str())
                .send()
                .await?
                .text()
                .await?;

            let streams: StreamsPage = serde_json::from_str(resp.as_str())?;

            Ok(streams)
        }
    }

    #[derive(Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct Channel {
        pub id: String,
        pub name: String,
        pub avatar_url: String,
        pub banner_url: String,
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
        pub uploader_avatar: String,
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
        pub uploader_name: String,
        pub uploader_url: String,
        pub uploaded_date: Option<String>,
        pub duration: i32,
        pub views: i64,
    }
}

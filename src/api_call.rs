use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Post {
    id: i32,
    user_id: i32,
    title: String,
    body: String,
}

impl Post {
    pub async fn fetch(id: i32, client: Client) -> Result<Option<Self>, reqwest::Error> {
        let url = format!("https://dummyjson.com/posts/{}", id);
        let resp = client.get(&url).send().await?;
        if resp.status() == StatusCode::NOT_FOUND {
            return Ok(None);
        }
        Ok(Some(resp.json().await?))
    }
}

#[cfg(test)]
mod tests {
    use reqwest::Client;

    use crate::api_call::Post;

    #[tokio::test]
    async fn test_fetches_post() {
        let client = Client::new();
        let post = Post::fetch(10, client).await.unwrap();
        println!("Post {:?}", post);

        assert!(!post.is_none());
        assert_eq!(post.map(|p| p.id), Some(10));
    }
}

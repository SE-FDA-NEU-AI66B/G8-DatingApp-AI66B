use crate::share::register::MessageBody;
use gloo_net::http::{Request, Response};
use serde::{de::DeserializeOwned, Serialize};

const NETWORK_ERROR: &str = "Network error, please try again";

async fn parse<T: DeserializeOwned>(resp: Response) -> Result<T, String> {
    if resp.ok() {
        resp.json::<T>()
            .await
            .map_err(|_| "Unexpected server response".to_string())
    } else {
        // Cả MessageBody lẫn AuthError đều có dạng {"message": ...}
        Err(resp
            .json::<MessageBody>()
            .await
            .map(|m| m.message)
            .unwrap_or_else(|_| format!("Request failed ({})", resp.status())))
    }
}

pub async fn get_json<T: DeserializeOwned>(url: &str) -> Result<T, String> {
    let resp = Request::get(url).send().await.map_err(|_| NETWORK_ERROR.to_string())?;
    parse(resp).await
}

pub async fn post_json<B: Serialize, T: DeserializeOwned>(url: &str, body: &B) -> Result<T, String> {
    let resp = Request::post(url)
        .json(body)
        .map_err(|e| e.to_string())?
        .send()
        .await
        .map_err(|_| NETWORK_ERROR.to_string())?;
    parse(resp).await
}

pub async fn post_empty<T: DeserializeOwned>(url: &str) -> Result<T, String> {
    let resp = Request::post(url).send().await.map_err(|_| NETWORK_ERROR.to_string())?;
    parse(resp).await
}

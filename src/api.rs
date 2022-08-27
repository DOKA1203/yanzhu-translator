use std::time::Duration;

use reqwest::{multipart, Body};
use serde_derive::Deserialize;
use tokio::fs::File;
use tokio_util::codec::{BytesCodec, FramedRead};
use uuid::Uuid;

use crate::signature;

static VERSION: &str = "1.9.9";
lazy_static! {
    static ref SID: String = format!("{}_{}", VERSION, Uuid::new_v4());
}

#[derive(Deserialize, Debug)]
pub struct TranslateResult {
    #[serde(rename = "renderedImage")]
    pub rendered_image: Option<String>,
}

pub async fn image_translate(
    src_lang: &str,
    dst_lang: &str,
    lang_detect: bool,
    file: File,
) -> Result<TranslateResult, Box<dyn std::error::Error>> {
    let url = "https://apis.naver.com/papago/papago_app/ocr/detect";
    let sig = signature::sign_url(&url);

    let client = reqwest::Client::new();
    let reader = Body::wrap_stream(FramedRead::new(file, BytesCodec::new()));
    let form = multipart::Form::new()
        .text("lang", "ko")
        .text("upload", "true")
        .text("sid", SID.to_owned())
        .part("image", multipart::Part::stream(reader).file_name("image"))
        .text("source", src_lang.to_owned())
        .text("target", dst_lang.to_owned())
        .text("langDetect", if lang_detect { "true" } else { "False" })
        .text("imageId", "")
        .text("reqType", "");

    let res = client
        .post(url)
        .query(&[("msgpad", sig.ts.to_string()), ("md", sig.msg)])
        .multipart(form)
        .timeout(Duration::from_secs(10))
        .send()
        .await?;
    Ok(res.json::<TranslateResult>().await?)
}

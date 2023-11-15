#[macro_use]
extern crate lazy_static;

use std::error;

use base64::{engine::general_purpose, Engine as _};
use tokio::{fs::File, io::AsyncWriteExt};

mod api;
mod signature;
mod utils;

async fn translate_and_save(
    src_lang: &str,
    dst_lang: &str,
    input_file: &str,
    output_file: &str,
) -> Result<(), Box<dyn error::Error>> {
    let file = File::open(&input_file).await?;

    let res = api::image_translate(&src_lang, &dst_lang, false, file).await?;

    match res.rendered_image {
        Some(data) => {
            let data = general_purpose::STANDARD.decode(data).unwrap();
            let mut output_file = File::create(output_file).await?;
            output_file.write_all(&data).await?;
            Ok(())
        }
        None => Err("no data".into()),
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    if let Err(err) = translate_and_save("en", "ko", "Example.jpg", "out.jpg").await {
        println!("error {:?}", err);
    } else {
        println!("success");
    }
    Ok(())
}

#[macro_use]
extern crate lazy_static;

use std::error;
use base64::{engine::general_purpose, Engine as _};
use tokio::{fs, fs::File, io, io::AsyncWriteExt};
use tokio::io::{AsyncBufReadExt};
use crate::utils::{find_jpeg_files, resize_image};

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
    let stdin = io::stdin();
    let mut handle = io::BufReader::new(stdin);

    println!("Yanzhu-image-translator");
    println!("lang: [ko,en,ja,zh-CN]");
    println!();
    println!("Source language: ");
    let mut src_lang = String::new();
    handle.read_line(&mut src_lang).await.expect("입력을 읽을 수 없습니다.");
    let src_lang: String = src_lang.trim().parse().unwrap();

    println!("Destination language: ");
    let mut dst_lang = String::new();
    handle.read_line(&mut dst_lang).await.expect("입력을 읽을 수 없습니다.");
    let dst_lang: String = dst_lang.trim().parse().unwrap();
    println!("Input folder: ");
    let mut input_folder = String::new();
    handle.read_line(&mut input_folder).await.expect("입력을 읽을 수 없습니다.");
    input_folder = input_folder.trim().parse().unwrap();

    let output_folder = format!("{}\\{}", input_folder, "translated");
    fs::create_dir_all(output_folder.clone()).await?;
    let mut cnt = 1;
    let files = find_jpeg_files(input_folder.trim()).unwrap();
    let len = files.len();
    for i in files {
        let file_name = i.rsplitn(2, '\\').next().unwrap_or("");
        resize_image(i.as_str(), format!("{output_folder}\\temp.jpg").as_str(), 1950).expect("WTF");
        if let Err(err) = translate_and_save(src_lang.as_str(), dst_lang.as_str(), format!("{output_folder}\\temp.jpg").as_str(), format!("{output_folder}\\{file_name}").as_str()).await {
            println!("error {:?}, {}", err, format!("{}\\{}", output_folder, file_name).as_str());
        } else {
            println!("success, {} ({}/{})", file_name, cnt, len);
            cnt+=1
        }
    }
    fs::remove_file(format!("{output_folder}\\temp.jpg")).await?;
    Ok(())
}

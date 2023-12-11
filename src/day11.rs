use axum::{extract::Multipart, http::StatusCode};
use std::io::Cursor;
use bytes::Bytes;
use image::{io::Reader as ImageReader, ImageFormat, Rgb};
use eyre::eyre;

fn magical_red(&Rgb([r, g, b]): &Rgb<u8>) -> bool {
    r > b.saturating_add(g)
}

fn compute_red_pixels(img: &Bytes) -> eyre::Result<u64> {
    let png = ImageReader::with_format(
        Cursor::new(img),
        ImageFormat::Png)
        .decode()?;
    let png = png.as_rgb8()
        .ok_or(eyre!("Cannot view as RGB8"))?;
    let mut reds = 0;
    for p in png.pixels() {
        if magical_red(p) {
            reds += 1;
        }
    }
    Ok(reds)
}

pub async fn red_pixels(mut multipart: Multipart) -> Result<String, StatusCode>
{
    while let Some(field) = multipart.next_field().await
        .or(Err(StatusCode::NOT_FOUND))?
    {
        let name = field.name().ok_or(StatusCode::NOT_FOUND)?;
        if name == "image" {
            let img = field.bytes().await.or(Err(StatusCode::NOT_FOUND))?;
            let n = compute_red_pixels(&img)
                .or(Err(StatusCode::NOT_FOUND))?;
            return Ok(format!("{n}"))
        }
    }
    Err(StatusCode::NOT_FOUND)
}

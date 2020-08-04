mod to_mono;
mod to_u32;
mod to_u8;

use image::DynamicImage;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct ReadInformationSource {
    img_path: String,
    prefix: String,
    height: usize,
    x_list: Vec<usize>,
    labels: Vec<String>,
}

pub struct ReadInformation {
    img: DynamicImage,
    rests: Vec<Rect>,
    labels: Vec<String>,
}

impl From<ReadInformationSource> for ReadInformation {
    fn from(src: ReadInformationSource) -> Self {
        let ReadInformationSource {
            img_path,
            height,
            x_list,
            labels,
            ..
        } = src;

        let mut x = 0;

        Self {
            img: image::open(img_path).unwrap(),
            rests: x_list
                .iter()
                .map(|w| {
                    let re = Rect {
                        x,
                        y: 0,
                        width: *w,
                        height,
                    };
                    x += w + 1;
                    re
                })
                .collect::<Vec<_>>(),
            labels,
        }
    }
}

#[derive(Debug)]
pub struct Rect {
    x: usize,
    y: usize,
    width: usize,
    height: usize,
}

fn main() {
    let args = std::env::args().into_iter().collect::<Vec<_>>();
    let file = args.get(1).expect("need file path");
    let mode = match args.get(2) {
        None => "mono".to_string(),
        Some(n) => n.to_string(),
    };
    let json = std::fs::read(file).unwrap();
    let src: ReadInformationSource = serde_json::from_slice(&json).unwrap();
    let prefix = src.prefix.clone();

    let t = if mode == "mono" {
        to_mono::to_vec(src.into()).into_iter().fold(
            "".to_string(),
            |a, (label, width, height, data)| {
                a + &format!(
                    "    pub static ref {}_{}: (usize, usize, Vec<u8>) = ({}, {}, vec!{:?});\n",
                    prefix.to_uppercase(),
                    label.to_uppercase(),
                    width,
                    height,
                    data
                )
            },
        )
    } else if mode == "8" {
        to_u8::to_vec(src.into()).into_iter().fold(
            "".to_string(),
            |a, (label, width, height, data)| {
                a + &format!(
                    "    pub static ref {}_{}: (usize, usize, Vec<u32>) = ({}, {}, vec!{:?});\n",
                    prefix.to_uppercase(),
                    label.to_uppercase(),
                    width,
                    height,
                    data
                )
            },
        )
    } else {
        to_u32::to_vec(src.into()).into_iter().fold(
            "".to_string(),
            |a, (label, width, height, data)| {
                a + &format!(
                    "    pub static ref {}_{}: (usize, usize, Vec<u32>) = ({}, {}, vec!{:?});\n",
                    prefix.to_uppercase(),
                    label.to_uppercase(),
                    width,
                    height,
                    data
                )
            },
        )
    };

    println!("{}", format!("lazy_static! {{\n{}}}\n", t));
}

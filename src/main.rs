mod to_mono;
mod to_u32;
mod to_u8;

use image::DynamicImage;
use image::GenericImageView;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct ReadInformationSource {
    img_path: String,
    prefix: String,
    height: usize,
    array_length: usize,
    x_list: Vec<usize>,
    mode: String,
    labels: Vec<String>,
}

struct Mono;
struct U8;
struct U32;

pub struct Config {
    img: DynamicImage,
    array_length: usize,
    image_rect_list: Vec<Rect>,
    label_list: Vec<String>,
}

impl From<ReadInformationSource> for Config {
    fn from(src: ReadInformationSource) -> Self {
        let ReadInformationSource {
            img_path,
            prefix,
            height,
            array_length,
            x_list,
            mode,
            labels,
        } = src;

        let mut x = 0;
        let image_rect_list = x_list
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
            .collect::<Vec<_>>();

        Self {
            array_length,
            img: image::open(img_path).unwrap(),
            image_rect_list,
            label_list: labels,
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
    let json = std::fs::read(file).unwrap();
    let src: ReadInformationSource = serde_json::from_slice(&json).unwrap();
    let prefix = src.prefix.clone();
    let array_length = src.array_length;
    let mode = src.mode.clone();

    let v = if mode == "bool" {
        Processor::new(Mono, src.into()).into_vec()
    } else if mode == "u8" {
        Processor::new(U8, src.into()).into_vec()
    } else {
        Processor::new(U32, src.into()).into_vec()
    };

    let out = v
        .into_iter()
        .fold("".to_string(), |a, (label, width, height, data)| {
            a + &format!(
                "const {prefix:}_{label:}: (usize, usize, [{mode:}; {array_length:}]) = ({w:}, {h:}, [{array:}]);\n",
                mode = mode,
                array_length = array_length,
                prefix = prefix.to_uppercase(),
                label = label.to_uppercase(),
                w = width,
                h = height,
                array = data.join(", ")
            )
        });
    println!("{}", out);
}

struct Processor<T> {
    target: Config,
    mode: T,
}

impl<T> Processor<T> {
    fn new(mode: T, target: Config) -> Self {
        Self { target, mode }
    }
}

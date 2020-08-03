use image::DynamicImage;
use image::GenericImageView;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct ReadInformationSource {
    img_path: String,
    rs_name: String,
    height: usize,
    x_list: Vec<usize>,
    labels: Vec<String>,
}

struct ReadInformation {
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
struct Rect {
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
    let rs_name = src.rs_name.clone();

    let result = to_detail(src.into());

    let mut t = "".to_string();
    for (label, width, height, data) in result {
        t += &format!(
            "    pub static ref VEC_{}: (usize, usize, Vec<u8>) = ({}, {}, vec!{:?});\n",
            label.to_uppercase(),
            width,
            height,
            data
        );
    }

    let whole = format!("lazy_static! {{\n{}}}\n", t);
    println!("{}", whole);

    std::fs::write(format!("./tmp/{}.rs", rs_name), whole).unwrap();
}

fn to_detail(target: ReadInformation) -> Vec<(String, usize, usize, Vec<usize>)> {
    let mut result = vec![];

    for (rect, label) in target.rests.into_iter().zip(target.labels) {
        let Rect {
            x, width, height, ..
        } = rect;

        let mut px_vec = vec![];

        for y in 0..height {
            for step_x in 0..width {
                let px = target.img.get_pixel((x + step_x) as u32, y as u32).0;
                let not_white = px.iter().fold(false, |a, n| a || *n != 255);

                if not_white {
                    px_vec.push(1);
                } else {
                    px_vec.push(0);
                }
            }
        }

        printer(width, px_vec.as_ref());

        result.push((label, width, height, px_vec));
    }

    result
}

fn printer(w: usize, v: &[usize]) {
    let t = v
        .chunks(w)
        .into_iter()
        .map(|row| {
            row.into_iter()
                .map(|n| if *n == 1 { "+" } else { " " })
                .collect::<Vec<_>>()
                .join("")
        })
        .collect::<Vec<_>>()
        .join("\n");

    println!("---\n{}\n---\n", t);
}

use crate::{ReadInformation, Rect};
use image::GenericImageView;

pub fn to_vec(target: ReadInformation) -> Vec<(String, usize, usize, Vec<usize>)> {
    let mut result = vec![];

    for (rect, label) in target.rests.into_iter().zip(target.labels) {
        let Rect {
            x, width, height, ..
        } = rect;

        let mut px_vec = vec![0; width * height];

        for y in 0..height {
            for step_x in 0..width {
                let px = target.img.get_pixel((x + step_x) as u32, y as u32).0;
                let not_white = px.iter().fold(false, |a, n| a || *n != 255);

                if not_white {
                    px_vec[width * y + step_x] = 1;
                }
            }
        }

        trace(width, px_vec.as_ref());

        result.push((label, width, height, px_vec));
    }

    result
}

pub fn trace(w: usize, v: &[usize]) {
    let t = v
        .chunks(w)
        .into_iter()
        .map(|row| {
            row.into_iter()
                .map(|n| if *n == 1 { "■" } else { "□" })
                .collect::<Vec<_>>()
                .join("")
        })
        .collect::<Vec<_>>()
        .join("\n");

    eprintln!("---\n{}\n---\n", t);
}

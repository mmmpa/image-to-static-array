use crate::{ReadInformation, Rect};
use image::GenericImageView;

pub fn compute_32_length(src: usize) -> usize {
    match src >> 5 {
        m if m == 0 => 1,
        m if src % 32 == 0 => m,
        m => m + 1,
    }
}

pub fn to_vec(target: ReadInformation) -> Vec<(String, usize, usize, Vec<u32>)> {
    let mut result = vec![];

    for (rect, label) in target.rests.into_iter().zip(target.labels) {
        let Rect {
            x, width, height, ..
        } = rect;

        let mut px_vec = vec![0; compute_32_length(width * height)];

        for y in 0..height {
            for step_x in 0..width {
                let px = target.img.get_pixel((x + step_x) as u32, y as u32).0;
                let not_white = px.iter().fold(false, |a, n| a || *n != 255);

                if not_white {
                    let pos = width * y + step_x;
                    px_vec[pos >> 5] |= 1 << 31 - (pos % 32);
                }
            }
        }

        trace(width, height, &px_vec);

        result.push((label, width, height, px_vec));
    }

    result
}

pub fn trace(w: usize, h: usize, v: &[u32]) {
    let t = v
        .into_iter()
        .fold("".to_string(), |a, n| a + &format!("{:>032b}", n))
        .split("")
        .filter(|s| *s != "")
        .take(w * h)
        .collect::<Vec<_>>()
        .chunks(w)
        .into_iter()
        .map(|row| {
            row.into_iter()
                .map(|n| if *n == "1" { "■" } else { "□" })
                .collect::<Vec<_>>()
                .join("")
        })
        .collect::<Vec<_>>()
        .join("\n");

    eprintln!("---\n{}\n---\n", t);
}

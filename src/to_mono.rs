use crate::{Config, Mono, Processor, Rect};
use image::GenericImageView;

impl Processor<Mono> {
    pub fn into_vec(self) -> Vec<(String, usize, usize, Vec<String>)> {
        let target = self.target;
        let mut result = vec![];

        for (rect, label) in target.image_rect_list.into_iter().zip(target.label_list) {
            let Rect {
                x, width, height, ..
            } = rect;

            let mut px_vec = vec![false; target.array_length];

            for y in 0..height {
                for step_x in 0..width {
                    let px = target.img.get_pixel((x + step_x) as u32, y as u32).0;
                    let not_white = px.iter().fold(false, |a, n| a || *n != 255);

                    if not_white {
                        px_vec[width * y + step_x] = true;
                    } else {
                        px_vec[width * y + step_x];
                    }
                }
            }

            Self::trace(width, px_vec.as_ref());

            result.push((
                label,
                width,
                height,
                px_vec
                    .into_iter()
                    .map(|n| (n as u8).to_string())
                    .collect::<Vec<_>>(),
            ));
        }

        result
    }

    pub fn trace(w: usize, v: &[bool]) {
        let t = v
            .chunks(w)
            .into_iter()
            .map(|row| {
                row.into_iter()
                    .map(|n| if *n { "■" } else { "□" })
                    .collect::<Vec<_>>()
                    .join("")
            })
            .collect::<Vec<_>>()
            .join("\n");

        eprintln!("---\n{}\n---\n", t);
    }
}

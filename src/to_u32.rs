use crate::{Config, Processor, Rect, U32};
use image::GenericImageView;

impl Processor<U32> {
    pub fn compute_32_length(src: usize) -> usize {
        match src >> 5 {
            m if m == 0 => 1,
            m if src % 32 == 0 => m,
            m => m + 1,
        }
    }

    pub fn into_vec(self) -> Vec<(String, usize, usize, Vec<String>)> {
        let target = self.target;
        let mut result = vec![];

        for (rect, label) in target.image_rect_list.into_iter().zip(target.label_list) {
            let Rect {
                x, width, height, ..
            } = rect;

            let mut px_vec = vec![0; target.array_length];

            for y in 0..height {
                for step_x in 0..width {
                    let px = target.img.get_pixel((x + step_x) as u32, y as u32).0;
                    let not_white = px.iter().fold(false, |a, n| a || *n != 255);

                    let pos = width * y + step_x;
                    if not_white {
                        px_vec[pos >> 5] |= 1 << 31 - (pos % 32);
                    } else {
                        px_vec[pos >> 5];
                    }
                }
            }

            Self::trace(width, height, &px_vec);

            result.push((
                label,
                width,
                height,
                px_vec
                    .into_iter()
                    .map(|n| n.to_string())
                    .collect::<Vec<_>>(),
            ));
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
}

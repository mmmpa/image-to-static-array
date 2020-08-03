# image_to_vec_static

This create rs file that defines static Vec<u8> by lazy_static for mono image data.

# Usage

## Command

```
cargo run "./tmp/nums.json"
```

## Json

`nums.png` is a monochrome image.

```json
{
  "img_path": "./tmp/nums.png",
  "rs_name": "nums",
  "height": 10,
  "x_list": [3,5,5,5,5,5,5,5,5,5,2],
  "labels": ["1","2","3","4","5","6","7","8","9","0", "preriod"]
}
```

## Result

We get `tmp/nums.rs` with following.

```rs
lazy_static! {
    pub static ref VEC_1: (usize, usize, Vec<u8>) = (3, 10, vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 1, 0, 0, 1, 0, 0, 1, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0]);
    pub static ref VEC_2: (usize, usize, Vec<u8>) = (5, 10, vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
    // snip
    pub static ref VEC_PRERIOD: (usize, usize, Vec<u8>) = (2, 10, vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0]);
}
```

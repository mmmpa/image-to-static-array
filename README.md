# image_to_vec_static

This create rs file that defines static Vec<u8> by lazy_static for mono image data.

# Usage

## Command

```
cargo run "./tmp/nums.json" > "./tmp/vec_mono.rs"
```

```rust
lazy_static! {
    pub static ref VEC_1: (usize, usize, Vec<u8>) = (3, 10, vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 1, 0, 0, 1, 0, 0, 1, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0]);
    pub static ref VEC_2: (usize, usize, Vec<u8>) = (5, 10, vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
    // snip
    pub static ref VEC_PRERIOD: (usize, usize, Vec<u8>) = (2, 10, vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0]);
}
```

## Command for u8

```
cargo run "./tmp/nums.json" 8 > "./tmp/vec_u8.rs"
```

```rust
lazy_static! {
    pub static ref VEC_NUM_1: (usize, usize, Vec<u32>) = (3, 10, vec![0, 44, 151, 0]);
    pub static ref VEC_NUM_2: (usize, usize, Vec<u32>) = (5, 10, vec![0, 0, 232, 136, 159, 0, 0]);
    // snip
    pub static ref VEC_NUM_PERIOD: (usize, usize, Vec<u32>) = (2, 10, vec![0, 15, 0]);
}
```

## JSON format

`nums.png` is a monochrome image.

```json
{
  "img_path": "./tmp/nums.png",
  "prefix": "vec_num",
  "height": 10,
  "x_list": [3,5,5,5,5,5,5,5,5,5,2],
  "labels": ["1","2","3","4","5","6","7","8","9","0", "period"]
}
```

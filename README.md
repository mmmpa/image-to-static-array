# image_to_vec_static

This create rs file that defines static Vec<u8> by lazy_static for mono image data.

# Usage

## Command

```
cargo run "./tmp/nums.json" > "./tmp/mono.rs"
```

```rs
lazy_static! {
    pub static ref VEC_1: (usize, usize, Vec<u8>) = (3, 10, vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 1, 0, 0, 1, 0, 0, 1, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0]);
    pub static ref VEC_2: (usize, usize, Vec<u8>) = (5, 10, vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
    // snip
    pub static ref VEC_PRERIOD: (usize, usize, Vec<u8>) = (2, 10, vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0]);
}
```

## Command for u8

```
cargo run "./tmp/nums.json" 32 > "./tmp/u32.rs"
```

```rs
lazy_static! {
    pub static ref VEC_NUM_1: (usize, usize, Vec<u32>) = (3, 10, vec![0, 44, 151, 0]);
    pub static ref VEC_NUM_2: (usize, usize, Vec<u32>) = (5, 10, vec![0, 0, 232, 136, 159, 0, 0]);
    pub static ref VEC_NUM_3: (usize, usize, Vec<u32>) = (5, 10, vec![0, 0, 232, 132, 193, 139, 128]);
    pub static ref VEC_NUM_4: (usize, usize, Vec<u32>) = (5, 10, vec![0, 0, 35, 42, 95, 16, 128]);
    pub static ref VEC_NUM_5: (usize, usize, Vec<u32>) = (5, 10, vec![0, 0, 244, 56, 33, 139, 128]);
    pub static ref VEC_NUM_6: (usize, usize, Vec<u32>) = (5, 10, vec![34, 33, 232, 198, 46, 0, 0]);
    pub static ref VEC_NUM_7: (usize, usize, Vec<u32>) = (5, 10, vec![0, 1, 248, 200, 68, 33, 0]);
    pub static ref VEC_NUM_8: (usize, usize, Vec<u32>) = (5, 10, vec![116, 98, 232, 198, 46, 0, 0]);
    pub static ref VEC_NUM_9: (usize, usize, Vec<u32>) = (5, 10, vec![0, 0, 232, 197, 225, 17, 0]);
    pub static ref VEC_NUM_0: (usize, usize, Vec<u32>) = (5, 10, vec![0, 0, 232, 198, 46, 0, 0]);
    pub static ref VEC_NUM_PERIOD: (usize, usize, Vec<u32>) = (2, 10, vec![0, 15, 0]);
}
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

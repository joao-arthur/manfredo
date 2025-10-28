# Manfredo

> Manfredo do Carmo (1928 - 2018) was a Brazilian mathematician known for his work in differential
> geometry.

[![CI](https://github.com/joao-arthur/manfredo/actions/workflows/ci.yaml/badge.svg)](https://github.com/joao-arthur/manfredo/actions/workflows/ci.yaml)
[![codecov](https://codecov.io/gh/joao-arthur/manfredo/graph/badge.svg?token=D1ZDT1FBZM)](https://codecov.io/gh/joao-arthur/manfredo)

Manfredo is a **free sofware and open-source (FLOSS)** geometry library. It is designed around the
philosophy of:

- **Flexibility** to choose the **structs** (signed, unsigned or float-point data) and **functions**
  (mutating, non-mutating) you need
- **Minimal abstractions**, with explicit side effects
- **Zero dependencies**

Currently, Manfredo only supports **2D geometry**.

## 📦️ Install

```toml
[dependencies]
manfredo = { git = "https://github.com/joao-arthur/manfredo", rev = "ed9fc12740a051c0c98d6278c5109b45d0679e58" }
```

## Data Structures

- **Cartesian plane** suited for continuous or signed coordinate spaces
    - `Point` and `Rect` available as **unsigned**, **signed**, and **floating-point**

- **Matrix plane** suited for discrete, index-based coordinate spaces
    - `Point` and `Rect` available as **unsigned** and **signed**.

## Operations

They follow a consistent naming pattern. For operations that are not garanteed to succeed:

- **try_operation_assign**: Attempts the operation and assigns the result if successful
- **operation_assign**: Infallible version, panics if the operation fails
- **try_operation**: Returns the result as a new value if successful
- **operation**: Infallible version, panics if the operation fails

For operations that always succeed:

- **operation_assign**: Assigns the result
- **operation**: Returns the result as a new value

---

Arithmetic operations have **explicit numeric behavior**, that is exposed in the operation name:

- **checked**: When the result would overflow or undeflow, it returns None
- **saturating**: When the result would overflow, returns the MAX, when the result would underflow,
  returns MIN
- **wrapping**: Explicitly allows the result to overflow or underflow

## In Practice

Manfredo deliberately avoids traits such as `Add`, `Sub`, or `Mul`. Instead, it prefers **explicit
functions**.

For example, the following code:

```rust
let a = rect_i32::Rect::of(-20, -10, 10, 20);
let b = rect_i32::Rect::of(-20, -10, 10, 20);
let c = a + b;
let d = a + b;
let e = a + b;
```

This kind of code, altough providing a good developer experience, hide two problems:

- It does not allow mutating directly _a_ or _b_
- if x1, y1, y2, y2 overflows or underflow, the operation should fail, clamp or wrap?

For this reason, Manfredo exposes this operation instead as:

```rust
let mut a = rect_i32::Rect::of(-20, -10, 10, 20);
let b = rect_i32::Rect::of(-20, -10, 10, 20);
let c = rect_i32::try_checked_add(&a, &b);
let d = rect_i32::saturating_add(&a, &b);
let e = rect_i32::wrapping_add_assign(&mut a, &b);
```

## 🚧 Roadmap

- [-] Cartesian Plane
  - [-] 1D
    - [x] `Point`
      - [x] add
      - [x] delta
    - [-] `Line`
      - [ ] delta
  - [-] 2D
    - [x] `Point`
      - [x] add
      - [x] delta
      - [x] distance
    - [-] `Line`
      - [ ] delta
      - [ ] distance
    - [x] `Circle`
      - [x] area
    - [x] `Rect`
      - [x] add
      - [x] delta
      - [x] inflate
      - [x] resize
      - [x] translate
      - [x] area
      - [x] contains_point
      - [x] contains_rect
      - [x] deflate
      - [x] len (can overflow)
  - [-] 3D
    - [-] `Point`
      - [ ] add
      - [x] delta
      - [ ] distance
    - [-] `Line`
      - [ ] delta
      - [ ] distance
    - [ ] `Sphere`
      - [ ] area
    - [-] `Rect`
      - [ ] add
      - [ ] delta
      - [ ] inflate
      - [ ] resize
      - [ ] translate
      - [ ] area
      - [ ] contains_point
      - [ ] contains_rect
      - [ ] deflate
      - [ ] len (can overflow)
  - [-] 4D
    - [-] `Point`
      - [ ] add
      - [x] delta
      - [ ] distance
    - [-] `Line`
      - [ ] delta
      - [ ] distance
    - [ ] `Sphere`
      - [ ] area
    - [-] `Rect`
      - [ ] add
      - [ ] delta
      - [ ] inflate
      - [ ] resize
      - [ ] translate
      - [ ] area
      - [ ] contains_point
      - [ ] contains_rect
      - [ ] deflate
      - [ ] len (can overflow)
- [-] Matrix Plane
  - [-] 1D
    - [x] `Point`
      - [x] add
      - [x] delta
    - [-] `Line`
      - [ ] delta
  - [-] 2D
    - [x] `Point`
      - [x] add
      - [x] delta
    - [-] `Line`
      - [ ] delta
    - [x] `Rect`
      - [x] add
      - [x] delta
      - [x] inflate
      - [x] resize
      - [x] translate
      - [x] area
      - [x] contains_point
      - [x] contains_rect
      - [x] deflate
      - [x] len (can overflow)
  - [-] 3D
    - [-] `Point`
      - [ ] add
      - [x] delta
    - [-] `Line`
      - [ ] delta
      - [ ] distance
    - [-] `Rect`
      - [ ] add
      - [ ] delta
      - [ ] inflate
      - [ ] resize
      - [ ] translate
      - [ ] area
      - [ ] contains_point
      - [ ] contains_rect
      - [ ] deflate
      - [ ] len (can overflow)
  - [-] 4D
    - [-] `Point`
      - [ ] add
      - [x] delta
    - [-] `Line`
      - [ ] delta
      - [ ] distance
    - [-] `Rect`
      - [ ] add
      - [ ] delta
      - [ ] inflate
      - [ ] resize
      - [ ] translate
      - [ ] area
      - [ ] contains_point
      - [ ] contains_rect
      - [ ] deflate
      - [ ] len (can overflow)
- [ ] Transform
  - [ ] 1D
  - [x] 2D
  - [ ] 3D
  - [ ] 4D
- [ ] Polar plane

## 📜 License

[GNU AGPLv3](./LICENSE)

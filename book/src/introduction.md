# Introduction

> Manfredo do Carmo (1928 - 2018) was a Brazilian mathematician known for his work in differential
> geometry.

Manfredo is a **free sofware and open-source (FLOSS)** geometry library. It is designed around the
philosophy of:

- **Flexibility** to choose the **structs** (signed, unsigned or float-point data) and **functions**
  (mutating, non-mutating) you need
- **Minimal abstractions**, with explicit side effects
- **Zero dependencies**

Currently, Manfredo only supports **2D geometry**.

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
let a = rect_i32::Rect::new((-20, -10), (10, 20));
let b = rect_i32::Rect::new((-20, -10), (10, 20));
let c = a + b;
let d = a + b;
let e = a + b;
```

This kind of code, altough providing a good developer experience, hide two problems:

- It does not allow mutating directly _a_ or _b_
- if x1, y1, y2, y2 overflows or underflow, the operation should fail, clamp or wrap?

For this reason, Manfredo exposes this operation instead as:

```rust
let mut a = rect_i32::Rect::new((-20, -10), (10, 20));
let b = rect_i32::Rect::new((-20, -10), (10, 20));
let c = rect_i32::try_checked_add(&a, &b);
let d = rect_i32::saturating_add(&a, &b);
let e = rect_i32::wrapping_add_assign(&mut a, &b);
```

## Roadmap

- 3D geometry
- Polar plane

## License

GNU AGPLv3

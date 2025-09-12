# Matrix Rect

## Variations

- **add** → sums two rectangles
- **contains_point** → returns wether the point is in the **closed interval** of the rectangle
- **contains_rect** → returns wether another rectangle is in the **closed interval** of the
  rectangle
- **inflate** → increases the dimensions of the rectangle by 1 unit
- **deflate** → decreases the dimensions of the rectangle by 1 unit
- **resize** → resizes the rectangle
- **translate** → moves the rectangle by the delta

### Unsigned

- rect_u8::Rect
- rect_u16::Rect
- rect_u32::Rect
- rect_u64::Rect

### Signed

> Consider using **[Cartesian Rect](../cartesian/rect.md) instead**

- rect_i8::Rect
- rect_i16::Rect
- rect_i32::Rect
- rect_i64::Rect

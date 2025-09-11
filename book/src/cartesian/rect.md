# Cartesian Rectangle

## Operations

- **add** → sums two rectangles
- **contains_point** → returns wether the point is in the **closed interval** of the rectangle
- **contains_rect** → returns wether another rectangle is in the **closed interval** of the
  rectangle
- **inflate** → increases the dimensions of the rectangle by 1 unit
- **deflate** → decreases the dimensions of the rectangle by 1 unit
- **resize** → resizes the rectangle
- **translate** → moves the rectangle by the delta

### Unsigned

> Consider using **[Matrix Rect](../matrix/rect.md) instead**

- RectU8
- RectU16
- RectU32
- RectU64

### Signed

- Rect
- Rect
- Rect
- RectI64

### Float-point

- RectF32
- RectF64

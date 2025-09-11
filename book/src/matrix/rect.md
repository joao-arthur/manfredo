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

- RectU8
- RectU16
- RectU32
- RectU64

### Signed

> Consider using **[Cartesian Rect](../cartesian/rect.md) instead**

- Rect
- Rect
- RectI32
- RectI64

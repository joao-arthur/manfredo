# Matrix Plane

Represents space using a **finite grid of discrete cells**, described by **row** and **column**
indices in _2D_, where `(0, 0)` corresponds to the **top-left**.

Each element is accessed as `(row, col)`, both being **non-negative integers**.

## Direction

- **+Row** → down
- **-Row** → up
- **+Col** → right
- **-Col** → left

### Use Case

Working with **bounded grids or tabular data**, where elements must be accessed by index. Example:

- Chess board (8 × 8)
- Sudoku grid (9 × 9)
- HTML Canvas pixel buffer
- Image processing

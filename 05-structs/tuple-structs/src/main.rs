fn main() {
    // NOTE: Color and Point both have the same field types, but THEY ARE OF DIFFERENT TYPES.
    // NOTE: A function that expects Color won't accept Point.
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);
}

# Mutable vs. Immutable Borrowing in Rust

This example demonstrates a common error in Rust related to borrowing:

Trying to create both mutable and immutable references to the same variable at the same time will lead to a compile-time error, because the compiler cannot ensure the memory safety.
# Unintentional Iterator Consumption in Rust
This repository demonstrates a common error in Rust: unintentionally consuming an iterator.  The example shows how to correctly handle iterators to avoid panics and unexpected behavior.

## The Bug
The `bug.rs` file contains code that attempts to access elements of a vector using an iterator. However, the iterator is consumed after the first element is accessed, causing a panic when trying to access the second element.  This is a common mistake when working with iterators in Rust.

## The Solution
The `bugSolution.rs` file demonstrates the corrected approach. The problem is solved by cloning the iterator before consuming it.
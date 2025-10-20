fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    // TODO: Fix the compiler errors only by reordering the lines in the test.
    // Don't add, change or remove any line.
    #[test]
    fn move_semantics4() {
        let mut x = Vec::new(); // Create a mutable Vec on the heap
        let y = &mut x; // Create a mutable reference to x (borrow x mutably)
        y.push(42); // Push 42 onto the Vec (accessed through y)
        let z = &mut x; // Create another mutable reference to x (y is no longer used)
        z.push(13); // Push 13 onto the Vec (accessed through z)
        assert_eq!(x, [42, 13]);
    }
}

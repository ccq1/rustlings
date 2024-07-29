fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    #[test]
    fn slice_out_of_array() {
        let a = [1, 2, 3, 4, 5];

        // TODO: Get a slice called `nice_slice` out of the array `a` so that the test passes.
        // let nice_slice = ???
        let nice_slice = &a[1..4];
        
        for i in 0..nice_slice.len() {
            println!("Element {} = {}", i, nice_slice[i]);
        }
        assert_eq!([2, 3, 4], nice_slice);
    }
}

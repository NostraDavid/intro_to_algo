// The actual algorithm function
pub fn insertion_sort(input: &mut [i32]) {
    // TODO: replac this with my own implementation
    input.sort();
}

// The run function generates data, runs the algorithm, and prints the result
pub fn run() {
    // Step 1: Data generation
    let mut data = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
    println!("Original data: {:?}", data);

    // Step 2: Run the algorithm
    insertion_sort(&mut data);

    // Step 3: Print the result
    println!("Sorted data: {:?}", data);
}

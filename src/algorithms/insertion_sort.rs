// Insertion Sort algorithm
// Works by iterating through the array and inserting each element into
// its correct sorted position
pub fn insertion_sort(input: &mut [i32]) {
    // Iterate through each element in the array
    for i in 1..input.len() {
        // Store the current element to be inserted
        let key = input[i];
        // Store the index of the element to the left of the current element
        let mut j = i;
        // Move elements to the right until a smaller element is found
        while j > 0 && input[j-1] > key {
            // Move the element to the right
            input[j] = input[j-1];
            // Move the index further left
            j -= 1;
        }
        // Insert the element at the correct position
        input[j] = key
    }
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

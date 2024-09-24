/// Insertion-Sort(A, n)
/// 1. for i = 2 to n
/// 2.     key = A[i]
/// 3.     // Insert  A[i]  into the sorted subarray  A[1 : i - 1].
/// 4.     j = i - 1
/// 5.     while j > 0 and A[j] > key
/// 6.         A[j + 1] = A[j]
/// 7.         j = j - 1
/// 8.     A[j + 1] = key
pub fn insertion_sort(a: &mut [i32]) {
    // using `a` instead of `A`, because Rust...
    // Iterate through each element in the array
    for i in 1..a.len() {
        // Store the current element to be inserted
        let key = a[i];
        // Store the index of the element to the left of the current element
        let mut j = i;
        // Move elements to the right until a smaller element is found
        while j > 0 && a[j - 1] > key {
            // Move the element to the right
            a[j] = a[j - 1];
            // Move the index to the left
            j -= 1;
        }
        // Insert the element at the correct position
        a[j] = key;
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

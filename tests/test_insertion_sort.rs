use proptest::prelude::*;
use intro_to_algo::algorithms::insertion_sort::insertion_sort;

proptest! {
    /// This property-based test checks if the algorithm sorts the data.
    #[test]
    fn test_algorithm_sorts_correctly(mut input in proptest::collection::vec(any::<i32>(), 0..100)) {
        // Keep a copy of the input before sorting
        let original = input.clone();

        // Run the sorting algorithm
        insertion_sort(&mut input);

        // Check that the output is sorted
        prop_assert!(input.windows(2).all(|w| w[0] <= w[1]));

        // Check that the output contains the same elements as the original
        prop_assert_eq!(input.len(), original.len());
        prop_assert!(input.iter().all(|x| original.contains(x)));
    }
}

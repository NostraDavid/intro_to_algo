#include <stdio.h>

int insertion_sort(int numbers[], int size) {

}

int main(char** argv, int argc) {
	int arr[] = {12,11,13,5,6};
	int n = sizeof(arr) / sizeof(arr[0]);

	for (int i = 0; i < n; i++) {
		printf("%d ", arr[i]);
	}
	printf("\n");

	insertion_sort(arr, n);
	
	for (int i = 0; i < n; i++) {
		printf("%d ", arr[i]);
	}
	printf("\n");

	return 0;
}

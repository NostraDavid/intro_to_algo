/*!
MERGE(A, p, q, r)
1   n_L = q - p + 1          // length of A[p : q]
2   n_R = r - q              // length of A[q + 1 : r]
3   let L[0 : n_L - 1] and R[0 : n_R - 1] be new arrays
4   for i = 0 to n_L - 1     // copy A[p : q] into L[0 : n_L - 1]
5       L[i] = A[p + i]
6   for j = 0 to n_R - 1     // copy A[q + 1 : r] into R[0 : n_R - 1]
7       R[j] = A[q + j + 1]
8   i = 0                    // i indexes the smallest remaining element in L
9   j = 0                    // j indexes the smallest remaining element in R
10  k = p                    // k indexes the location in A to fill
11  // As long as each of the arrays L and R contains an unmerged element,
    // copy the smallest unmerged element back into A[p : r].
12  while i < n_L and j < n_R
13      if L[i] <= R[j]
14          A[k] = L[i]
15          i = i + 1
16      else A[k] = R[j]
17          j = j + 1
18      k = k + 1
19  // Having gone through one of L and R entirely, copy the
    // remainder of the other to the end of A[p : r].
20  while i < n_L
21      A[k] = L[i]
22      i = i + 1
23      k = k + 1
24  while j < n_R
25      A[k] = R[j]
26      j = j + 1
27      k = k + 1
*/
fn merge(a: &mut [i32], p: usize, q: usize, r: usize) {
    let n_l = q - p + 1;
    let n_r = r - q;
    let mut l = vec![0; n_l];
    let mut r = vec![0; n_r];
    for i in 0..n_l {
        l[i] = a[p + i];
    }
    for j in 0..n_r {
        r[j] = a[q + j + 1];
    }
    let mut i = 0;
    let mut j = 0;
    let mut k = p;
    while i < n_l && j < n_r {
        if l[i] <= r[j] {
            a[k] = l[i];
            i += 1;
        } else {
            a[k] = r[j];
            j += 1;
        }
        k += 1;
    }

    while i < n_l {
        a[k] = l[i];
        i += 1;
        k += 1;
    }

    while j < n_r {
        a[k] = r[j];
        j += 1;
        k += 1;
    }
}

/*
MERGE-SORT(A, p, r)
1    if p ≥ r                  // zero or one element?
2        return
3    q = ⌊(p + r)/2⌋           // midpoint of A[p : r]
4    MERGE-SORT(A, p, q)       // recursively sort A[p : q]
5    MERGE-SORT(A, q + 1, r)   // recursively sort A[q + 1 : r]
6    // Merge A[p : q] and A[q + 1 : r] into A[p : r].
7    MERGE(A, p, q, r)
*/
fn _merge_sort(a: &mut [i32], p: usize, r: usize) {
    //! Can NOT handle empty lists, as r would become -1 and fail.
    //! This is also the actual
    if p >= r {
        return;
    }
    // Integer division rounds down, this does NOT return a float.
    let q = (p + r) / 2;
    _merge_sort(a, p, q);
    _merge_sort(a, q + 1, r);
    merge(a, p, q, r);
}

pub fn merge_sort(a: &mut [i32]) {
    /*! We're using an extra function to wrap the actual function, so we don't
    have to change out interface compared with insertion sort. */
    if a.len() <= 1 {
        return;
    }
    let len = a.len();
    _merge_sort(a, 0, len - 1);
}

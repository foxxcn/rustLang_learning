fn heapify(arr: &mut [i32], n: usize, i: usize) {
    let mut largest = i;
    let left = 2 * i + 1;
    let right = 2 * i + 2;

    if left < n && arr[left] > arr[largest] {
        largest = left;
    }

    if right < n && arr[right] > arr[largest] {
        largest = right;
    }

    if largest != i {
        arr.swap(i, largest);
        heapify(arr, n, largest);
    }
}

fn heap_sort(arr: &mut [i32]) {
    let n = arr.len();

    // Build heap (rearrange array)
    for i in (0..n / 2).rev() {
        heapify(arr, n, i);
    }

    // One by one extract elements
    for i in (1..n).rev() {
        arr.swap(0, i);
        heapify(arr, i, 0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_heap_sort() {
        let mut arr = [12, 11, 13, 5, 6, 7];
        heap_sort(&mut arr);
        assert_eq!(arr, [5, 6, 7, 11, 12, 13]);
    }

    #[test]
    fn test_heap_sort_empty() {
        let mut arr: [i32; 0] = [];
        heap_sort(&mut arr);
        assert_eq!(arr, []);
    }

    #[test]
    fn test_heap_sort_one_element() {
        let mut arr = [1];
        heap_sort(&mut arr);
        assert_eq!(arr, [1]);
    }

    #[test]
    fn test_heap_sort_sorted() {
        let mut arr = [1, 2, 3, 4, 5];
        heap_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_heap_sort_reverse_sorted() {
        let mut arr = [5, 4, 3, 2, 1];
        heap_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5]);
    }
}

fn main() {
    let mut arr = [12, 11, 13, 5, 6, 7];
    println!("Original array: {:?}", arr);
    heap_sort(&mut arr);
    println!("Sorted array: {:?}", arr);
}

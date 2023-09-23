/// - https://youtu.be/CI60af3hhS8
/// - https://en.wikipedia.org/wiki/Binary_heap
/// - https://leetcode.com/problems/k-closest-points-to-origin/
/// - https://doc.rust-lang.org/stable/std/collections/struct.BinaryHeap.html
///
/// Types Of Binary Tree:
/// - https://en.wikipedia.org/wiki/Binary_tree#Types_of_binary_trees
///
/// A heap data structure takes the form of a binary tree (each node
/// have two children).
/// They are usually used to implement priority queues.
///
/// - All levels of the tree, except possibly the last one (deepest) are fully filled.
/// - The nodes are filled from left to right.
///
/// - left node: 2i + 1
/// - right node: 2i + 2

#[derive(PartialEq)]
struct Point {
    distance: f32, // euclidean distance
    point: Vec<i32>,
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.distance.partial_cmp(&other.distance)
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.distance.partial_cmp(&other.distance).unwrap()
    }
}

impl Eq for Point {
    // fn assert_receiver_is_total_eq(&self) {}
}

pub struct Heap<T: Ord> {
    heap: Vec<T>,
}

impl<T: Ord> Heap<T> {
    pub fn new() -> Self {
        Self { heap: vec![] }
    }

    /// To add an element to a heap, we can perform this algorithm:
    ///     1. Add the element to the bottom level of the heap at the leftmost open space.
    ///     2. Compare the added element with its parent; if they are in the correct order, stop.
    ///     3. If not, swap the element with its parent and return to the previous step.
    pub fn push(&mut self, val: T) {
        self.heap.push(val);
        self.bubble_up();
    }

    /// The `pop` operation (also called "extract") removes or extracts the root node
    /// from the heap (the maximum element in a max-heap or the minimum element in a min-hea) while
    /// retaining the heap property is as follows:
    ///
    /// See: https://en.wikipedia.org/wiki/Binary_heap#Extract
    ///
    /// 1. Replace the root of the heap with the last element on the last level.
    /// 2. Compare the new root with its children; if they are in the correct order, stop.
    /// 3. If not, swap the element with one of its children and return to the previous step.
    ///    IMPORTANT: Swap with its smaller child in a min-heap and its larger child in a
    ///    max-heap.
    pub fn pop(&mut self) -> Option<T> {
        let len = self.heap.len();

        if len == 1 {
            return self.heap.pop();
        }

        if len > 1 {
            self.heap.swap(0, len - 1); // swap the root node with the rightmost child
            let root_node = self.heap.pop(); // extract the root node

            // because of the `swap` above now the root of the tree is the node
            // that should actually be at the rightmost place

            // we pass the index from which the bubble down should start. In
            // this case is the root node
            self.bubble_down(0);

            return root_node;
        }

        None
    }

    pub fn bubble_up(&mut self) {
        // we start from the end
        let mut i = self.heap.len() - 1;

        while i > 0 {
            let node = &self.heap[i];
            let parent_index = (i - 1) / 2; // this is the oposite to find the children of a node
            let parent_node = &self.heap[parent_index];

            if parent_node >= node {
                break;
            }

            self.heap.swap(i, parent_index);
            i = parent_index;
        }
    }

    pub fn bubble_down(&mut self, i: usize) {
        let left_child = (2 * i) + 1;
        let right_child = (2 * i) + 2;
        let mut largest = i;
        let len = self.heap.len();

        if left_child < len && self.heap[left_child] > self.heap[largest] {
            largest = left_child;
        }

        if right_child < len && self.heap[right_child] > self.heap[largest] {
            largest = right_child;
        }

        if largest != i {
            self.heap.swap(largest, i);
            self.bubble_down(largest);
        }
    }
}

fn k_nearest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
    let mut heap = Heap::new();

    for point in points {
        let distance: i32 = point.iter().map(|p| p.pow(2)).sum();
        let distance: f32 = (distance as f32).sqrt();
        heap.push(core::cmp::Reverse(Point { distance, point }))
    }

    let mut results = Vec::new();

    for _ in 0..k {
        if let Some(core::cmp::Reverse(v)) = heap.pop() {
            results.push(v.point);
        } else {
            break;
        }
    }

    results
}

fn main() {}

#[cfg(test)]
mod test {
    // Run the tests:
    //
    // ```bash
    // # Run the tests
    // cargo test --example k_nearest_point
    //
    // # Test & Watch
    // cargo watch -q -c -w examples/ -x 'test --example k_nearest_point'
    // ```

    #[test]
    fn works_with_empty_points() {
        assert_eq!(super::k_nearest(vec![], 3).len(), 0);
    }

    #[test]
    fn works_with_bad_k() {
        assert_eq!(
            super::k_nearest(vec![vec![3, 9], vec![2, 2], vec![1, 1]], -4).len(),
            0
        );
    }

    #[test]
    fn works_with_one_point() {
        assert_eq!(super::k_nearest(vec![vec![0, 0]], 3), vec![vec![0, 0]]);
    }

    #[test]
    fn works_with_top_k_points() {
        assert_eq!(
            super::k_nearest(vec![vec![3, 9], vec![2, 2], vec![1, 1]], 2),
            vec![vec![1, 1], vec![2, 2]],
        );
    }
}

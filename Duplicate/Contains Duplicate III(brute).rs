// nums: 2 to 100_000 values, numbers ranging in +- 1_000_000_000
// indexDiff: 1 to nums.len
// valueDiff: 0 to 1_000_000_000
// find (i, j) where:
//     i != j
//     abs(i - j) <= indexDiff
//     abs(nums[i] - nums[j]) <= valueDiff
// true iff (i, j) exists
// find two elements at most indexDiff positions apart with an abs_diff of at most valueDiff

// brute force, start at every left position:
//   iterate out to at most indexDiff away:
//      if any of the values had the abs_diff <= valueDiff, return true
// this iterates every element, with an inner iteration of up to every element
// O(n^2)

// how can we do better?
// We need to hit O(n) or O(n log n) likely
// sorting would likely cause difficulties with back-tracking indices, might work
// seems like a sliding window kind of thing, we don't actually need to examine smaller windows
// fixed size window of indexDiff
// how can we efficiently maintain state?
// A step removes one from the left, and adds in one on the right
// we then need to answer: does anything in the window have abs_diff <= valueDiff
// which means we want to find the two closest numbers in the window
// since we're early exiting we can say that our window DOES NOT contain such a pair as our invariant:
// it kind of feels like we want a heap, but that's min and max
// our update needs to take at most O(log n)
// so our state needs to be, e.g. a sorted array
// with a sorted array, we can insert a number at a position, and check nieghbours,
// as they are by definition closest
// we can also find and remove, no check needed
// so maintain a sorted view of the window and iterate
// for O(n log n)
// 
// SORTED VECTOR

struct Window {
    elements: Vec<i32>,
}

impl Window {
    fn new(slice: &[i32], max_len: usize) -> (Self, i32) {
        let taking = slice.len().min(max_len);
        let mut elements: Vec<i32> = slice[0..taking].into_iter().copied().collect();
        elements.sort_unstable();
        let min_diff = elements.windows(2).map(|window| {
            assert_eq!(window.len(), 2);
            window[0].abs_diff(window[1])
        }).min().expect("empty initial window");
        (Self { elements }, min_diff as i32)
    }

    fn pop_left(&mut self, x: i32) {
        let index = self.elements.binary_search(&x).expect("tried to pop element not in window");
        self.elements.remove(index);
    }

    // returns smallest abs_diff for x
    fn push_right(&mut self, x: i32) -> i32 {
        assert!(!self.elements.is_empty());
        match self.elements.binary_search(&x) {
            // element was present already! we don't even need to maintain state
            Ok(index) => 0,
            Err(0) => {
                let diff = self.elements[0].abs_diff(x) as i32;
                self.elements.insert(0, x);
                diff
            }
            Err(i) => {
                // i > 0
                let mut diff = self.elements[i - 1].abs_diff(x) as i32;
                if let Some(rite) = self.elements.get(i) {
                    diff = diff.min(rite.abs_diff(x) as i32);
                }
                self.elements.insert(i, x);
                diff
            }
        }

    }
}

impl Solution {
    pub fn contains_nearby_almost_duplicate(nums: Vec<i32>, index_diff: i32, value_diff: i32) -> bool {
        let index_diff = index_diff as usize;
        let (mut window, min_diff) = Window::new(&nums, index_diff + 1);
        if min_diff <= value_diff { return true; }

        let endpoint = (nums.len() - index_diff).saturating_sub(1);
        let mut i = 0;
        for i in 1..=endpoint {
            window.pop_left(nums[i - 1]);
            if window.push_right(nums[i + index_diff]) <= value_diff {
                return true;
            }
        }

        false
    }
}


# Contains Duplicate Pattern Notes

## The Main Idea

All "Contains Duplicate" problems are really asking:

```text
"How do I remember previous elements efficiently?"
```

That is the entire pattern.

---

# LEVEL 1 — Exact Duplicate

## Example

* Contains Duplicate

## Problem Meaning

```text
"Have I seen this exact number before?"
```

---

## Brain Trigger

When you see:

* duplicate
* repeated
* already exists
* unique
* distinct

Think:

```text
HashSet
```

---

## Core Logic

Walk through array:

For each number:

* if already in set → duplicate found
* else insert it

---

## Rust Skeleton

```rust
use std::collections::HashSet;

let mut set = HashSet::new();

for num in nums {
    if !set.insert(num) {
        return true;
    }
}

false
```

---

## Complexity

| Time | Space |
| ---- | ----- |
| O(n) | O(n)  |

---

# LEVEL 2 — Nearby Duplicate

## Example

* Contains Duplicate II

## New Condition

```text
duplicates must be CLOSE
```

Meaning:

```text
abs(i - j) <= k
```

---

## Brain Trigger

When you see:

* nearby
* within k
* recent
* last k elements

Think:

```text
Sliding Window
```

---

## Core Logic

Only keep:

* last k elements

Throw away old ones.

---

## Mental Model

```text
small moving box
```

Inside the box:

* only recent elements

---

## Rust Skeleton

```rust
use std::collections::HashSet;

let mut window = HashSet::new();

for i in 0..nums.len() {

    if i > k {
        window.remove(&nums[i - k - 1]);
    }

    if !window.insert(nums[i]) {
        return true;
    }
}

false
```

---

## Important Line

```rust
nums[i - k - 1]
```

Means:

```text
remove the OLD element leaving the window
```

---

## Complexity

| Time | Space |
| ---- | ----- |
| O(n) | O(k)  |

---

# LEVEL 3 — Nearby ALMOST Duplicate

## Example

* Contains Duplicate III

## New Condition

Now values can be:

```text
CLOSE
```

not necessarily equal.

---

## Problem Meaning

```text
abs(nums[i] - nums[j]) <= valueDiff
```

Meaning:

```text
find numbers within a VALUE RANGE
```

---

## Brain Trigger

When you see:

* close values
* ranges
* nearby values
* <= valueDiff

Think:

```text
Ordered Structure
Range Search
Buckets
TreeSet
```

NOT plain HashSet.

---

## Core Idea

Now you need:

* sliding window
* AND fast range checking

This problem is much harder because it combines:

* index distance
* value distance

---

# MASTER PATTERN TABLE

| Problem Type     | Main Idea         | Main Structure            |
| ---------------- | ----------------- | ------------------------- |
| Exact duplicate  | seen before?      | HashSet                   |
| Nearby duplicate | recent duplicates | Sliding Window + HashSet  |
| Almost duplicate | nearby values     | Ordered Structure/Buckets |
| Count frequency  | count occurrences | HashMap                   |
| Closest/recent   | moving range      | Sliding Window            |

---

# SLIDING WINDOW MENTAL MODEL

Sliding window means:

```text
Keep recent stuff.
Throw away old stuff.
```

Usually appears when problem says:

* nearby
* within k
* substring
* consecutive
* recent
* range
* moving interval

---

# GENERAL DSA THINKING PROCESS

## Step 1

Ask:

```text
What am I trying to remember?
```

Examples:

* seen values
* recent values
* frequencies
* positions

---

## Step 2

Ask:

```text
Does order matter?
```

If YES:

* sorting
* heap
* tree
* deque

---

## Step 3

Ask:

```text
Do I care about ONLY recent elements?
```

If YES:

```text
Sliding Window
```

---

## Step 4

Ask:

```text
Am I checking:
- exact values?
OR
- nearby/range values?
```

Exact:

* HashSet

Range:

* ordered structure

---

# 10 GOOD PRACTICE QUESTIONS

## Easy

1. Contains Duplicate
2. Contains Duplicate II
3. Valid Anagram
4. Intersection of Two Arrays
5. Happy Number

---

## Sliding Window

6. Longest Substring Without Repeating Characters
7. Maximum Average Subarray
8. Permutation in String

---

## Harder Duplicate/Range Style

9. Contains Duplicate III
10. Top K Frequent Elements

---

# MOST IMPORTANT LESSON

Strong DSA is NOT memorizing solutions.

It is learning:

```text
problem clue -> data structure pattern
```

Examples:

| Clue            | Structure                |
| --------------- | ------------------------ |
| duplicate       | HashSet                  |
| frequency       | HashMap                  |
| nearby          | Sliding Window           |
| min/max quickly | Heap                     |
| ordered values  | Sorting/Tree             |
| ranges          | Prefix Sum / Ordered Set |

That pattern recognition is the real skill.

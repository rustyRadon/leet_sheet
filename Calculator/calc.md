# Basic Calculator - Complete Line by Line Explanation 


```rust
use std::collections::VecDeque;

impl Solution {
    pub fn calculate(s: String) -> i32 {
        let bytes = s.as_bytes();
        let mut stack = Vec::with_capacity(16);  
        let mut result = 0;
        let mut sign = 1;
        let mut i = 0;

        while i < bytes.len() {
            match bytes[i] {
                b'0'..=b'9' => {
                    let mut num = 0;
                    while i < bytes.len() && bytes[i].is_ascii_digit() {
                        num = num * 10 + (bytes[i] - b'0') as i32;
                        i += 1;
                    }
                    result += sign * num;
                    continue;
                }
                b'+' => sign = 1,
                b'-' => sign = -1,
                b'(' => {
                    stack.push((result, sign));
                    result = 0;
                    sign = 1;
                }
                b')' => {
                    let (prev_result, prev_sign) = stack.pop().unwrap();
                    result = prev_result + prev_sign * result;
                }
                _ => {}
            }
            i += 1;
        }
        result
    }
}
```

---

##  Explanation

###  `use std::collections::VecDeque;`

**What it means:**
We're borrowing a special box from Rust's library. This box can hold things and we can take things from the front or back.
It's like asking your mom for a container to store your toys. This container can give you toys from the top OR bottom.

**Why we need it:**
We need to store our work when we see parentheses `( )`. When we finish the parentheses, we need to get our old work back.

---



###  `let bytes = s.as_bytes();`

**What it means:**
We're converting our string into a list of numbers (bytes). 

**Example:**
```
String: "1 + 2"
Becomes: [49, 32, 43, 32, 50]
Because:
- '1' = 49
- ' ' (space) = 32
- '+' = 43
- ' ' = 32
- '2' = 50
```

**Why we do this:**
Checking if something is a digit (`b'0'..=b'9'`) is faster with bytes than with letters.

---

### `let mut stack = Vec::with_capacity(16);`

**What it means:**
We're creating a stack (like a pile of plates) that can hold 16 items without needing to grow.

- `Vec` = a list that can grow
- `with_capacity(16)` = make room for 16 items right now

**What it stores:**
We'll store pairs of numbers: `(result, sign)`. Like `(5, 1)` means "the answer was 5 and we were adding".

---

### `let mut result = 0;`

**What it means:**
We're creating a box to hold our running answer. Start at 0.

---

### L `let mut sign = 1;`

**What it means:**
We're remembering if we're adding or subtracting the next number.

- `sign = 1` = we are ADDING (positive)
- `sign = -1` = we are SUBTRACTING (negative)

**Example:**
```
"1 + 2" → sign = +1, result = 0 + 1 = 1, then sign stays +1, result = 1 + 2 = 3
"1 - 2" → sign = +1, result = 0 + 1 = 1, then sign becomes -1, result = 1 + (-1)*2 = -1
```

---

###`let mut i = 0;`

**What it means:**
We're creating a counter that points to our current position in the string.

- `i` = index (position number)
- `= 0` = start at the first character (position 0)


**Example:**
```
String: "1 + 2"
i=0 → finger on '1'
i=1 → finger on ' '
i=2 → finger on '+'
i=3 → finger on ' '
i=4 → finger on '2'
```

---

### `while i < bytes.len() {`

**What it means:**
Keep looping (repeating) as long as we haven't reached the end of the string.

- `bytes.len()` = how many characters in the string

---

### `match bytes[i] {`

**What it means:**
Look at the current character and decide what to do based on what it is.
"It's like playing a game: 'What letter am I looking at?' If it's a number, do THIS. If it's a plus sign, do THAT. If it's a minus sign, do SOMETHING ELSE."

---

## The MOST IMPORTANT Part: Numbers! 

###  `b'0'..=b'9' => {`

**What it means:**
If the current character is a digit between '0' and '9' (like '1', '2', '3', etc.), do this block of code.

- `b'0'` = the byte for character '0' (which is 48)
- `b'9'` = the byte for character '9' (which is 57)
- `..=` = range from start to end (including both)

"If you see a number (like 1, 2, 3, 4, 5, 6, 7, 8, or 9), then it's time to read that number!"

---

###  `let mut num = 0;`

**What it means:**
Create a temporary box to build our number, starting at 0.

- `mut` = we can change it
- `num` = the number we're building

---

###`while i < bytes.len() && bytes[i].is_ascii_digit() {`

**What it means:**
Keep looping as long as:
1. We haven't reached the end of the string (`i < bytes.len()`)
2. The current character is a digit (`bytes[i].is_ascii_digit()`)

"Keep reading digits as long as there are more digits! Don't stop just because you read one number."

**Example with "123 + 5":**
```
Start with i at position of '1'
i=0: '1' is digit → read it
i=1: '2' is digit → read it
i=2: '3' is digit → read it
i=3: ' ' is NOT digit → STOP!
We read "123" as one number!
```

---

### `num = num * 10 + (bytes[i] - b'0') as i32;`

**What it means:**
It builds the number digit by digit.


**Part 1:** `num * 10` 
- Shifts existing digits left (makes room for new digit)

**Part 2:** `(bytes[i] - b'0')`
- Converts character '5' to number 5
- '5' in computer = 53
- '0' in computer = 48
- 53 - 48 = 5

**Part 3:** `as i32`
- Convert to 32-bit integer (a normal number)

**Part 4:** Add them together

**Kid version with example "123":**
```
Start: num = 0

See '1':
  0 × 10 = 0
  '1' - '0' = 1
  0 + 1 = 1
  num = 1

See '2':
  1 × 10 = 10
  '2' - '0' = 2
  10 + 2 = 12
  num = 12

See '3':
  12 × 10 = 120
  '3' - '0' = 3
  120 + 3 = 123
  num = 123
```

**Picture it like this:**
```
Start:     (empty)
Add '1':   1
Add '2':   12  (the 1 moves left, 2 comes in on right)
Add '3':   123 (the 12 moves left, 3 comes in on right)
```

---

###  `i += 1;`

**What it means:**
Move to the next character (increment the index by 1).
"Move your finger to the next letter. You just finished reading a digit, so now look at what comes next."

---

### `result += sign * num;`

**What it means:**
Add the number to our running total, using the current sign.

- If `sign = 1`: add the number (`result += num`)
- If `sign = -1`: subtract the number (`result += -num`)


**Example:**
```
result = 5, sign = 1, num = 3 → result = 5 + 3 = 8
result = 5, sign = -1, num = 3 → result = 5 - 3 = 2
```

---

### `continue;`

**What it means:**
Skip the rest of the loop and go back to the beginning (the `while` loop).

"We already moved our finger (`i` got increased inside the number loop). So let's skip the `i += 1` at the bottom and immediately look at the next character."

**Why we need this:**
Inside the number loop, we already increased `i` to point to the character AFTER the number. If we also did `i += 1` at the bottom, we would skip a character!

---


### `b'+' => sign = 1,`

**What it means:**
If we see a plus sign, set the sign to +1 (meaning we'll add the next number).

**Kid version:**
"The next number should be ADDED to our total."

---

### `b'-' => sign = -1,`

**What it means:**
If we see a minus sign, set the sign to -1 (meaning we'll subtract the next number).

---

### Parentheses Open `(`

```rust
b'(' => {
    stack.push((result, sign));
    result = 0;
    sign = 1;
}
```

** `stack.push((result, sign));`**
- Save our current result and sign onto the stack (memory box)

** `result = 0;`**
- Reset our running total to 0 for the stuff inside parentheses

** `sign = 1;`**
- Reset sign to +1 for the stuff inside parentheses

"When you see `(`, it's like saying 'PAUSE my current work and save it in a box. Start a fresh new calculation for what's inside the parentheses.'"

**Example:**
```
Before "(": result=5, sign=+1
Push (5, 1) onto stack
Reset result=0, sign=+1
Now calculate "(2 + 3)" → result becomes 5
```

---

### Parentheses Close `)`

```rust
b')' => {
    let (prev_result, prev_sign) = stack.pop().unwrap();
    result = prev_result + prev_sign * result;
}
```

** `let (prev_result, prev_sign) = stack.pop().unwrap();`**
- Take the top box from the stack (the most recent saved work)
- This gives us the result and sign from BEFORE the `(`

** `result = prev_result + prev_sign * result;`**
- Combine the old work with the new work
- Multiply the parentheses result by the sign before the `(`
- Add to the previous result

**Example:**
```
Stack had (5, 1) meaning result=5, sign=+1
Inside parentheses we got result=5
New result = 5 + (1 × 5) = 10
```

**Why multiply by prev_sign?**
If there was a minus before `(`, like `5 - (2 + 3)`:
- Inside parentheses = 5
- prev_sign = -1
- result = 5 + (-1 × 5) = 0 ✅ (because 5 - 5 = 0)

---

### `_ => {}`

**What it means:**
If the character is anything else (like a space), do nothing.

"If it's a space or something we don't care about, just ignore it and move on."

---

### `i += 1;`

**What it means:**
Move to the next character (increment index by 1).

**Why outside the match:**
This runs for everything EXCEPT numbers (because numbers have `continue` that skips this).

---

### `result`

**What it means:**
After the loop finishes, return the final answer.

---

## Complete Walkthrough Example

Let's trace `"3 - (2 + 1)"`:

```
Initial: result=0, sign=1, stack=[], i=0

i=0: char='3' (number)
  num=0
  num=0×10+(51-48)=3
  i=1 (now i=1)
  result=0+(1×3)=3
  continue (skip bottom i+=1)

i=1: char=' ' (space)
  do nothing
  i=2

i=2: char='-' (minus)
  sign = -1
  i=3

i=3: char=' ' (space)
  do nothing
  i=4

i=4: char='(' (open paren)
  stack.push((3, -1))
  result=0, sign=1
  i=5

i=5: char='2' (number)
  num=0→num=2, i=6
  result=0+(1×2)=2
  continue

i=6: char=' ' (space)
  i=7

i=7: char='+' (plus)
  sign=1
  i=8

i=8: char=' ' (space)
  i=9

i=9: char='1' (number)
  num=0→num=1, i=10
  result=2+(1×1)=3
  continue

i=10: char=')' (close paren)
  pop (prev_result=3, prev_sign=-1)
  result = 3 + (-1 × 3) = 0
  i=11

i=11: i == bytes.len(), exit loop

Return result = 0 
```

## Why This Code is SO GOOD

| Feature | Why It's Fast |
|---------|---------------|
| `bytes` instead of `chars` | Direct memory access, no Unicode handling |
| `match` statement | Compiler creates jump table (very fast) |
| `while` for numbers | Single pass, no recursion |
| `continue` after numbers | Avoids extra increment |
| `Vec::with_capacity` | No reallocation overhead |
| Stack stores tuples | One push/pop instead of two |
| Single pass | O(n) time, only reads string once |

---

## The Memory Picture 

```
String: "3 - (2 + 1)"
Becomes: [51, 32, 45, 32, 40, 50, 32, 43, 32, 49, 41]

Walking through:
i=0: '3' → parse number 3, result=3
i=1: ' ' → skip
i=2: '-' → sign=-1
i=3: ' ' → skip
i=4: '(' → PUSH (3,-1) to stack, reset
i=5: '2' → parse 2, result=2
i=6: ' ' → skip
i=7: '+' → sign=1
i=8: ' ' → skip
i=9: '1' → parse 1, result=3
i=10: ')' → POP, calculate 3 + (-1×3)=0

Stack visualization:
Start: []
After '(': [(3,-1)]
After ')': []
```

---

## The "Never Forget" Summary 

1. **result** = your piggy bank (running total)
2. **sign** = your flag (+1 for add, -1 for subtract)
3. **stack** = your memory box for `(`
4. **i** = your finger pointing at current character
5. **Numbers** = read all digits, build number with `×10 + digit`
6. **+/-** = just change the sign flag
7. **(** = save work in box, start fresh
8. **)** = take work from box, combine with parentheses result

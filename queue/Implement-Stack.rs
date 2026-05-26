// Implement a last-in-first-out (LIFO) stack using only two queues. The implemented stack should support all the functions of a normal stack (push, top, pop, and empty).

// Implement the MyStack class:

// void push(int x) Pushes element x to the top of the stack.
// int pop() Removes the element on the top of the stack and returns it.
// int top() Returns the element on the top of the stack.
// boolean empty() Returns true if the stack is empty, false otherwise.


use std::collections::VecDeque;

//  structure called MyStack
struct MyStack {

    // q is the queue storing i32 integers
    q: VecDeque<i32>,
}

impl MyStack {

    // create a new empty stack
    fn new() -> Self {

        // return a MyStack object
        MyStack {

            // create empty queue
            q: VecDeque::new(),
        }
    }
    
    // Push a value into the stack
    fn push(&mut self, x: i32) {

        // Store current queue length
        let len = self.q.len();

        // Add new value to back of queue
        self.q.push_back(x);

        // Rotate old elements behind the new one
        for _ in 0..len {

            // Remove front element
            let front = self.q.pop_front().unwrap();

            // Push removed element to back
            self.q.push_back(front);
        }
    }
    
    // Remove and return top stack element
    fn pop(&mut self) -> i32 {

        // Front of queue acts as stack top
        self.q.pop_front().unwrap()
    }
    
    // Return top element without removing it
    fn top(&self) -> i32 {

        // Get front value and dereference it
        *self.q.front().unwrap()
    }
    
    // Check if stack is empty
    fn empty(&self) -> bool {

        // Return true if queue has no elements
        self.q.is_empty()
    }
}

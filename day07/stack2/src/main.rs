struct Stack<T> {
    elements: Box<[Option<T>]>,
    top: isize,
    capacity: usize,
}

impl<T> Stack<T> {
    fn new(initial_capacity: usize) -> Self {
        Stack {
            elements: Self::allocate_array(initial_capacity),
            top: -1,
            capacity: initial_capacity,
        }
    }
    fn allocate_array(size: usize) -> Box<[Option<T>]> {
        let mut v = Vec::with_capacity(size);
        for _ in 0..size {
            v.push(None);
        }
        v.into_boxed_slice()
    }
    fn push(&mut self, item: T) {
        if (self.top as usize) >= self.capacity - 1 {
            self.resize();
        }
        self.top += 1;
        self.elements[self.top as usize] = Some(item);
    }
    fn resize(&mut self) {
        let new_capacity = self.capacity * 2;
        let new_elements = Self::allocate_array(new_capacity);

        for i in 0..self.capacity {
            self.elements[i] = None;
        }
        self.elements = new_elements;
        self.capacity = new_capacity;
    }

    fn pop(&mut self) -> Option<T> {
        if self.top >= 0 {
            let item = self.elements[self.top as usize].take(); // 取出元素并将位置设为 None
            self.top -= 1;
            item
        } else {
            None
        }
    }

    fn peek(&self) -> Option<&T> {
        if self.top >= 0 {
            self.elements[self.top as usize].as_ref()
        } else {
            None
        }
    }

    fn is_empty(&self) -> bool {
        self.top == -1
    }

    fn size(&self) -> usize {
        (self.top + 1) as usize
    }
}

fn main() {
    let mut stack = Stack::new(5);
    for i in 1..=5 {
        stack.push(i);
    }
    println!("Top element is: {:?}", stack.peek());
    println!("----------------------");

    println!("Stack size is:{}", stack.size());
    println!("----------------------");

    while !stack.is_empty() {
        println!("Popped element: {:?}", stack.pop());
        println!("Stack size is now:{}", stack.size());
        println!("Is stack empty now? {}", stack.is_empty());
        println!("----------------------");
    }
    println!("Is stack empty? {}", stack.is_empty());
}

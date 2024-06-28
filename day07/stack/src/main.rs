struct Stack<T> {
    elements: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Stack { elements: Vec::new() }
    }

    fn push(&mut self, item: T) {
        self.elements.push(item);
    }

    fn pop(&mut self) -> Option<T> {
        self.elements.pop()
    }

    fn peek(&self) -> Option <&T> {
        self.elements.last()
    }

    fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    fn size(&self) -> usize {
        self.elements.len()
    }
}

    fn main() {
        let mut stack = Stack::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);

        println!("Top element is: {:?}", stack.peek());
        println!("Stack size is:{}", stack.size());
        while !stack.is_empty() {
            println!("Popped element: {:?}", stack.pop());
            println!("Stack size is now:{}", stack.size());
            println!("Is stack empty now? {}", stack.is_empty());

        }
        println!("----------------------");
        println!("Is stack empty? {}", stack.is_empty());
    }

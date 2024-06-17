#[derive(Debug)]
struct Queue<T> {
    elements: Vec<T>,
}

impl<T> Queue<T> {
    // 新建空队列
    pub fn new() -> Self {
    Queue { elements: vec!()}
    }

    // 入队:尾部添加
    pub fn enqueue(&mut self, element: T) {
        self.elements.push(element);
    }

    // 出队：首部移出（Option自动实现Debug）
    pub fn dequeue(&mut self) -> Option<T> {
        if self.elements.is_empty() {
            None
        } else {
            Some(self.elements.remove(0))
        }
    }

    // 首队元素：返回首元素（不移出）
    pub fn peek(&self) -> Option<&T> {
        self.elements.first()
    }

    // 队列大小
    pub fn size(&self) -> usize {
        self.elements.len()
    }

    // 检查是否为空
    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}

fn main() {
    let mut queue = Queue::new();

    // 入队
    queue.enqueue(1);
    queue.enqueue(2);
    queue.enqueue(3);

    println!("队列：{:?}", queue);// 需要手动Debug自定义类型 #[derive(Debug)]

    // 查看队首
    if let Some(front) = queue.peek() {
        println!("队首元素是：{}", front);
    }

    // 出队
    queue.dequeue();
    println!("出队一次后，队首是：{:?}", queue.peek());

    // 队列大小
    println!("队列大小：{}", queue.size());

    // 再出队
    while !queue.is_empty() {
        println!("出队：{}", queue.dequeue().unwrap());
    }

    // 查空
    println!("队列是否为空：{}", queue.is_empty());
}

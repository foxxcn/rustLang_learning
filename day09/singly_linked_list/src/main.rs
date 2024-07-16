#[derive(Debug)]
struct ListNode {
    value: i32,
    next: Option<Box<ListNode>>,
}

#[derive(Debug)]
struct LinkedList {
    head: Option<Box<ListNode>>
}

impl LinkedList {
    fn new() -> Self {
        LinkedList { head: None }
    }

    fn push(&mut self, value: i32) {
        let new_node = Box::new(ListNode {
            value,
            next: self.head.take(),
        });
        self.head = Some(new_node);
    }

    fn pop(&mut self) -> Option<i32> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.value
        })
    }

    fn append(&mut self, value: i32) {
        let new_node = Box::new(ListNode { value, next: None });

        match self.head.as_mut() {
            Some(mut node) => {
                // 遍历到链表的最后一个节点
                while let Some(ref mut next) = node.next {
                    node = next;
                }
                node.next = Some(new_node);
            }
            None => {
                self.head = Some(new_node);
            }
        }
    }

    fn print_list(&self) {
        let mut node = &self.head;
        while let Some(ref current) = node {
            print!("{} -> ", current.value);
            node = &current.next;
        }
        println!("None");
    }

    //有歧义
    fn to_vec(&self) -> Vec<i32> {
        let mut vec = vec![];
        let mut node = &self.head;
        while let Some(ref current) = node {
            vec.push(current.value);
            node = &current.next;
        }
        vec
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push() {
        let mut list = LinkedList::new();
        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.to_vec(), vec![3, 2, 1]);
    }

    #[test]
    fn test_pop() {
        let mut list = LinkedList::new();
        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.to_vec(), vec![2, 1]);
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.to_vec(), vec![1]);
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.to_vec(), vec![]);
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn test_append() {
        let mut list = LinkedList::new();
        list.append(1);
        list.append(2);
        list.append(3);

        assert_eq!(list.to_vec(), vec![1, 2, 3]);
    }
}

fn main() {
    let mut list = LinkedList::new();
    list.push(1);
    list.push(2);
    list.push(3);

    println!("链表插入节点后：");
    list.print_list();

    //单向链表表头是最后插入的
    list.pop();
    println!("链表弹出头部节点后：");
    list.print_list();

    list.append(4);
    println!("链表在尾部插入节点后：");
    list.print_list();
}

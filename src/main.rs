use std::vec::Vec;

// contains a single field, Vector 'Vec<T>' that is used to store elements
#[derive(Debug)]
struct Queue<T> {
    data: Vec<T>,
}

// implement some methods to manipulate the queue
impl<T> Queue<T> {
    // creates a new, emtpy Queue
    fn new() -> Self {
        Queue { data: Vec::new() }
    }

    // enqueue method will add to the beginning of the queue
    fn enqueue(&mut self, elem: T) {
        self.data.push(elem);
    }

    // dequeue will remove and return an element from the start of the queue
    fn dequeue(&mut self) -> T {
        self.data.remove(0)
    }

    // this will return true if the queue is empty, and false if not
    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

fn main() {
    // create a new empty stack that will hold i32 elements
    let mut queue: Queue<i32> = Queue::new();

    // add some elements to the queue using the enqueue method
    queue.enqueue(1);
    queue.enqueue(2);
    queue.enqueue(3);

    // check if the queue is empty
    println!("Is the queue empty? {}", queue.is_empty());
    println!("Here is the queue: {:?}", queue);

    // remove elements using the dequeue method
    // removes '1' and returns '1'
    println!("Dequeued: {:?}", queue.dequeue());

    // continue removing elements
    println!("Dequeued: {:?}", queue.dequeue());
    println!("Dequeued: {:?}", queue.dequeue());

    // double check using is_empty
    println!("Is the queue empty now? {}", queue.is_empty());
}

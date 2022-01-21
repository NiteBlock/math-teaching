use std::{
    error::Error as StdError,
    fmt::{Display, Formatter, Result as FmtResult},
};

// The maximal amount of elements to be stored in a single queue
const MAX_SIZE: usize = 100;

#[derive(Clone, Debug)]
pub enum QueueError {
    // This error occurs when an element is attempted to be added but the queue is already full
    QueueOverflowError,
    // This error occurs when an element is attempted to be removed but the queue is empty
    QueueUnderflowError,
}

impl Display for QueueError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            QueueError::QueueUnderflowError => {
                f.write_str("The queue doesn't have any more items to enqueue")
            }
            QueueError::QueueOverflowError => f.write_str("The queue's maxsize has been reached."),
        }
    }
}

impl StdError for QueueError {}

pub struct Queue<T> {
    arr: [Option<T>; MAX_SIZE], // The array representation of the queue
    head: usize,                // The head of the queue (place where next element is added)
    tail: usize,                // The tail of the queue (place where next element is removed)
}

impl<T> Queue<T>
where
    T: Copy,
{
    fn new() -> Queue<T> {
        Queue {
            arr: [None; MAX_SIZE],
            head: 0,
            tail: 0,
        }
    }

    fn is_empty(&self) -> bool {
        // Checks weather there are any elements in the queue
        self.head == self.tail
    }
    fn is_full(&self) -> bool {
        (self.head + 1) % MAX_SIZE == self.tail
    }

    fn enqueue(&mut self, item: T) -> crate::Result<()> {
        // add an element to the queue
        if self.is_full() {
            return Err(Box::new(QueueError::QueueOverflowError));
        }
        self.head = (self.head + 1) % MAX_SIZE;
        self.arr[self.head] = Some(item);
        Ok(())
    }

    fn dequeue(&mut self) -> crate::Result<T> {
        if self.is_empty() {
            return Err(Box::new(QueueError::QueueUnderflowError));
        }
        self.tail = (self.tail + 1) % MAX_SIZE;
        let item: Option<T> = self.arr[self.tail];
        if item.is_none() {
            // This statment should ideally never be reached, since we are always adding the item
            return Err(Box::new(QueueError::QueueUnderflowError));
        }
        Ok(item.unwrap())
    }
}

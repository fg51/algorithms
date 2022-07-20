// LIFO: Last In First Out
// ring-buffer

pub trait Queue<T> {
    fn enqueue(&mut self, x: T) -> Option<T>;
    fn dequeue(&mut self) -> Option<T>;
}

#[cfg(test)]
mod tests {
    use super::*;

    const MAX: usize = 6;

    #[derive(Default)]
    pub struct Mock {
        que: [u32; MAX],
        head: usize,
        tail: usize,
    }

    impl Queue<u32> for Mock {
        fn enqueue(&mut self, x: u32) -> Option<u32> {
            let next = (self.tail + 1) % MAX;
            if next == self.head {
                return None;
            }
            self.que[self.tail] = x;
            self.tail = next;
            return Some(x);
        }

        fn dequeue(&mut self) -> Option<u32> {
            if self.head == self.tail {
                return None;
            }
            let x = self.que[self.head];
            self.head = (self.head + 1) % MAX;
            Some(x)
        }
    }

    #[test]
    fn it_works() {
        let max = 6;
        let mut mock = Mock::default();

        for i in 0..max - 1 {
            assert_eq!(mock.enqueue(i as u32), Some(i));
        }
        assert_eq!(mock.enqueue(max - 1), None);

        for i in 0..max - 1 {
            assert_eq!(mock.dequeue(), Some(i as u32));
        }
        assert_eq!(mock.dequeue(), None);
    }
}

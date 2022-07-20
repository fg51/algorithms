// LIFO: Last In First Out

pub trait Stack<T> {
    fn push(&mut self, x: T) -> Option<()>;
    fn pop(&mut self) -> Option<T>;
}

#[cfg(test)]
mod tests {
    use super::*;

    const MAX: usize = 5;

    #[derive(Default)]
    pub struct Mock {
        stack: [u32; MAX],
        sp: usize,
    }

    impl Stack<u32> for Mock {
        fn push(&mut self, x: u32) -> Option<()> {
            if self.sp < MAX {
                self.stack[self.sp] = x;
                self.sp += 1;
                return Some(());
            }
            None
        }

        fn pop(&mut self) -> Option<u32> {
            if self.sp > 0 {
                self.sp -= 1;
                return Some(self.stack[self.sp]);
            }
            None
        }
    }

    #[test]
    fn it_works() {
        let max = 6;
        let mut mock = Mock::default();

        for i in 0..max - 1 {
            assert_eq!(mock.push(i as u32), Some(()));
        }
        assert_eq!(mock.push(max), None);

        for i in 0..max - 1 {
            assert_eq!(mock.pop(), Some(max-2 - i as u32));
        }
        assert_eq!(mock.pop(), None);
    }
}

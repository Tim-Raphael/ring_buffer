#[derive(Default, Debug)]
pub struct RingBuffer<T, const CAPACITY: usize>
where
    [Option<T>; CAPACITY]: Default,
    T: Default,
{
    data: [Option<T>; CAPACITY],
    front: usize,
    back: usize,
}

impl<T, const CAPACITY: usize> RingBuffer<T, CAPACITY>
where
    [Option<T>; CAPACITY]: Default,
    T: Default + std::fmt::Debug,
{
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, item: T) -> bool {
        if self.is_full() {
            return false;
        }
        self.back = (self.back + 1) % CAPACITY;
        self.data[self.back] = Some(item);
        true
    }

    pub fn push_front(&mut self, item: T) -> bool {
        if self.is_full() {
            return false;
        }
        self.front = if self.front == 0 {
            CAPACITY - 1
        } else {
            self.front - 1
        };
        self.data[self.front] = Some(item);
        true
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        let item = self.data[self.back].take();
        self.back = if self.back == 0 {
            CAPACITY - 1
        } else {
            self.back - 1
        };
        item
    }

    pub fn pop_front(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        let item = self.data[self.front].take();
        self.front = (self.front + 1) % CAPACITY;
        item
    }

    pub fn get_front_ref(&self) -> &Option<T> {
        &self.data[self.front]
    }

    pub fn get_back_ref(&self) -> &Option<T> {
        &self.data[self.back]
    }

    pub fn is_empty(&self) -> bool {
        self.front == self.back
    }

    pub fn is_full(&self) -> bool {
        (self.back + 1) % CAPACITY == self.front
    }
}

impl<T, const CAPACITY: usize> RingBuffer<T, CAPACITY>
where
    [Option<T>; CAPACITY]: Default,
    T: Default + Copy,
{
    pub fn get_front(&self) -> Option<T> {
        self.data[self.front]
    }

    pub fn get_back(&self) -> Option<T> {
        self.data[self.back]
    }
}

// TODO
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init() {
        let buf = RingBuffer::<usize, 10>::new();

        assert_eq!(
            buf.get_front(),
            None,
            "There should be None in the front directly after init!"
        );
        assert_eq!(
            buf.get_back(),
            None,
            "There should be None in the back directly after init!"
        );

        assert!(
            buf.is_empty(),
            "RingBuffer should be empty directly after init!"
        );
        assert!(
            !buf.is_full(),
            "RingBuffer should not be full directly after init!"
        );

        let string_buf = RingBuffer::<String, 10>::new();

        assert_eq!(
            string_buf.get_front_ref(),
            &None,
            "There should be &None (when the T is not copy) in the front directly after init!"
        )
    }

    #[test]
    fn test_push() {
        const CAP: usize = 10;
        let mut buf = RingBuffer::<usize, CAP>::new();

        for i in 1..CAP {
            assert!(
                buf.push(i),
                "Expected to be able to fill the RingBuffer until the CAPACITY is reached!"
            );
        }

        assert!(!buf.push(CAP + 1), "Expected the RingBuffer to be full!");
    }

    #[test]
    fn test_push_front() {
        const CAP: usize = 10;
        let mut buf = RingBuffer::<usize, CAP>::new();

        for i in 1..CAP {
            assert!(
                buf.push_front(i),
                "Expected to be able to fill the RingBuffer until the CAPACITY is reached!"
            );
        }

        assert!(
            !buf.push_front(CAP + 1),
            "Expected the RingBuffer to be full!"
        );
    }

    #[test]
    fn test_pop() {
        let mut buf = RingBuffer::<String, 10>::new();

        assert_eq!(
            buf.pop(),
            None,
            "The last item should be empty directly after init!"
        );

        buf.push("Test".to_string());
        assert_eq!(
            buf.pop(),
            Some("Test".to_string()),
            "The last item should be Some(\"Test\") directly after pushing \"Test\"!"
        );
        assert!(
            buf.is_empty(),
            "The RingBuffer should be empty after removing the only item!"
        );
    }

    #[test]
    fn test_pop_front() {
        let mut buf = RingBuffer::<usize, 10>::new();

        assert_eq!(
            buf.pop_front(),
            None,
            "The last item should be empty directly after init!"
        );

        buf.push_front(1);
        assert_eq!(
            buf.pop_front(),
            Some(1),
            "The last item should be Some(1) directly after pushing 1!"
        );
        assert!(
            buf.is_empty(),
            "The RingBuffer should be empty after removing the only item!"
        );
    }
}

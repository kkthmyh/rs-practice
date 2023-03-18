//RefCell<T> 与 Box<T> 的区别
// Box<T>
// 编译阶段强制代码遵循借用规则，否则编译出错
// RefCell<T>
// 只会在运行时检查借用规则，出错则panic


// RefCell<T>的两个接口
// borrow
// 返回智能指针Ref<T>,它实现了Deref
// borrow_mut
// 返回智能指针RefMut<T>,它实现了Deref
fn main() {}

pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: 'a + Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T: Messenger> LimitTracker<'a, T> {
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;
        let percentage_of_max = self.value as f64 / self.max as f64;
        if percentage_of_max >= 1.0 {
            self.messenger.send("Error:You are over your quota!");
        } else if (percentage_of_max >= 0.9) {
            self.messenger
                .send("Urgent warning :You are over 90% of your quota!");
        } else if (percentage_of_max >= 0.75) {
            self.messenger
                .send("Warning:You are over 75% of your quota!");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages:RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, msg: &str) {
            self.sent_messages.borrow_mut().push(String::from(msg));
        }
    }

    #[test]
    fn send_msg() {
        let mock_msg = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_msg, 100);
        limit_tracker.set_value(80);
        assert_eq!(mock_msg.sent_messages.borrow().len(), 1);
    }
}

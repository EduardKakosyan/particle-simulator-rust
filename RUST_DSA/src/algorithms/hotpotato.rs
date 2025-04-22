#![allow(dead_code)]
use super::queue::Queue;
/// implementing the hot potato problem or Josephus problem
/// using queues in rust
pub fn hot_potato(names: Vec<&str>, num: usize) -> &str {
    let mut q = Queue::new(names.len());

    for name in names {
        let _nm = q.enqueue(name);
    }

    while q.len() > 1 {
        for _i in 0..num {
            let name = q.dequeue().unwrap();
            let _rm = q.enqueue(name);
        }

        let _rm = q.dequeue();
    }

    q.dequeue().unwrap()
}

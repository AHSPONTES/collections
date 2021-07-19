use std::collections::VecDeque;

fn main() {
    let mut queue = VecDeque::new();
    queue.push_back(String::from("first"));
    queue.push_back(String::from("second"));
    queue.push_back(String::from("third"));
    queue.push_back(String::from("fourth"));
    queue.push_front(String::from("zeroth"));

    while let Some(q_entry) = queue.pop_front() {
        println!("{}", q_entry);
    }
}

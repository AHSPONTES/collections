use std::vec::Vec;

fn main() {
    let mut sort_stack = Vec::new();
    sort_stack.push("anteater");
    sort_stack.push("zebra");
    sort_stack.push("tapir");
    sort_stack.push("elephant");
    sort_stack.push("coati");
    sort_stack.push("leopard");
    sort_stack.sort();

    while let Some(element) = sort_stack.pop() {
        println!("{}", element);
    }
}

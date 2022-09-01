# Print Queues
A string queue that can be add from different thread and print on main thread

# Usage

### Simple Usage
```rs
fn main() {
    print_queues::init();

    print_queues::add("GG");

    print_queues::add_string("Hello, World!".to_owned())

    print_queues::print();
    /*
        "GG"
        "Hello, World!"
    */
}
```

### Thread Usage
```rs
use std::thread;

fn main() {
    let th = thread::spawn(move || {
        // some server or application loop that want to print
        print_queues::add("Hello, Server!");
    });

    while !th.is_finished() {
        print_queues::print();
    }
}
```
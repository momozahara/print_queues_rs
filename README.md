# Print Queues
A print queue that can be add from different thread and print on main thread

# Usage

### Simple Usage
```rust
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
```rust
fn main() {
    let th = std::thread::spawn(move || {
        // some server or application loop that want to print
        print_queues::add("Hello, Server!");
    });

    while !th.is_finished() {
        print_queues::print();
        std::thread::sleep(
            std::thread::time::Duration::from_millis(1)
        );
    }
}
```
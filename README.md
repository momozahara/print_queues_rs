# Print Queues
A print queue that can be add from different thread and print on main thread

just use [tracing](https://docs.rs/tracing/latest/tracing/) fr fr this is project to learn how to publish on [crates.io](https://crates.io)

# Usage

### Simple Usage
```rust
struct Person<'a> {
    name: &'a str,
}

impl<'a> std::fmt::Display for Person<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Hello: {}", self.name)
    }
}

fn main() {
    let person = Person { name: "John doe" };

    print_queues::init();

    print_queues::add("John Doe");

    print_queues::add(person);

    print_queues::print();
    /*
        "John Doe"
        "Hello: John doe"
    */
}
```

```rust
print_queues::add("John");
print_queues::add("Doe");

let r = print_queues::next().unwrap();
// John

let r = print_queues::next().unwrap();
// Doe
```

### Thread Usage
```rust
fn main() {
    print_queues::init();

    let th = std::thread::spawn(move || {
        // some server or application loop that want to print
        print_queues::add("Hello, Server!");

        std::thread::sleep(
            std::time::Duration::from_secs(3)
        );

    });

    while !th.is_finished() {
        print_queues::print();
        std::thread::sleep(
            std::time::Duration::from_millis(1)
        );
    }
}
```

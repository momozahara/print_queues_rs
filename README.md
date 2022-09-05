# Print Queues
A print queue that can be add from different thread and print on main thread

# Usage

### Simple Usage
```rust
struct Person<'a> {
    name: &'a str
}

impl<'a> ToPrint for Person<'a> {
    fn to_string(&self) -> String {
        format!("Hello: {}!", self.name)
    }
}

fn main() {
    let john = Person { name: "John doe" };

    print_queues::init();

    print_queues::add("GG");

    print_queues::add_string("Hello, World!".to_owned());

    print_queues::add_struct(&john);

    print_queues::print();
    /*
        "GG"
        "Hello, World!"
        "Hello: John doe"
    */
}
```

```rust
print_queues::print_one();
/*
    "GG"
*/
print_queues::print_one();
/*
    "Hello, World!"
*/
print_queues::print_one();
/*
    "Hello: John doe!"
*/
```

### Thread Usage
```rust
fn main() {
    print_queues::init();

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
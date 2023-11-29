use queues::*;
use std::{fmt::Display, sync::Mutex};

static INSTANCE: Mutex<Option<Queue<String>>> = Mutex::new(None);

#[cfg(test)]
mod test {
    use std::fmt::Display;

    use crate::{add, init, next};

    #[test]
    fn test_str() {
        init();

        add("John Doe");

        let r = next().unwrap();
        assert_eq!(r, "John Doe");
    }

    #[test]
    fn test_struct() {
        init();
        struct MyType<'a> {
            name: &'a str,
        }
        impl<'a> Display for MyType<'a> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.name)
            }
        }

        add(MyType { name: "John" });
        add(MyType { name: "Doe" });

        let r = next().unwrap();
        assert_eq!(r, "John");

        let r = next().unwrap();
        assert_eq!(r, "Doe");
    }
}

pub fn init() {
    let mut i = INSTANCE.lock().unwrap();
    *i = Some(queue![]);
}

pub fn add<T: Display>(s: T) {
    let mut i = INSTANCE.lock().unwrap();
    let r = i.as_mut().unwrap();
    let _ = r.add(format!("{s}"));
}

pub fn print() {
    let mut i = INSTANCE.lock().unwrap();
    let r = i.as_mut().unwrap();
    loop {
        let q = r.peek();
        match q {
            Ok(s) => {
                println!("{s}");
                let _ = r.remove();
            }
            Err(_) => break,
        }
    }
}

pub fn next() -> Option<String> {
    let mut i = INSTANCE.lock().unwrap();
    let r = i.as_mut().unwrap();
    let q = r.peek();
    match q {
        Ok(s) => {
            let _ = r.remove();
            Some(s)
        }
        Err(_) => None,
    }
}

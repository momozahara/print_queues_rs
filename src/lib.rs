use queues::*;
use std::sync::Mutex;

static INSTANCE: Mutex<Option<Queue<String>>> = Mutex::new(None);

pub fn init() {
    let mut i = INSTANCE.lock().unwrap();
    *i = Some(queue![]);
}

pub fn add(s: &'static str) {
    let mut i = INSTANCE.lock().unwrap();
    let r = i.as_mut().unwrap();
    let _ = r.add(s.to_owned());
}

pub fn add_string(s: String) {
    let mut i = INSTANCE.lock().unwrap();
    let r = i.as_mut().unwrap();
    let _ = r.add(s);
}

pub fn print() {
    let mut i = INSTANCE.lock().unwrap();
    let r = i.as_mut().unwrap();
    loop {
        let q = r.peek();
        match q {
            Ok(s) => {
                println!("{}", s);
            },
            Err(_) => break,
        }
    }
}
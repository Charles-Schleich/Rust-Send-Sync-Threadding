use std::sync::Arc;
use std::thread;

trait Foo: Send + Sync{
    fn inc(&mut self, c:i32) -> i32;
    fn get_inner(&self) -> i32;
}

#[derive(Debug)]
struct FooImpl {
    c: i32,
}

impl Foo for FooImpl {
    fn inc(&mut self, c: i32) -> i32 {
        self.c += c;
        self.c
    }
    fn get_inner(&self) -> i32{
        self.c
    }
}


pub fn main() {
    println!("Start Simple Example! \n -----------------------");

    let counter: Arc<Box<Foo>> = Arc::new(Box::new(FooImpl { c: 23 }));
    let mut threads = Vec::new();

    for thrd in 0..10 {
        
        let mut ctr = counter.clone();

        threads.push(thread::spawn(move || {
            let x = ctr.get_inner();
            println!("Thread:{} , counter:{:?}", thrd, x);
        }));
    }

    for thread in threads {
        thread.join().unwrap();
    }
}
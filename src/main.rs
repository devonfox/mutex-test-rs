use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use std::fmt;

fn main() {
    let mutex = Arc::new(Mutex::new(Vec::new()));
    let c_mutex = Arc::clone(&mutex);

    let push_thread = thread::spawn(move || {
        let data = Data {
            num: 10,
            flag: true,
        };
        c_mutex.lock().unwrap().push(data);
        another(c_mutex);
        println!("Inside moved thread")
    });

    println!("After moved thread.");
    let data = Data {
            num: 0,
            flag: true,
        };
    mutex.lock().unwrap().push(data);
    // push_thread.join()
    // .expect("thread::spawn failed");
   let data = Data {
            num: 20,
            flag: true,
        };

    data.display();
    // mutex.lock().unwrap().push(data);
    let d_mutex = Arc::clone(&mutex);
    let push2_thread = thread::spawn(move || {
        let data = Data {
            num: 30,
            flag: false,
        };
        d_mutex.lock().unwrap().push(data);
        let data = Data {
            num: 40,
            flag: true,
        };
        
        d_mutex.lock().unwrap().push(data);
        println!("Inside second thread.");
    });

    push2_thread.join().expect("thread::spawn failed again");
    // println!("{:?}", mutex.lock().unwrap());
    let newone = Arc::clone(&mutex);
    println!("{:?}", mutex.lock().unwrap());
    another(newone);
     
    // println!("{:?}", mutex.lock().unwrap())

}

fn another(mutex: Arc<Mutex<Vec<Data>>>) {
    thread::sleep(Duration::from_millis(5));
    let data = Data {
            num: 1000,
            flag: true,
        };
    mutex.lock().unwrap().push(data);
    println!("Inside function");
}

#[derive(Debug)]
pub struct Data {
    pub num: u64,
    pub flag: bool,
}

impl fmt::Display for Data {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "{}", self.num)
    }
}

impl Data {
    pub fn display(self) {
        print!("{}", self.num)
    }
}
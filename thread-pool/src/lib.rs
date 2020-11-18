use std::sync::{mpsc, Arc, Mutex};

pub struct ThreadPool {
    _handles: Vec<std::thread::JoinHandle<()>>,
    tx: mpsc::Sender<Job>,
}

type Job = Box<dyn FnOnce() + Send>;

impl ThreadPool {
    pub fn new(worker_count: usize) -> Self {
        let (tx, rx) = mpsc::channel::<Job>();
        let rx = Arc::new(Mutex::new(rx));
        let mut _handles = Vec::with_capacity(worker_count);
        for _ in 0..worker_count {
            let rx = Arc::clone(&rx);
            let handle = std::thread::spawn(move || loop {
                let result = rx.lock().expect("Mutex err").recv();
                match result {
                    Ok(work) => work(),
                    Err(err) => {
                        println!("Recv err: {}", err);
                        break;
                    }
                }
            });
            _handles.push(handle);
        }
        Self { _handles, tx }
    }

    pub fn execute<F>(&self, work: F)
    where
        F: FnOnce() + Send + 'static,
    {
        self.tx.send(Box::new(work)).expect("thread closed")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let pool = ThreadPool::new(2);
        pool.execute(|| println!("Do my work, hurry up."));
        pool.execute(|| println!("Do my work, hurry up."));
    }
}

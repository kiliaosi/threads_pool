use std::thread;
use std::sync::{ Mutex, mpsc, Arc };
pub struct Worker {
  id: usize,
  thread: thread::JoinHandle<()>,
}

impl Worker {
  pub fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>)->Worker{
    let thread = thread::spawn(move || {
      loop {
        let job = receiver.lock().unwrap().recv().unwrap();
        job();
      }
    });
    Worker {
      id,
      thread,
    }
  }
}

pub struct ThreadPool {
  workers: Vec<Worker>,
  sender: mpsc::Sender<Job>,
}

type Job = Box<dyn FnOnce()+ Send + 'static>;

impl ThreadPool {
  pub fn new(size:usize)->ThreadPool{
    assert!(size>0);
    let mut workers = Vec::with_capacity(size);
    let (sender, receiver) = mpsc::channel();
    let receiver = Arc::new(Mutex::new(receiver));

    for id in 0..size {
        let worker = Worker::new(id, Arc::clone(&receiver));
        workers.push(worker);
    }

    ThreadPool{
      workers,
      sender,
    }
  }

  pub fn execute<F>(&self, f:F)
  where F:FnOnce() + Send + 'static
  {
    self.sender.send(Box::new(f)).unwrap();
  }
}
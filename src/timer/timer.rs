use std::sync::{
    atomic::{AtomicBool, Ordering},
    mpsc::{channel, SendError, Sender},
    Arc,
};
use std::thread::{sleep, spawn, JoinHandle};
use std::time::{Duration, Instant};

pub struct Timer(JoinHandle<()>, Sender<()>, Arc<AtomicBool>);

impl Timer {
    pub fn new<F>(time: Duration, f: F) -> Timer
    where
        F: Fn() -> (),
        F: Send + 'static,
    {
        let (tx, rx) = channel();
        let run = Arc::new(AtomicBool::new(false));
        let running = run.clone();

        let handle = spawn(move || {
            running.store(true, Ordering::SeqCst);
            let mut last_start = Instant::now();
            loop {
                f();

                match rx.try_recv() {
                    Ok(_) => {
                        running.store(false, Ordering::SeqCst);
                        return ();
                    }
                    Err(_) => {}
                };

                sleep({
                    let dur = Instant::now() - last_start;
                    if dur >= time {
                        Duration::new(0, 0)
                    } else {
                        time - dur
                    }
                });

                last_start = Instant::now();
            }
        });

        Timer(handle, tx, run)
    }

    pub fn join(self) -> std::thread::Result<()> {
        self.0.join()
    }

    pub fn thread(&self) -> &std::thread::Thread {
        self.0.thread()
    }

    pub fn stop(&self) -> Result<(), SendError<()>> {
        self.1.send(())
    }

    pub fn is_running(&self) -> bool {
        self.2.load(Ordering::SeqCst)
    }
}

unsafe impl Send for Timer {}
unsafe impl Sync for Timer {}

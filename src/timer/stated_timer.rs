use super::timer::Timer;
use std::sync::{mpsc::SendError, Arc, Mutex};
use std::time::Duration;

pub struct StatedTimer<T: Clone + Send + 'static> {
    state: Arc<Mutex<T>>,
    timer: Timer,
}

impl<T: Clone + Send + 'static> StatedTimer<T> {
    pub fn new<F>(initial: T, duration: Duration, f: F) -> Self
    where
        F: Fn(T) -> T,
        F: Send + 'static,
    {
        let res_state = Arc::new(Mutex::new(initial));
        let state = res_state.clone();

        let timer = Timer::new(duration, move || {
            let s = f({ state.lock().unwrap().clone() });

            {
                let mut bs = state.lock().unwrap();
                *bs = s;
            }
        });

        StatedTimer {
            state: res_state,
            timer,
        }
    }

    pub fn get(&self) -> T {
        self.state.lock().unwrap().clone()
    }

    pub fn join(self) -> std::thread::Result<()> {
        self.timer.join()
    }

    pub fn thread(&self) -> &std::thread::Thread {
        self.timer.thread()
    }

    pub fn stop(&self) -> Result<(), SendError<()>> {
        self.timer.stop()
    }

    pub fn is_running(&self) -> bool {
        self.timer.is_running()
    }
}

unsafe impl<T: Clone + Send + 'static> Send for StatedTimer<T> {}
unsafe impl<T: Clone + Send + 'static> Sync for StatedTimer<T> {}

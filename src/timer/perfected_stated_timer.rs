use super::StatedTimer;
use crate::perfecter::PerfecterProvider;

use std::sync::{mpsc::SendError, Arc};
use std::time::Duration;

pub struct PerfectedStatedTimer<T: Clone + Send + 'static> {
    timer: StatedTimer<T>,
    // provider: Arc<RwLock<PerfecterProvider>>
}

impl<T: Clone + Send + 'static> PerfectedStatedTimer<T> {
    pub fn new<F>(initial: T, duration: Duration, f: F) -> Self
    where
        F: Fn(T, Arc<PerfecterProvider>) -> T,
        F: Send + 'static,
    {
        let provider = Arc::new(PerfecterProvider::new(duration));
        // let p = provider.clone();

        let timer = StatedTimer::new(initial, duration, move |state| f(state, provider.clone()));

        PerfectedStatedTimer {
            timer,
            // provider: p,
        }
    }

    pub fn get(&self) -> T {
        self.timer.get()
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

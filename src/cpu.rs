use crate::platform::imp::cpu;
use crate::types::cpu::{CoreLoadInfo, CoresLoadInfo};
use crate::Timer;

#[derive(Clone)]
struct State {
    prev: CoresLoadInfo,
    current: Option<CoresLoadInfo>,
}

pub struct Cpu {
    timer: Timer<Option<State>>,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            timer: Timer::new(
                None,
                std::time::Duration::from_secs(1),
                move |state, provider| match state {
                    None => {
                        provider.get();
                        let s = cpu::load().ok()?;
                        Some(State {
                            prev: s,
                            current: None,
                        })
                    }
                    Some(state) => {
                        let next_prev = cpu::load().ok()?;
                        let perfecter = match provider.get() {
                            None => {
                                return Some(State {
                                    prev: next_prev,
                                    current: None,
                                })
                            }
                            Some(p) => p,
                        };

                        if state.prev.len() != next_prev.len() {
                            Some(State {
                                prev: next_prev,
                                current: None,
                            })
                        } else {
                            let prev = state.prev;
                            let mut current = Vec::with_capacity(prev.len());

                            for i in 0..prev.len() {
                                current.push(CoreLoadInfo {
                                    system: perfecter
                                        .perfect(&(next_prev[i].system - prev[i].system)),
                                    user: perfecter.perfect(&(next_prev[i].user - prev[i].user)),
                                    idle: perfecter.perfect(&(next_prev[i].idle - prev[i].idle)),
                                });
                            }

                            Some(State {
                                prev: next_prev,
                                current: Some(current),
                            })
                        }
                    }
                },
            ),
        }
    }

    pub fn load(&self) -> Option<CoresLoadInfo> {
        self.timer.get()?.current
    }

    pub fn close(self) {
        (&self).timer.stop().unwrap();
        self.timer.join().unwrap();
    }
}

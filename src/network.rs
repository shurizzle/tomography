use crate::platform::imp::network;
use crate::types::network::NetworkInterface;
use crate::Timer;

use std::collections::HashMap;

#[derive(Clone)]
struct State {
    prev: HashMap<String, NetworkInterface>,
    current: Option<Vec<NetworkInterface>>,
}

pub struct Network {
    timer: Timer<Option<State>>,
}

fn all() -> Option<HashMap<String, NetworkInterface>> {
    let ifs = network::all().ok()?;
    let mut hm = HashMap::with_capacity(ifs.len());

    for netif in ifs.into_iter() {
        hm.insert(netif.name.clone(), netif);
    }

    Some(hm)
}

impl Network {
    pub fn new() -> Network {
        Network {
            timer: Timer::new(
                None,
                std::time::Duration::from_secs(1),
                move |state, provider| match state {
                    None => {
                        provider.get();
                        let prev = all()?;
                        Some(State {
                            prev,
                            current: None,
                        })
                    }
                    Some(state) => {
                        let next_prev = all()?;
                        let perfecter = match provider.get() {
                            None => {
                                return Some(State {
                                    prev: next_prev,
                                    current: None,
                                })
                            }
                            Some(p) => p,
                        };

                        let prev = state.prev;
                        let mut current = Vec::new();
                        for (k, v) in next_prev.iter() {
                            if prev.contains_key(k) {
                                let mut netif = v.clone();
                                netif.up = perfecter.perfect(&(netif.up - prev[k].up));
                                netif.down = perfecter.perfect(&(netif.down - prev[k].down));
                                current.push(netif);
                            }
                        }

                        Some(State {
                            prev: next_prev,
                            current: Some(current),
                        })
                    }
                },
            ),
        }
    }

    pub fn interfaces(&self) -> Option<Vec<NetworkInterface>> {
        self.timer.get()?.current
    }

    pub fn close(self) {
        (&self).timer.stop().unwrap();
        self.timer.join().unwrap();
    }
}

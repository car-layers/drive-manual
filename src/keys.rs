use async_std::stream::{self, Stream, StreamExt};
use device_query::{DeviceQuery, DeviceState, Keycode};
use std::time::Duration;

#[derive(Debug, Clone)]
pub enum KeyEvent {
    Down(Keycode),
    Up(Keycode),
}

impl KeyEvent {
    const POLL_MS: u8 = 50;

    pub fn listen() -> impl Stream<Item = Self> {
        let mut prev_keys = vec![];
        let kb = DeviceState::new();
        stream::interval(Duration::from_millis(Self::POLL_MS.into()))
            .filter_map(move |_| {
                let keys = kb.get_keys();
                if prev_keys != keys {
                    let up_n_down = prev_keys
                        .iter()
                        .filter(|k| !keys.contains(k))
                        .map(|k| KeyEvent::Up(k.clone()))
                        .chain(
                            keys.iter()
                                .filter(|k| !prev_keys.contains(k))
                                .map(|k| KeyEvent::Down(k.clone())),
                        )
                        .collect::<Vec<_>>();
                    prev_keys = keys;
                    Some(stream::from_iter(up_n_down))
                } else {
                    None
                }
            })
            .flatten()
    }

    pub fn listen_keys(keys: &[Keycode]) -> impl Stream<Item = Self> {
        let keys = keys.to_owned();
        Self::listen().filter(move |k| match k {
            Self::Up(key) | Self::Down(key) => keys.contains(key),
        })
    }
}

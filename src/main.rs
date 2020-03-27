use async_std::prelude::*;
use device_query::Keycode as K;

mod keys;
use keys::KeyEvent;

#[async_std::main]
async fn main() {
    KeyEvent::listen_keys(&[K::Left, K::Right])
        .scan(0.0, |s, k| {
            let next = match k {
                KeyEvent::Down(K::Left) => -1.0,
                KeyEvent::Down(K::Right) => 1.0,
                KeyEvent::Up(_) => 0.0,
                _ => unreachable!(),
            };
            *s = next;
            Some(next)
        })
        .for_each(|k| println!("{}", k))
        .await;
}

use starbloom_base::*;

mod core;
mod player;

egor::main!(main);
#[allow(dead_code)]
fn main() {
    crate::core::main(Option::<&'static [DummyPlugin]>::None);
}

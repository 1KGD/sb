use starbloom_base::*;

mod core;
mod player;

fn main() {
    crate::core::main(Option::<&'static [DummyPlugin]>::None);
}

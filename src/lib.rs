#![feature(never_type)]
#![feature(try_blocks)]

pub mod prelude;
pub mod thing_delegator;

pub mod data {
    mod user;
    pub use user::*;
}

pub mod util {
    pub mod discord;
    pub mod language;
}

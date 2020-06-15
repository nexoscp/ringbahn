#![no_std]
pub mod fs;
pub mod drive;
pub mod event;
pub mod net;
pub mod io;

mod buf;
mod completion;
mod ring;
mod submission;

pub use ring::Ring;
pub use submission::Submission;

#[doc(inline)]
pub use drive::Drive;
#[doc(inline)]
pub use event::Event;
#[doc(inline)]
pub use fs::File;

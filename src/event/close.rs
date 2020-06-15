use core::mem::ManuallyDrop;

use super::{Event, Cancellation};
//use std::os::unix::io::RawFd;
use crate::linux::io::RawFd;

pub struct Close {
    fd: RawFd,
}

impl Close {
    pub fn new(fd: RawFd) -> Close {
        Close { fd }
    }
}

impl Event for Close {
    unsafe fn prepare(&mut self, sqe: &mut iou::SubmissionQueueEvent<'_>) {
        uring_sys::io_uring_prep_close(sqe.raw_mut(), self.fd)
    }

    fn cancellation(_: &mut ManuallyDrop<Self>) -> Cancellation {
        Cancellation::null()
    }
}

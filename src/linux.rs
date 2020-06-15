
//replaces std::os::unix::io::RawFd
pub mod io {
    use crate::linux::raw;

    // see libstd/sys/unix/ext/io.rs
    pub type RawFd = raw::c_int;

    // see libstd/sys/unix/ext/io.rs
    pub trait AsRawFd {
        fn as_raw_fd(&self) -> RawFd;
    }

    // see libstd/sys/unix/ext/io.rs
    /* pub trait FromRawFd {
        unsafe fn from_raw_fd(fd: RawFd) -> Self;
    } */
}

//replace libstd/os/raw/mod.rs
mod raw {
    #[warn(non_camel_case_types)]
    pub type c_int = i32;
}
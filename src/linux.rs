
//replaces std::os::unix::io::RawFd
pub mod io {
    use crate::linux::raw;

    // see libstd/sys/unix/ext/io.rs
    pub type RawFd = raw::c_int;
}

//replace libstd/os/raw/mod.rs
mod raw {
    pub type c_int = i32;
}
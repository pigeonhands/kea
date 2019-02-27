

pub fn is_root() -> bool {
    unsafe{
        libc::getuid() == 0
    }
}
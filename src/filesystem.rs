use libc::ENOSYS;

pub trait FileSystem {
    fn init(&mut self) -> Result<(), i32> {
        Err(ENOSYS)
    }
    fn destroy(&mut self) -> Result<(), i32> {
        Err(ENOSYS)
    }
    fn lookup(&mut self) -> Result<(), i32> {
        Err(ENOSYS)
    }
    fn forget(&mut self) -> Result<(), i32> {
        Err(ENOSYS)
    }
    fn getattr(&mut self) -> Result<(), i32> {
        Err(ENOSYS)
    }
    fn setattr(&mut self) -> Result<(), i32> {
        Err(ENOSYS)
    }
    fn readlink(&mut self) -> Result<(), i32> {
        Err(ENOSYS)
    }
    fn mknod(&mut self) -> Result<(), i32> {
        Err(ENOSYS)
    }
    fn mkdir(&mut self) -> Result<(), i32> {
        Err(ENOSYS)
    }
    fn unlink(&mut self) -> Result<(), i32> {
        Err(ENOSYS)
    }
    fn rmdir(&mut self) -> Result<(), i32> {
        Err(ENOSYS)
    }
    fn symlink(&mut self) -> Result<(), i32> {
        Err(ENOSYS)
    }
    fn rename(&mut self) -> Result<(), i32> {
        Err(ENOSYS)
    }
    fn link(&mut self) -> Result<(), i32> {
        Err(ENOSYS)
    }
    fn open(&mut self) -> Result<(), i32> {
        Err(ENOSYS)
    }
    fn read(&mut self) -> Result<(), i32> {
        Err(ENOSYS)
    }
    fn write(&mut self) -> Result<(), i32> {
        Err(ENOSYS)
    }
    fn flush(&mut self) -> Result<(), i32> {
        Err(ENOSYS)
    }
    fn release(&mut self) -> Result<(), i32> {
        Err(ENOSYS)
    }
    fn fsync(&mut self) -> Result<(), i32> {
        Err(ENOSYS)
    }
    fn opendir(&mut self) -> Result<(), i32> {
        Err(ENOSYS)
    }
    fn readdir(&mut self) -> Result<(), i32> {
        Err(ENOSYS)
    }
    fn releasedir(&mut self) -> Result<(), i32> {
        Err(ENOSYS)
    }
    fn fsyncdir(&mut self) -> Result<(), i32> {
        Err(ENOSYS)
    }
    fn statfs(&mut self) -> Result<(), i32> {
        Err(ENOSYS)
    }
    fn setxattr(&mut self) -> Result<(), i32> {
        Err(ENOSYS)
    }
    fn getxattr(&mut self) -> Result<(), i32> {
        Err(ENOSYS)
    }
    fn listxattr(&mut self) -> Result<(), i32> {
        Err(ENOSYS)
    }
    fn removexattr(&mut self) -> Result<(), i32> {
        Err(ENOSYS)
    }
    fn access(&mut self) -> Result<(), i32> {
        Err(ENOSYS)
    }
    fn create(&mut self) -> Result<(), i32> {
        Err(ENOSYS)
    }
    fn getlk(&mut self) -> Result<(), i32> {
        Err(ENOSYS)
    }
    fn setlk(&mut self) -> Result<(), i32> {
        Err(ENOSYS)
    }
    fn bmap(&mut self) -> Result<(), i32> {
        Err(ENOSYS)
    }
    fn poll(&mut self) -> Result<(), i32> {
        Err(ENOSYS)
    }
    fn write_buf(&mut self) -> Result<(), i32> {
        Err(ENOSYS)
    }
    fn retrieve_reply(&mut self) -> Result<(), i32> {
        Err(ENOSYS)
    }
    fn forget_multi(&mut self) -> Result<(), i32> {
        Err(ENOSYS)
    }
    fn flock(&mut self) -> Result<(), i32> {
        Err(ENOSYS)
    }
    fn fallocate(&mut self) -> Result<(), i32> {
        Err(ENOSYS)
    }
    fn readdirplus(&mut self) -> Result<(), i32> {
        Err(ENOSYS)
    }
    fn copy_file_range(&mut self) -> Result<(), i32> {
        Err(ENOSYS)
    }
    fn lseek(&mut self) -> Result<(), i32> {
        Err(ENOSYS)
    }
}

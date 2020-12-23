use std::env;

use rusfuse;

struct ExampleFs;

const ENOSYS: i32 = 38;

impl rusfuse::FileSystem for ExampleFs {
    fn init(&mut self) -> Result<(), i32> {
        println!("call init");
        Err(ENOSYS)
    }
    fn destroy(&mut self) -> Result<(), i32> {
        println!("call destroy");
        Err(ENOSYS)
    }
    fn lookup(&mut self) -> Result<(), i32> {
        println!("call lookup");
        Err(ENOSYS)
    }
    fn forget(&mut self) -> Result<(), i32> {
        println!("call forget");
        Err(ENOSYS)
    }
    fn getattr(&mut self) -> Result<(), i32> {
        println!("call getattr");
        Err(ENOSYS)
    }
    fn setattr(&mut self) -> Result<(), i32> {
        println!("call setattr");
        Err(ENOSYS)
    }
    fn readlink(&mut self) -> Result<(), i32> {
        println!("call readlink");
        Err(ENOSYS)
    }
    fn mknod(&mut self) -> Result<(), i32> {
        println!("call mknod");
        Err(ENOSYS)
    }
    fn mkdir(&mut self) -> Result<(), i32> {
        println!("call mkdir");
        Err(ENOSYS)
    }
    fn unlink(&mut self) -> Result<(), i32> {
        println!("call unlink");
        Err(ENOSYS)
    }
    fn rmdir(&mut self) -> Result<(), i32> {
        println!("call rmdir");
        Err(ENOSYS)
    }
    fn symlink(&mut self) -> Result<(), i32> {
        println!("call symlink");
        Err(ENOSYS)
    }
    fn rename(&mut self) -> Result<(), i32> {
        println!("call rename");
        Err(ENOSYS)
    }
    fn link(&mut self) -> Result<(), i32> {
        println!("call link");
        Err(ENOSYS)
    }
    fn open(&mut self) -> Result<(), i32> {
        println!("call open");
        Err(ENOSYS)
    }
    fn read(&mut self) -> Result<(), i32> {
        println!("call read");
        Err(ENOSYS)
    }
    fn write(&mut self) -> Result<(), i32> {
        println!("call write");
        Err(ENOSYS)
    }
    fn flush(&mut self) -> Result<(), i32> {
        println!("call flush");
        Err(ENOSYS)
    }
    fn release(&mut self) -> Result<(), i32> {
        println!("call release");
        Err(ENOSYS)
    }
    fn fsync(&mut self) -> Result<(), i32> {
        println!("call fsync");
        Err(ENOSYS)
    }
    fn opendir(&mut self) -> Result<(), i32> {
        println!("call opendir");
        Err(ENOSYS)
    }
    fn readdir(&mut self) -> Result<(), i32> {
        println!("call readdir");
        Err(ENOSYS)
    }
    fn releasedir(&mut self) -> Result<(), i32> {
        println!("call releasedir");
        Err(ENOSYS)
    }
    fn fsyncdir(&mut self) -> Result<(), i32> {
        println!("call fsyncdir");
        Err(ENOSYS)
    }
    fn statfs(&mut self) -> Result<(), i32> {
        println!("call statfs");
        Err(ENOSYS)
    }
    fn setxattr(&mut self) -> Result<(), i32> {
        println!("call setxattr");
        Err(ENOSYS)
    }
    fn getxattr(&mut self) -> Result<(), i32> {
        println!("call getxattr");
        Err(ENOSYS)
    }
    fn listxattr(&mut self) -> Result<(), i32> {
        println!("call listxattr");
        Err(ENOSYS)
    }
    fn removexattr(&mut self) -> Result<(), i32> {
        println!("call removexattr");
        Err(ENOSYS)
    }
    fn access(&mut self) -> Result<(), i32> {
        println!("call access");
        Err(ENOSYS)
    }
    fn create(&mut self) -> Result<(), i32> {
        println!("call create");
        Err(ENOSYS)
    }
    fn getlk(&mut self) -> Result<(), i32> {
        println!("call init");
        Err(ENOSYS)
    }
    fn setlk(&mut self) -> Result<(), i32> {
        println!("call setlk");
        Err(ENOSYS)
    }
    fn bmap(&mut self) -> Result<(), i32> {
        println!("call bmap");
        Err(ENOSYS)
    }
    fn poll(&mut self) -> Result<(), i32> {
        println!("call poll");
        Err(ENOSYS)
    }
    fn write_buf(&mut self) -> Result<(), i32> {
        println!("call write_buf");
        Err(ENOSYS)
    }
    fn retrieve_reply(&mut self) -> Result<(), i32> {
        println!("call retrieve_reply");
        Err(ENOSYS)
    }
    fn forget_multi(&mut self) -> Result<(), i32> {
        println!("call forget_multi");
        Err(ENOSYS)
    }
    fn flock(&mut self) -> Result<(), i32> {
        println!("call flock");
        Err(ENOSYS)
    }
    fn fallocate(&mut self) -> Result<(), i32> {
        println!("call fallocate");
        Err(ENOSYS)
    }
    fn readdirplus(&mut self) -> Result<(), i32> {
        println!("call readdirplus");
        Err(ENOSYS)
    }
    fn copy_file_range(&mut self) -> Result<(), i32> {
        println!("call copy_file_range");
        Err(ENOSYS)
    }
    fn lseek(&mut self) -> Result<(), i32> {
        println!("call lseek");
        Err(ENOSYS)
    }}

fn main() {
    let mountpoint: String = env::args().nth(1).unwrap();
    let mut file_system = ExampleFs {};
    rusfuse::fuse_loop(&mountpoint, &mut file_system);
}

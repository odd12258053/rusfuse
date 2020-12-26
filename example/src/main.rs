use std::env;

use rusfuse::*;

struct ExampleFs;

const ENOENT: i32 = 2;
const ENOSYS: i32 = 38;

const FILE_NAME: &str = "hello";
const TEXT: &str = "Hello World!\n";

const TEST_DIR_ATTR: FuseAttr = FuseAttr {
    dev: 0,
    ino: 1,
    nlink: 2,
    size: 0,
    blocks: 0,
    atime: 0,
    atimensec: 0,
    mtime: 0,
    mtimensec: 0,
    ctime: 0,
    ctimensec: 0,
    mode: 0o040755,
    uid: 1000,
    gid: 1000,
    rdev: 0,
    blksize: 4032,
};

const TEST_FILE_ATTR: FuseAttr = FuseAttr {
    dev: 0,
    ino: 2,
    nlink: 1,
    size: 13,
    blocks: 8,
    atime: 0,
    atimensec: 0,
    mtime: 0,
    mtimensec: 0,
    ctime: 0,
    ctimensec: 0,
    mode: 0o100644,
    uid: 1000,
    gid: 1000,
    rdev: 0,
    blksize: 4032,
};

impl rusfuse::FileSystem for ExampleFs {
    fn init(&mut self) -> Result<(), i32> {
        println!("call init");
        Err(ENOSYS)
    }
    fn destroy(&mut self) -> Result<(), i32> {
        println!("call destroy");
        Err(ENOSYS)
    }
    fn lookup(&mut self, _ctx: &FuseCtx, parent: u64, name: &str) -> Result<FuseEntryParam, i32> {
        println!("call lookup parent: {:?} name: {:?}", parent, name);
        if parent == 1 && name == FILE_NAME {
            Ok(rusfuse::FuseEntryParam::new(TEST_FILE_ATTR, 0, 10.0, 10.0))
        } else {
            Err(ENOENT)
        }
    }
    fn forget(&mut self, _ctx: &FuseCtx, _ino: u64, _nlookup: u64) {
        println!("call forget");
    }
    fn getattr(
        &mut self,
        _ctx: &FuseCtx,
        ino: u64,
        fi: Option<&mut FuseFileInfo>,
    ) -> Result<(FuseAttr, f64), i32> {
        println!("call getattr ino: {:?} fi: {:?}", ino, fi);
        if ino == 1 {
            Ok((TEST_DIR_ATTR, 1.0))
        } else if ino == 2 {
            Ok((TEST_FILE_ATTR, 1.0))
        } else {
            Err(ENOENT)
        }
    }
    fn setattr(
        &mut self,
        _ctx: &FuseCtx,
        _ino: u64,
        _attr: &FuseAttr,
        _to_set: i16,
        _fi: Option<&mut FuseFileInfo>,
    ) -> Result<(FuseAttr, f64), i32> {
        println!("call setattr");
        Err(ENOSYS)
    }
    fn readlink(&mut self, _ctx: &FuseCtx, _ino: u64) -> Result<&str, i32> {
        println!("call readlink");
        Err(ENOSYS)
    }
    fn mknod(
        &mut self,
        _ctx: &FuseCtx,
        _parent: u64,
        _name: &str,
        _mode: u32,
        _rdev: u64,
    ) -> Result<FuseEntryParam, i32> {
        println!("call mknod");
        Err(ENOSYS)
    }
    fn mkdir(
        &mut self,
        _ctx: &FuseCtx,
        _parent: u64,
        _name: &str,
        _mode: u32,
    ) -> Result<FuseEntryParam, i32> {
        println!("call mkdir");
        Err(ENOSYS)
    }
    fn unlink(&mut self, _ctx: &FuseCtx, _parent: u64, _name: &str) -> Result<(), i32> {
        println!("call unlink");
        Err(ENOSYS)
    }
    fn rmdir(&mut self, _ctx: &FuseCtx, _parent: u64, _name: &str) -> Result<(), i32> {
        println!("call rmdir");
        Err(ENOSYS)
    }
    fn symlink(
        &mut self,
        _ctx: &FuseCtx,
        _link: &str,
        _parent: u64,
        _name: &str,
    ) -> Result<FuseEntryParam, i32> {
        println!("call symlink");
        Err(ENOSYS)
    }
    fn rename(
        &mut self,
        _ctx: &FuseCtx,
        _parent: u64,
        _name: &str,
        _newparent: u64,
        _newname: &str,
        _flags: u16,
    ) -> Result<(), i32> {
        println!("call rename");
        Err(ENOSYS)
    }
    fn link(
        &mut self,
        _ctx: &FuseCtx,
        _ino: u64,
        _newparent: u64,
        _newname: &str,
    ) -> Result<FuseEntryParam, i32> {
        println!("call link");
        Err(ENOSYS)
    }
    fn open(
        &mut self,
        _ctx: &FuseCtx,
        _ino: u64,
        _fi: &mut FuseFileInfo,
    ) -> Result<FuseFileInfo, i32> {
        println!("call open");
        Err(ENOSYS)
    }
    fn read(
        &mut self,
        _ctx: &FuseCtx,
        ino: u64,
        _size: usize,
        _off: i64,
        _fi: &mut FuseFileInfo,
    ) -> Result<&str, i32> {
        println!("call read");
        if ino == 2 {
            Ok(TEXT)
        } else {
            Err(ENOSYS)
        }
    }
    fn write(
        &mut self,
        _ctx: &FuseCtx,
        _ino: u64,
        _buf: &str,
        _size: usize,
        _off: i64,
        _fi: &mut FuseFileInfo,
    ) -> Result<usize, i32> {
        println!("call write");
        Err(ENOSYS)
    }
    fn flush(&mut self, _ctx: &FuseCtx, _ino: u64, _fi: &mut FuseFileInfo) -> Result<(), i32> {
        println!("call flush");
        Err(ENOSYS)
    }
    fn release(&mut self, _ctx: &FuseCtx, _ino: u64, _fi: &mut FuseFileInfo) -> Result<(), i32> {
        println!("call release");
        Err(ENOSYS)
    }
    fn fsync(
        &mut self,
        _ctx: &FuseCtx,
        _ino: u64,
        _datasync: i32,
        _fi: &mut FuseFileInfo,
    ) -> Result<(), i32> {
        println!("call fsync");
        Err(ENOSYS)
    }
    fn opendir(
        &mut self,
        _ctx: &FuseCtx,
        ino: u64,
        fi: &mut FuseFileInfo,
    ) -> Result<FuseFileInfo, i32> {
        println!("call opendir ino: {:?} fi: {:?}", ino, fi);
        Err(ENOSYS)
    }
    fn readdir(
        &mut self,
        _ctx: &FuseCtx,
        ino: u64,
        _size: usize,
        _off: i64,
        _fi: &mut FuseFileInfo,
    ) -> Result<Vec<FuseDirectory>, i32> {
        println!(
            "call readdir ino: {:?}, size: {:?}, off: {:?}",
            ino, _size, _off
        );
        if ino != 1 {
            Err(ENOSYS)
        } else {
            Ok(vec![
                FuseDirectory {
                    name: ".".to_owned(),
                    file_type: FileType::Directory,
                    ino: 1,
                },
                FuseDirectory {
                    name: "..".to_owned(),
                    file_type: FileType::Directory,
                    ino: 1,
                },
                FuseDirectory {
                    name: FILE_NAME.to_owned(),
                    file_type: FileType::RegularFile,
                    ino: 2,
                },
            ])
        }
    }
    fn releasedir(&mut self, _ctx: &FuseCtx, _ino: u64, _fi: &mut FuseFileInfo) -> Result<(), i32> {
        println!("call releasedir");
        Err(ENOSYS)
    }
    fn fsyncdir(
        &mut self,
        _ctx: &FuseCtx,
        _ino: u64,
        _datasync: i32,
        _fi: &mut FuseFileInfo,
    ) -> Result<(), i32> {
        println!("call fsyncdir");
        Err(ENOSYS)
    }
    fn statfs(&mut self, _ctx: &FuseCtx, _ino: u64) -> Result<FuseStatvfs, i32> {
        println!("call statfs");
        Err(ENOSYS)
    }
    fn setxattr(
        &mut self,
        _ctx: &FuseCtx,
        _ino: u64,
        _name: &str,
        _value: &str,
        _size: usize,
        _flags: i32,
    ) -> Result<(), i32> {
        println!("call setxattr");
        Err(ENOSYS)
    }
    fn getxattr(
        &mut self,
        _ctx: &FuseCtx,
        _ino: u64,
        _name: &str,
        _size: usize,
    ) -> Result<&str, i32> {
        println!("call getxattr");
        Err(ENOSYS)
    }
    fn listxattr(&mut self, _ctx: &FuseCtx, _ino: u64, _size: usize) -> Result<&str, i32> {
        println!("call listxattr");
        Err(ENOSYS)
    }
    fn removexattr(&mut self, _ctx: &FuseCtx, _ino: u64, _name: &str) -> Result<(), i32> {
        println!("call removexattr");
        Err(ENOSYS)
    }
    fn access(&mut self, _ctx: &FuseCtx, _ino: u64, _mask: i32) -> Result<(), i32> {
        println!("call access");
        Err(ENOSYS)
    }
    fn create(
        &mut self,
        _ctx: &FuseCtx,
        _parent: u64,
        _name: &str,
        _mode: u32,
        _fi: &mut FuseFileInfo,
    ) -> Result<FuseEntryParam, i32> {
        println!("call create");
        Err(ENOSYS)
    }
    fn getlk(
        &mut self,
        _ctx: &FuseCtx,
        _ino: u64,
        _fi: &mut FuseFileInfo,
        _lock: &mut FuseLock,
    ) -> Result<FuseLock, i32> {
        println!("call init");
        Err(ENOSYS)
    }
    fn setlk(
        &mut self,
        _ctx: &FuseCtx,
        _ino: u64,
        _fi: &mut FuseFileInfo,
        _lock: &mut FuseLock,
        _sleep: i32,
    ) -> Result<(), i32> {
        println!("call setlk");
        Err(ENOSYS)
    }
    fn bmap(
        &mut self,
        _ctx: &FuseCtx,
        _ino: u64,
        _blocksize: usize,
        _idx: u64,
    ) -> Result<u64, i32> {
        println!("call bmap");
        Err(ENOSYS)
    }
    fn poll(
        &mut self,
        _ctx: &FuseCtx,
        _ino: u64,
        _fi: &mut FuseFileInfo,
        _ph: &mut FusePollhandle,
    ) -> Result<u32, i32> {
        println!("call poll");
        Err(ENOSYS)
    }
    fn write_buf(
        &mut self,
        _ctx: &FuseCtx,
        _ino: u64,
        _bufv: &mut FuseBufvec,
        _off: i64,
        _fi: &FuseFileInfo,
    ) -> Result<usize, i32> {
        println!("call write_buf");
        Err(ENOSYS)
    }
    fn forget_multi(&mut self, _ctx: &FuseCtx, _count: usize, _forgets: &mut FuseForgetData) {
        println!("call forget_multi");
    }
    fn flock(
        &mut self,
        _ctx: &FuseCtx,
        _ino: u64,
        _fi: &mut FuseFileInfo,
        _op: i32,
    ) -> Result<(), i32> {
        println!("call flock");
        Err(ENOSYS)
    }
    fn fallocate(
        &mut self,
        _ctx: &FuseCtx,
        _ino: u64,
        _mode: i32,
        _offset: i64,
        _length: i64,
        _fi: &mut FuseFileInfo,
    ) -> Result<(), i32> {
        println!("call fallocate");
        Err(ENOSYS)
    }
    fn readdirplus(
        &mut self,
        _ctx: &FuseCtx,
        _ino: u64,
        _size: usize,
        _off: i64,
        _fi: &mut FuseFileInfo,
    ) -> Result<&str, i32> {
        println!("call readdirplus");
        Err(ENOSYS)
    }
    fn copy_file_range(
        &mut self,
        _ctx: &FuseCtx,
        _ino_in: u64,
        _off_in: i64,
        _fi_in: &mut FuseFileInfo,
        _ino_out: u64,
        _off_out: i64,
        _fi_out: &mut FuseFileInfo,
        _len: usize,
        _flags: i32,
    ) -> Result<usize, i32> {
        println!("call copy_file_range");
        Err(ENOSYS)
    }
    fn lseek(
        &mut self,
        _ctx: &FuseCtx,
        _ino: u64,
        _off: i64,
        _whence: i32,
        _fi: &mut FuseFileInfo,
    ) -> Result<i64, i32> {
        println!("call lseek");
        Err(ENOSYS)
    }
}
fn main() {
    let mountpoint: String = env::args().nth(1).unwrap();
    let mut file_system = ExampleFs {};
    let ops = FuseOpFlag::Init
        | FuseOpFlag::Destroy
        | FuseOpFlag::Lookup
        | FuseOpFlag::Readdir
        | FuseOpFlag::Read
        | FuseOpFlag::Getattr
        | FuseOpFlag::Open;
    rusfuse::fuse_loop(&mountpoint, &mut file_system, ops);
}

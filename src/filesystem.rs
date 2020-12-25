use crate::fuse::{
    FuseAttr, FuseBufvec, FuseCtx, FuseEntryParam, FuseFileInfo, FuseForgetData, FuseLock,
    FusePollhandle, FuseStatvfs,
};
use libc::ENOSYS;

pub trait FileSystem {
    fn init(&mut self) -> Result<(), i32> {
        Err(ENOSYS)
    }
    fn destroy(&mut self) -> Result<(), i32> {
        Err(ENOSYS)
    }
    fn lookup(&mut self, _ctx: &FuseCtx, _parent: u64, _name: &str) -> Result<FuseEntryParam, i32> {
        Err(ENOSYS)
    }
    fn forget(&mut self, _ctx: &FuseCtx, _ino: u64, _nlookup: u64) {}
    fn getattr(
        &mut self,
        _ctx: &FuseCtx,
        _ino: u64,
        _fi: Option<&mut FuseFileInfo>,
    ) -> Result<(FuseAttr, f64), i32> {
        Err(ENOSYS)
    }
    fn setattr(
        &mut self,
        _ctx: &FuseCtx,
        _ino: u64,
        _attr: &FuseAttr,
        _to_set: i16,
        _fi: Option<&mut FuseFileInfo>,
    ) -> Result<(FuseAttr, f64), i32> {
        Err(ENOSYS)
    }
    fn readlink(&mut self, _ctx: &FuseCtx, _ino: u64) -> Result<&str, i32> {
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
        Err(ENOSYS)
    }
    fn mkdir(
        &mut self,
        _ctx: &FuseCtx,
        _parent: u64,
        _name: &str,
        _mode: u32,
    ) -> Result<FuseEntryParam, i32> {
        Err(ENOSYS)
    }
    fn unlink(&mut self, _ctx: &FuseCtx, _parent: u64, _name: &str) -> Result<(), i32> {
        Err(ENOSYS)
    }
    fn rmdir(&mut self, _ctx: &FuseCtx, _parent: u64, _name: &str) -> Result<(), i32> {
        Err(ENOSYS)
    }
    fn symlink(
        &mut self,
        _ctx: &FuseCtx,
        _link: &str,
        _parent: u64,
        _name: &str,
    ) -> Result<FuseEntryParam, i32> {
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
        Err(ENOSYS)
    }
    fn link(
        &mut self,
        _ctx: &FuseCtx,
        _ino: u64,
        _newparent: u64,
        _newname: &str,
    ) -> Result<FuseEntryParam, i32> {
        Err(ENOSYS)
    }
    fn open(
        &mut self,
        _ctx: &FuseCtx,
        _ino: u64,
        _fi: &mut FuseFileInfo,
    ) -> Result<FuseFileInfo, i32> {
        Err(ENOSYS)
    }
    fn read(
        &mut self,
        _ctx: &FuseCtx,
        _ino: u64,
        _size: usize,
        _off: i64,
        _fi: &mut FuseFileInfo,
    ) -> Result<&str, i32> {
        Err(ENOSYS)
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
        Err(ENOSYS)
    }
    fn flush(&mut self, _ctx: &FuseCtx, _ino: u64, _fi: &mut FuseFileInfo) -> Result<(), i32> {
        Err(ENOSYS)
    }
    fn release(&mut self, _ctx: &FuseCtx, _ino: u64, _fi: &mut FuseFileInfo) -> Result<(), i32> {
        Err(ENOSYS)
    }
    fn fsync(
        &mut self,
        _ctx: &FuseCtx,
        _ino: u64,
        _datasync: i32,
        _fi: &mut FuseFileInfo,
    ) -> Result<(), i32> {
        Err(ENOSYS)
    }
    fn opendir(
        &mut self,
        _ctx: &FuseCtx,
        _ino: u64,
        _fi: &mut FuseFileInfo,
    ) -> Result<FuseFileInfo, i32> {
        Err(ENOSYS)
    }
    fn readdir(
        &mut self,
        _ctx: &FuseCtx,
        _ino: u64,
        _size: usize,
        _off: i64,
        _fi: &mut FuseFileInfo,
    ) -> Result<&str, i32> {
        Err(ENOSYS)
    }
    fn releasedir(&mut self, _ctx: &FuseCtx, _ino: u64, _fi: &mut FuseFileInfo) -> Result<(), i32> {
        Err(ENOSYS)
    }
    fn fsyncdir(
        &mut self,
        _ctx: &FuseCtx,
        _ino: u64,
        _datasync: i32,
        _fi: &mut FuseFileInfo,
    ) -> Result<(), i32> {
        Err(ENOSYS)
    }
    fn statfs(&mut self, _ctx: &FuseCtx, _ino: u64) -> Result<FuseStatvfs, i32> {
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
        Err(ENOSYS)
    }
    fn getxattr(
        &mut self,
        _ctx: &FuseCtx,
        _ino: u64,
        _name: &str,
        _size: usize,
    ) -> Result<&str, i32> {
        Err(ENOSYS)
    }
    fn listxattr(&mut self, _ctx: &FuseCtx, _ino: u64, _size: usize) -> Result<&str, i32> {
        Err(ENOSYS)
    }
    fn removexattr(&mut self, _ctx: &FuseCtx, _ino: u64, _name: &str) -> Result<(), i32> {
        Err(ENOSYS)
    }
    fn access(&mut self, _ctx: &FuseCtx, _ino: u64, _mask: i32) -> Result<(), i32> {
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
        Err(ENOSYS)
    }
    fn getlk(
        &mut self,
        _ctx: &FuseCtx,
        _ino: u64,
        _fi: &mut FuseFileInfo,
        _lock: &mut FuseLock,
    ) -> Result<FuseLock, i32> {
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
        Err(ENOSYS)
    }
    fn bmap(
        &mut self,
        _ctx: &FuseCtx,
        _ino: u64,
        _blocksize: usize,
        _idx: u64,
    ) -> Result<u64, i32> {
        Err(ENOSYS)
    }
    fn poll(
        &mut self,
        _ctx: &FuseCtx,
        _ino: u64,
        _fi: &mut FuseFileInfo,
        _ph: &mut FusePollhandle,
    ) -> Result<u32, i32> {
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
        Err(ENOSYS)
    }
    // fn retrieve_reply(&mut self, _ctx: &FuseCtx) {}
    fn forget_multi(&mut self, _ctx: &FuseCtx, _count: usize, _forgets: &mut FuseForgetData) {}
    fn flock(
        &mut self,
        _ctx: &FuseCtx,
        _ino: u64,
        _fi: &mut FuseFileInfo,
        _op: i32,
    ) -> Result<(), i32> {
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
        Err(ENOSYS)
    }
}

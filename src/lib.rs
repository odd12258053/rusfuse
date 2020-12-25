use std::borrow::{Borrow, BorrowMut};
use std::env;
use std::ffi::CString;
use std::mem::size_of;

use libc::{c_char, c_int, c_void, dev_t, flock, mode_t, off_t, size_t, stat, ENOSYS};

mod filesystem;
mod fuse;
mod utils;

pub use crate::filesystem::FileSystem;
pub use crate::fuse::FuseAttr;
use crate::fuse::{
    fuse_reply_err, fuse_req_userdata, fuse_session_destroy, fuse_session_loop, fuse_session_mount,
    fuse_session_new, fuse_session_unmount, FuseArgs, FuseBufvec, FuseConnInfo, FuseFileInfo,
    FuseForgetData, FuseLowLevelOps, FusePollhandle, FuseReq,
};

impl FuseLowLevelOps {
    pub fn new<T: FileSystem>() -> FuseLowLevelOps {
        FuseLowLevelOps {
            init: FuseLowLevelOps::init::<T>,
            destroy: FuseLowLevelOps::destroy::<T>,
            lookup: FuseLowLevelOps::lookup::<T>,
            forget: FuseLowLevelOps::forget::<T>,
            getattr: FuseLowLevelOps::getattr::<T>,
            setattr: FuseLowLevelOps::setattr::<T>,
            readlink: FuseLowLevelOps::readlink::<T>,
            mknod: FuseLowLevelOps::mknod::<T>,
            mkdir: FuseLowLevelOps::mkdir::<T>,
            unlink: FuseLowLevelOps::unlink::<T>,
            rmdir: FuseLowLevelOps::rmdir::<T>,
            symlink: FuseLowLevelOps::symlink::<T>,
            rename: FuseLowLevelOps::rename::<T>,
            link: FuseLowLevelOps::link::<T>,
            open: FuseLowLevelOps::open::<T>,
            read: FuseLowLevelOps::read::<T>,
            write: FuseLowLevelOps::write::<T>,
            flush: FuseLowLevelOps::flush::<T>,
            release: FuseLowLevelOps::release::<T>,
            fsync: FuseLowLevelOps::fsync::<T>,
            opendir: FuseLowLevelOps::opendir::<T>,
            readdir: FuseLowLevelOps::readdir::<T>,
            releasedir: FuseLowLevelOps::releasedir::<T>,
            fsyncdir: FuseLowLevelOps::fsyncdir::<T>,
            statfs: FuseLowLevelOps::statfs::<T>,
            setxattr: FuseLowLevelOps::setxattr::<T>,
            getxattr: FuseLowLevelOps::getxattr::<T>,
            listxattr: FuseLowLevelOps::listxattr::<T>,
            removexattr: FuseLowLevelOps::removexattr::<T>,
            access: FuseLowLevelOps::access::<T>,
            create: FuseLowLevelOps::create::<T>,
            getlk: FuseLowLevelOps::getlk::<T>,
            setlk: FuseLowLevelOps::setlk::<T>,
            bmap: FuseLowLevelOps::bmap::<T>,
            poll: FuseLowLevelOps::poll::<T>,
            write_buf: FuseLowLevelOps::write_buf::<T>,
            retrieve_reply: FuseLowLevelOps::retrieve_reply::<T>,
            forget_multi: FuseLowLevelOps::forget_multi::<T>,
            flock: FuseLowLevelOps::flock::<T>,
            fallocate: FuseLowLevelOps::fallocate::<T>,
            readdirplus: FuseLowLevelOps::readdirplus::<T>,
            copy_file_range: FuseLowLevelOps::copy_file_range::<T>,
            lseek: FuseLowLevelOps::lseek::<T>,
        }
    }
    pub fn init<T: FileSystem>(userdata: *mut c_void, _conn: *mut FuseConnInfo) {
        let file_system = userdata as *mut T;
        unsafe {
            let _ = file_system.as_mut().unwrap().init();
        }
    }
    pub fn destroy<T: FileSystem>(userdata: *mut c_void) {
        let file_system = userdata as *mut T;
        unsafe {
            let _ = file_system.as_mut().unwrap().destroy();
        }
    }
    pub fn lookup<T: FileSystem>(req: *mut FuseReq, _parent: u64, _name: *const c_char) {
        let userdata = unsafe { fuse_req_userdata(req) };
        let file_system = userdata as *mut T;
        match unsafe { file_system.as_mut().unwrap().lookup() } {
            Ok(..) => unsafe {
                fuse_reply_err(req, ENOSYS);
            },
            Err(e) => unsafe {
                fuse_reply_err(req, e);
            },
        }
    }
    pub fn forget<T: FileSystem>(req: *mut FuseReq, _ino: u64, _nlookup: u64) {
        let userdata = unsafe { fuse_req_userdata(req) };
        let file_system = userdata as *mut T;
        match unsafe { file_system.as_mut().unwrap().forget() } {
            Ok(..) => unsafe {
                fuse_reply_err(req, ENOSYS);
            },
            Err(e) => unsafe {
                fuse_reply_err(req, e);
            },
        }
    }
    pub fn getattr<T: FileSystem>(req: *mut FuseReq, _ino: u64, _fi: *mut FuseFileInfo) {
        let userdata = unsafe { fuse_req_userdata(req) };
        let file_system = userdata as *mut T;
        match unsafe { file_system.as_mut().unwrap().getattr() } {
            Ok(..) => unsafe {
                fuse_reply_err(req, ENOSYS);
            },
            Err(e) => unsafe {
                fuse_reply_err(req, e);
            },
        }
    }
    pub fn setattr<T: FileSystem>(
        req: *mut FuseReq,
        _ino: u64,
        _attr: *mut stat,
        _to_set: i16,
        _fi: *mut FuseFileInfo,
    ) {
        let userdata = unsafe { fuse_req_userdata(req) };
        let file_system = userdata as *mut T;
        match unsafe { file_system.as_mut().unwrap().setattr() } {
            Ok(..) => unsafe {
                fuse_reply_err(req, ENOSYS);
            },
            Err(e) => unsafe {
                fuse_reply_err(req, e);
            },
        }
    }
    pub fn readlink<T: FileSystem>(req: *mut FuseReq, _ino: u64) {
        let userdata = unsafe { fuse_req_userdata(req) };
        let file_system = userdata as *mut T;
        match unsafe { file_system.as_mut().unwrap().readlink() } {
            Ok(..) => unsafe {
                fuse_reply_err(req, ENOSYS);
            },
            Err(e) => unsafe {
                fuse_reply_err(req, e);
            },
        }
    }
    pub fn mknod<T: FileSystem>(
        req: *mut FuseReq,
        _parent: u64,
        _name: *const c_char,
        _mode: mode_t,
        _rdev: dev_t,
    ) {
        let userdata = unsafe { fuse_req_userdata(req) };
        let file_system = userdata as *mut T;
        match unsafe { file_system.as_mut().unwrap().mknod() } {
            Ok(..) => unsafe {
                fuse_reply_err(req, ENOSYS);
            },
            Err(e) => unsafe {
                fuse_reply_err(req, e);
            },
        }
    }
    pub fn mkdir<T: FileSystem>(
        req: *mut FuseReq,
        _parent: u64,
        _name: *const c_char,
        _mode: mode_t,
    ) {
        let userdata = unsafe { fuse_req_userdata(req) };
        let file_system = userdata as *mut T;
        match unsafe { file_system.as_mut().unwrap().mkdir() } {
            Ok(..) => unsafe {
                fuse_reply_err(req, ENOSYS);
            },
            Err(e) => unsafe {
                fuse_reply_err(req, e);
            },
        }
    }
    pub fn unlink<T: FileSystem>(req: *mut FuseReq, _parent: u64, _name: *const c_char) {
        let userdata = unsafe { fuse_req_userdata(req) };
        let file_system = userdata as *mut T;
        match unsafe { file_system.as_mut().unwrap().unlink() } {
            Ok(..) => unsafe {
                fuse_reply_err(req, ENOSYS);
            },
            Err(e) => unsafe {
                fuse_reply_err(req, e);
            },
        }
    }
    pub fn rmdir<T: FileSystem>(req: *mut FuseReq, _parent: u64, _name: *const c_char) {
        let userdata = unsafe { fuse_req_userdata(req) };
        let file_system = userdata as *mut T;
        match unsafe { file_system.as_mut().unwrap().rmdir() } {
            Ok(..) => unsafe {
                fuse_reply_err(req, ENOSYS);
            },
            Err(e) => unsafe {
                fuse_reply_err(req, e);
            },
        }
    }
    pub fn symlink<T: FileSystem>(
        req: *mut FuseReq,
        _link: *const c_char,
        _parent: u64,
        _name: *const c_char,
    ) {
        let userdata = unsafe { fuse_req_userdata(req) };
        let file_system = userdata as *mut T;
        match unsafe { file_system.as_mut().unwrap().symlink() } {
            Ok(..) => unsafe {
                fuse_reply_err(req, ENOSYS);
            },
            Err(e) => unsafe {
                fuse_reply_err(req, e);
            },
        }
    }
    pub fn rename<T: FileSystem>(
        req: *mut FuseReq,
        _parent: u64,
        _name: *const c_char,
        _newparent: u64,
        _newname: *const c_char,
        _flags: u16,
    ) {
        let userdata = unsafe { fuse_req_userdata(req) };
        let file_system = userdata as *mut T;
        match unsafe { file_system.as_mut().unwrap().rename() } {
            Ok(..) => unsafe {
                fuse_reply_err(req, ENOSYS);
            },
            Err(e) => unsafe {
                fuse_reply_err(req, e);
            },
        }
    }
    pub fn link<T: FileSystem>(
        req: *mut FuseReq,
        _ino: u64,
        _newparent: u64,
        _newname: *const c_char,
    ) {
        let userdata = unsafe { fuse_req_userdata(req) };
        let file_system = userdata as *mut T;
        match unsafe { file_system.as_mut().unwrap().link() } {
            Ok(..) => unsafe {
                fuse_reply_err(req, ENOSYS);
            },
            Err(e) => unsafe {
                fuse_reply_err(req, e);
            },
        }
    }
    pub fn open<T: FileSystem>(req: *mut FuseReq, _ino: u64, _fi: *mut FuseFileInfo) {
        let userdata = unsafe { fuse_req_userdata(req) };
        let file_system = userdata as *mut T;
        match unsafe { file_system.as_mut().unwrap().open() } {
            Ok(..) => unsafe {
                fuse_reply_err(req, ENOSYS);
            },
            Err(e) => unsafe {
                fuse_reply_err(req, e);
            },
        }
    }
    pub fn read<T: FileSystem>(
        req: *mut FuseReq,
        _ino: u64,
        _size: size_t,
        _off: off_t,
        _fi: *mut FuseFileInfo,
    ) {
        let userdata = unsafe { fuse_req_userdata(req) };
        let file_system = userdata as *mut T;
        match unsafe { file_system.as_mut().unwrap().read() } {
            Ok(..) => unsafe {
                fuse_reply_err(req, ENOSYS);
            },
            Err(e) => unsafe {
                fuse_reply_err(req, e);
            },
        }
    }
    pub fn write<T: FileSystem>(
        req: *mut FuseReq,
        _ino: u64,
        _buf: *const c_char,
        _size: size_t,
        _off: off_t,
        _fi: *mut FuseFileInfo,
    ) {
        let userdata = unsafe { fuse_req_userdata(req) };
        let file_system = userdata as *mut T;
        match unsafe { file_system.as_mut().unwrap().write() } {
            Ok(..) => unsafe {
                fuse_reply_err(req, ENOSYS);
            },
            Err(e) => unsafe {
                fuse_reply_err(req, e);
            },
        }
    }
    pub fn flush<T: FileSystem>(req: *mut FuseReq, _ino: u64, _fi: *mut FuseFileInfo) {
        let userdata = unsafe { fuse_req_userdata(req) };
        let file_system = userdata as *mut T;
        match unsafe { file_system.as_mut().unwrap().flush() } {
            Ok(..) => unsafe {
                fuse_reply_err(req, ENOSYS);
            },
            Err(e) => unsafe {
                fuse_reply_err(req, e);
            },
        }
    }
    pub fn release<T: FileSystem>(req: *mut FuseReq, _ino: u64, _fi: *mut FuseFileInfo) {
        let userdata = unsafe { fuse_req_userdata(req) };
        let file_system = userdata as *mut T;
        match unsafe { file_system.as_mut().unwrap().release() } {
            Ok(..) => unsafe {
                fuse_reply_err(req, ENOSYS);
            },
            Err(e) => unsafe {
                fuse_reply_err(req, e);
            },
        }
    }
    pub fn fsync<T: FileSystem>(
        req: *mut FuseReq,
        _ino: u64,
        _datasync: c_int,
        _fi: *mut FuseFileInfo,
    ) {
        let userdata = unsafe { fuse_req_userdata(req) };
        let file_system = userdata as *mut T;
        match unsafe { file_system.as_mut().unwrap().fsync() } {
            Ok(..) => unsafe {
                fuse_reply_err(req, ENOSYS);
            },
            Err(e) => unsafe {
                fuse_reply_err(req, e);
            },
        }
    }
    pub fn opendir<T: FileSystem>(req: *mut FuseReq, _ino: u64, _fi: *mut FuseFileInfo) {
        let userdata = unsafe { fuse_req_userdata(req) };
        let file_system = userdata as *mut T;
        match unsafe { file_system.as_mut().unwrap().opendir() } {
            Ok(..) => unsafe {
                fuse_reply_err(req, ENOSYS);
            },
            Err(e) => unsafe {
                fuse_reply_err(req, e);
            },
        }
    }
    pub fn readdir<T: FileSystem>(
        req: *mut FuseReq,
        _ino: u64,
        _size: size_t,
        _off: off_t,
        _fi: *mut FuseFileInfo,
    ) {
        let userdata = unsafe { fuse_req_userdata(req) };
        let file_system = userdata as *mut T;
        match unsafe { file_system.as_mut().unwrap().readdir() } {
            Ok(..) => unsafe {
                fuse_reply_err(req, ENOSYS);
            },
            Err(e) => unsafe {
                fuse_reply_err(req, e);
            },
        }
    }
    pub fn releasedir<T: FileSystem>(req: *mut FuseReq, _ino: u64, _fi: *mut FuseFileInfo) {
        let userdata = unsafe { fuse_req_userdata(req) };
        let file_system = userdata as *mut T;
        match unsafe { file_system.as_mut().unwrap().releasedir() } {
            Ok(..) => unsafe {
                fuse_reply_err(req, ENOSYS);
            },
            Err(e) => unsafe {
                fuse_reply_err(req, e);
            },
        }
    }
    pub fn fsyncdir<T: FileSystem>(
        req: *mut FuseReq,
        _ino: u64,
        _datasync: c_int,
        _fi: *mut FuseFileInfo,
    ) {
        let userdata = unsafe { fuse_req_userdata(req) };
        let file_system = userdata as *mut T;
        match unsafe { file_system.as_mut().unwrap().fsyncdir() } {
            Ok(..) => unsafe {
                fuse_reply_err(req, ENOSYS);
            },
            Err(e) => unsafe {
                fuse_reply_err(req, e);
            },
        }
    }
    pub fn statfs<T: FileSystem>(req: *mut FuseReq, _ino: u64) {
        let userdata = unsafe { fuse_req_userdata(req) };
        let file_system = userdata as *mut T;
        match unsafe { file_system.as_mut().unwrap().statfs() } {
            Ok(..) => unsafe {
                fuse_reply_err(req, ENOSYS);
            },
            Err(e) => unsafe {
                fuse_reply_err(req, e);
            },
        }
    }
    pub fn setxattr<T: FileSystem>(
        req: *mut FuseReq,
        _ino: u64,
        _name: *const c_char,
        _value: *const c_char,
        _size: size_t,
        _flags: c_int,
    ) {
        let userdata = unsafe { fuse_req_userdata(req) };
        let file_system = userdata as *mut T;
        match unsafe { file_system.as_mut().unwrap().setxattr() } {
            Ok(..) => unsafe {
                fuse_reply_err(req, ENOSYS);
            },
            Err(e) => unsafe {
                fuse_reply_err(req, e);
            },
        }
    }
    pub fn getxattr<T: FileSystem>(
        req: *mut FuseReq,
        _ino: u64,
        _name: *const c_char,
        _size: size_t,
    ) {
        let userdata = unsafe { fuse_req_userdata(req) };
        let file_system = userdata as *mut T;
        match unsafe { file_system.as_mut().unwrap().getxattr() } {
            Ok(..) => unsafe {
                fuse_reply_err(req, ENOSYS);
            },
            Err(e) => unsafe {
                fuse_reply_err(req, e);
            },
        }
    }
    pub fn listxattr<T: FileSystem>(req: *mut FuseReq, _ino: u64, _size: size_t) {
        let userdata = unsafe { fuse_req_userdata(req) };
        let file_system = userdata as *mut T;
        match unsafe { file_system.as_mut().unwrap().listxattr() } {
            Ok(..) => unsafe {
                fuse_reply_err(req, ENOSYS);
            },
            Err(e) => unsafe {
                fuse_reply_err(req, e);
            },
        }
    }
    pub fn removexattr<T: FileSystem>(req: *mut FuseReq, _ino: u64, _name: *const c_char) {
        let userdata = unsafe { fuse_req_userdata(req) };
        let file_system = userdata as *mut T;
        match unsafe { file_system.as_mut().unwrap().removexattr() } {
            Ok(..) => unsafe {
                fuse_reply_err(req, ENOSYS);
            },
            Err(e) => unsafe {
                fuse_reply_err(req, e);
            },
        }
    }
    pub fn access<T: FileSystem>(req: *mut FuseReq, _ino: u64, _mask: c_int) {
        let userdata = unsafe { fuse_req_userdata(req) };
        let file_system = userdata as *mut T;
        match unsafe { file_system.as_mut().unwrap().access() } {
            Ok(..) => unsafe {
                fuse_reply_err(req, ENOSYS);
            },
            Err(e) => unsafe {
                fuse_reply_err(req, e);
            },
        }
    }
    pub fn create<T: FileSystem>(
        req: *mut FuseReq,
        _parent: u64,
        _name: *const c_char,
        _mode: mode_t,
        _fi: *mut FuseFileInfo,
    ) {
        let userdata = unsafe { fuse_req_userdata(req) };
        let file_system = userdata as *mut T;
        match unsafe { file_system.as_mut().unwrap().create() } {
            Ok(..) => unsafe {
                fuse_reply_err(req, ENOSYS);
            },
            Err(e) => unsafe {
                fuse_reply_err(req, e);
            },
        }
    }
    pub fn getlk<T: FileSystem>(
        req: *mut FuseReq,
        _ino: u64,
        _fi: *mut FuseFileInfo,
        _lock: *mut flock,
    ) {
        let userdata = unsafe { fuse_req_userdata(req) };
        let file_system = userdata as *mut T;
        match unsafe { file_system.as_mut().unwrap().getlk() } {
            Ok(..) => unsafe {
                fuse_reply_err(req, ENOSYS);
            },
            Err(e) => unsafe {
                fuse_reply_err(req, e);
            },
        }
    }
    pub fn setlk<T: FileSystem>(
        req: *mut FuseReq,
        _ino: u64,
        _fi: *mut FuseFileInfo,
        _lock: *mut flock,
        _sleep: c_int,
    ) {
        let userdata = unsafe { fuse_req_userdata(req) };
        let file_system = userdata as *mut T;
        match unsafe { file_system.as_mut().unwrap().setlk() } {
            Ok(..) => unsafe {
                fuse_reply_err(req, ENOSYS);
            },
            Err(e) => unsafe {
                fuse_reply_err(req, e);
            },
        }
    }
    pub fn bmap<T: FileSystem>(req: *mut FuseReq, _ino: u64, _blocksize: size_t, _idx: u64) {
        let userdata = unsafe { fuse_req_userdata(req) };
        let file_system = userdata as *mut T;
        match unsafe { file_system.as_mut().unwrap().bmap() } {
            Ok(..) => unsafe {
                fuse_reply_err(req, ENOSYS);
            },
            Err(e) => unsafe {
                fuse_reply_err(req, e);
            },
        }
    }
    pub fn poll<T: FileSystem>(
        req: *mut FuseReq,
        _ino: u64,
        _fi: *mut FuseFileInfo,
        _ph: *mut FusePollhandle,
    ) {
        let userdata = unsafe { fuse_req_userdata(req) };
        let file_system = userdata as *mut T;
        match unsafe { file_system.as_mut().unwrap().poll() } {
            Ok(..) => unsafe {
                fuse_reply_err(req, ENOSYS);
            },
            Err(e) => unsafe {
                fuse_reply_err(req, e);
            },
        }
    }
    pub fn write_buf<T: FileSystem>(
        req: *mut FuseReq,
        _ino: u64,
        _bufv: *mut FuseBufvec,
        _off: off_t,
        _fi: *mut FuseFileInfo,
    ) {
        let userdata = unsafe { fuse_req_userdata(req) };
        let file_system = userdata as *mut T;
        match unsafe { file_system.as_mut().unwrap().write_buf() } {
            Ok(..) => unsafe {
                fuse_reply_err(req, ENOSYS);
            },
            Err(e) => unsafe {
                fuse_reply_err(req, e);
            },
        }
    }
    pub fn retrieve_reply<T: FileSystem>(
        req: *mut FuseReq,
        _cookie: *mut c_void,
        _ino: u64,
        _off: off_t,
        _bufv: *mut FuseBufvec,
    ) {
        let userdata = unsafe { fuse_req_userdata(req) };
        let file_system = userdata as *mut T;
        match unsafe { file_system.as_mut().unwrap().retrieve_reply() } {
            Ok(..) => unsafe {
                fuse_reply_err(req, ENOSYS);
            },
            Err(e) => unsafe {
                fuse_reply_err(req, e);
            },
        }
    }
    pub fn forget_multi<T: FileSystem>(
        req: *mut FuseReq,
        _count: size_t,
        _forgets: *mut FuseForgetData,
    ) {
        let userdata = unsafe { fuse_req_userdata(req) };
        let file_system = userdata as *mut T;
        match unsafe { file_system.as_mut().unwrap().forget_multi() } {
            Ok(..) => unsafe {
                fuse_reply_err(req, ENOSYS);
            },
            Err(e) => unsafe {
                fuse_reply_err(req, e);
            },
        }
    }
    pub fn flock<T: FileSystem>(req: *mut FuseReq, _ino: u64, _fi: *mut FuseFileInfo, _op: c_int) {
        let userdata = unsafe { fuse_req_userdata(req) };
        let file_system = userdata as *mut T;
        match unsafe { file_system.as_mut().unwrap().flock() } {
            Ok(..) => unsafe {
                fuse_reply_err(req, ENOSYS);
            },
            Err(e) => unsafe {
                fuse_reply_err(req, e);
            },
        }
    }
    pub fn fallocate<T: FileSystem>(
        req: *mut FuseReq,
        _ino: u64,
        _mode: c_int,
        _offset: off_t,
        _length: off_t,
        _fi: *mut FuseFileInfo,
    ) {
        let userdata = unsafe { fuse_req_userdata(req) };
        let file_system = userdata as *mut T;
        match unsafe { file_system.as_mut().unwrap().fallocate() } {
            Ok(..) => unsafe {
                fuse_reply_err(req, ENOSYS);
            },
            Err(e) => unsafe {
                fuse_reply_err(req, e);
            },
        }
    }
    pub fn readdirplus<T: FileSystem>(
        req: *mut FuseReq,
        _ino: u64,
        _size: size_t,
        _off: off_t,
        _fi: *mut FuseFileInfo,
    ) {
        let userdata = unsafe { fuse_req_userdata(req) };
        let file_system = userdata as *mut T;
        match unsafe { file_system.as_mut().unwrap().readdirplus() } {
            Ok(..) => unsafe {
                fuse_reply_err(req, ENOSYS);
            },
            Err(e) => unsafe {
                fuse_reply_err(req, e);
            },
        }
    }
    pub fn copy_file_range<T: FileSystem>(
        req: *mut FuseReq,
        _ino_in: u64,
        _off_in: off_t,
        _fi_in: *mut FuseFileInfo,
        _ino_out: u64,
        _off_out: off_t,
        _fi_out: *mut FuseFileInfo,
        _len: size_t,
        _flags: c_int,
    ) {
        let userdata = unsafe { fuse_req_userdata(req) };
        let file_system = userdata as *mut T;
        match unsafe { file_system.as_mut().unwrap().copy_file_range() } {
            Ok(..) => unsafe {
                fuse_reply_err(req, ENOSYS);
            },
            Err(e) => unsafe {
                fuse_reply_err(req, e);
            },
        }
    }
    pub fn lseek<T: FileSystem>(
        req: *mut FuseReq,
        _ino: u64,
        _off: off_t,
        _whence: c_int,
        _fi: *mut FuseFileInfo,
    ) {
        let userdata = unsafe { fuse_req_userdata(req) };
        let file_system = userdata as *mut T;
        match unsafe { file_system.as_mut().unwrap().lseek() } {
            Ok(..) => unsafe {
                fuse_reply_err(req, ENOSYS);
            },
            Err(e) => unsafe {
                fuse_reply_err(req, e);
            },
        }
    }
}

pub fn fuse_loop<T: FileSystem>(mountpoint: &str, file_system: &mut T) {
    let arg0 = CString::new(env::args().nth(0).unwrap()).unwrap();
    let mountpoint = CString::new(mountpoint).unwrap();
    let c_argv: Vec<*const c_char> = vec![arg0.as_ptr()];
    let mut fuse_args = FuseArgs {
        argc: 1 as c_int,
        argv: c_argv.as_ptr(),
        allocated: 0 as c_int,
    };
    let op = FuseLowLevelOps::new::<T>();

    let session = unsafe {
        let session = fuse_session_new(
            fuse_args.borrow_mut(),
            op.borrow(),
            size_of::<FuseLowLevelOps>(),
            file_system.borrow_mut() as *mut T as *mut c_void,
        );
        let _ = fuse_session_mount(session, mountpoint.as_ptr());
        session
    };

    let _ = unsafe { fuse_session_loop(session) };

    unsafe {
        fuse_session_unmount(session);
        fuse_session_destroy(session);
    }
}

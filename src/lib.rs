use std::borrow::{Borrow, BorrowMut};
use std::env;
use std::ffi::{CStr, CString};
use std::mem::size_of;
use std::ptr::{null, null_mut};

use libc::{c_char, c_int, c_void, dev_t, flock, mode_t, off_t, size_t, stat};

mod filesystem;
mod fuse;
mod utils;

pub use crate::filesystem::FileSystem;
use crate::fuse::{
    fuse_add_direntry, fuse_reply_attr, fuse_reply_bmap, fuse_reply_buf, fuse_reply_create,
    fuse_reply_entry, fuse_reply_err, fuse_reply_lock, fuse_reply_lseek, fuse_reply_none,
    fuse_reply_open, fuse_reply_poll, fuse_reply_readlink, fuse_reply_statfs, fuse_reply_write,
    fuse_reply_xattr, fuse_req_ctx, fuse_req_userdata, fuse_session_destroy, fuse_session_loop,
    fuse_session_mount, fuse_session_new, fuse_session_unmount, FuseArgs, FuseConnInfo,
    FuseLowLevelOps, FuseReq,
};
pub use crate::fuse::{
    FileType, FuseAttr, FuseBufvec, FuseCtx, FuseDirectory, FuseEntryParam, FuseFileInfo,
    FuseForgetData, FuseLock, FusePollhandle, FuseStatvfs,
};
use std::cmp::min;
use std::ops::{BitAnd, BitOr};

pub enum FuseOpFlag {
    Init = 1 << 0,
    Destroy = 1 << 1,
    Lookup = 1 << 2,
    Forget = 1 << 3,
    Getattr = 1 << 4,
    Setattr = 1 << 5,
    Readlink = 1 << 6,
    Mknod = 1 << 7,
    Mkdir = 1 << 8,
    Unlink = 1 << 9,
    Rmdir = 1 << 10,
    Symlink = 1 << 11,
    Rename = 1 << 12,
    Link = 1 << 13,
    Open = 1 << 14,
    Read = 1 << 15,
    Write = 1 << 16,
    Flush = 1 << 17,
    Release = 1 << 18,
    Fsync = 1 << 19,
    Opendir = 1 << 20,
    Readdir = 1 << 21,
    Releasedir = 1 << 22,
    Fsyncdir = 1 << 23,
    Statfs = 1 << 24,
    Setxattr = 1 << 25,
    Getxattr = 1 << 26,
    Listxattr = 1 << 27,
    Removexattr = 1 << 28,
    Access = 1 << 29,
    Create = 1 << 30,
    Getlk = 1 << 31,
    Setlk = 1 << 32,
    Bmap = 1 << 33,
    Poll = 1 << 34,
    WriteBuf = 1 << 35,
    ForgetMulti = 1 << 36,
    Flock = 1 << 37,
    Fallocate = 1 << 38,
    Readdirplus = 1 << 39,
    CopyFileRange = 1 << 40,
    Lseek = 1 << 41,
}

impl BitOr<FuseOpFlag> for FuseOpFlag {
    type Output = u64;
    fn bitor(self, rhs: Self) -> Self::Output {
        self as u64 | rhs as u64
    }
}
impl BitOr<FuseOpFlag> for u64 {
    type Output = u64;
    fn bitor(self, rhs: FuseOpFlag) -> Self::Output {
        self | rhs as u64
    }
}
impl BitAnd<FuseOpFlag> for u64 {
    type Output = u64;
    fn bitand(self, rhs: FuseOpFlag) -> Self::Output {
        self & rhs as u64
    }
}

struct FuseOps;

macro_rules! filesystem {
    ($req:expr) => {
        unsafe {
            let userdata = fuse_req_userdata($req);
            (userdata as *mut T).as_mut().unwrap()
        }
    };
}

macro_rules! ctx {
    ($req:expr) => {
        unsafe { fuse_req_ctx($req).as_ref().unwrap() }
    };
}

macro_rules! to_str {
    ($char:expr) => {
        CStr::from_ptr($char).to_str().unwrap()
    };
}

macro_rules! op {
    ($ops:expr, $name:ident, $flag:ident) => {
        if $ops & FuseOpFlag::$flag == 0 {
            null()
        } else {
            (FuseOps::$name::<T>) as *const _
        }
    };
}

impl FuseOps {
    fn fuse_low_level_ops<T: FileSystem>(ops: u64) -> FuseLowLevelOps {
        FuseLowLevelOps {
            init: op!(ops, init, Init),
            destroy: op!(ops, destroy, Destroy),
            lookup: op!(ops, lookup, Lookup),
            forget: op!(ops, forget, Forget),
            getattr: op!(ops, getattr, Getattr),
            setattr: op!(ops, setattr, Setattr),
            readlink: op!(ops, readlink, Readlink),
            mknod: op!(ops, mknod, Mknod),
            mkdir: op!(ops, mkdir, Mkdir),
            unlink: op!(ops, unlink, Unlink),
            rmdir: op!(ops, rmdir, Rmdir),
            symlink: op!(ops, symlink, Symlink),
            rename: op!(ops, rename, Rename),
            link: op!(ops, link, Link),
            open: op!(ops, open, Open),
            read: op!(ops, read, Read),
            write: op!(ops, write, Write),
            flush: op!(ops, flush, Flush),
            release: op!(ops, release, Release),
            fsync: op!(ops, fsync, Fsync),
            opendir: op!(ops, opendir, Opendir),
            readdir: op!(ops, readdir, Readdir),
            releasedir: op!(ops, releasedir, Releasedir),
            fsyncdir: op!(ops, fsyncdir, Fsyncdir),
            statfs: op!(ops, statfs, Statfs),
            setxattr: op!(ops, setxattr, Setxattr),
            getxattr: op!(ops, getxattr, Getxattr),
            listxattr: op!(ops, listxattr, Listxattr),
            removexattr: op!(ops, removexattr, Removexattr),
            access: op!(ops, access, Access),
            create: op!(ops, create, Create),
            getlk: op!(ops, getlk, Getlk),
            setlk: op!(ops, setlk, Setlk),
            bmap: op!(ops, bmap, Bmap),
            poll: op!(ops, poll, Poll),
            write_buf: op!(ops, write_buf, WriteBuf),
            forget_multi: op!(ops, forget_multi, ForgetMulti),
            flock: op!(ops, flock, Flock),
            fallocate: op!(ops, fallocate, Fallocate),
            readdirplus: op!(ops, readdirplus, Readdirplus),
            copy_file_range: op!(ops, copy_file_range, CopyFileRange),
            lseek: op!(ops, lseek, Lseek),
        }
    }
    fn init<T: FileSystem>(userdata: *mut c_void, _conn: *mut FuseConnInfo) {
        let file_system = unsafe { (userdata as *mut T).as_mut().unwrap() };
        let _ = file_system.init();
    }
    fn destroy<T: FileSystem>(userdata: *mut c_void) {
        let file_system = unsafe { (userdata as *mut T).as_mut().unwrap() };
        let _ = file_system.destroy();
    }
    fn lookup<T: FileSystem>(req: *mut FuseReq, parent: u64, name: *const c_char) {
        let file_system = filesystem!(req);
        let ctx = ctx!(req);
        match file_system.lookup(ctx, parent, unsafe { to_str!(name) }) {
            Ok(entry_param) => unsafe {
                let _ret = fuse_reply_entry(req, entry_param.borrow());
            },
            Err(e) => unsafe {
                fuse_reply_err(req, e);
            },
        }
    }
    fn forget<T: FileSystem>(req: *mut FuseReq, ino: u64, nlookup: u64) {
        let file_system = filesystem!(req);
        let ctx = ctx!(req);
        file_system.forget(ctx, ino, nlookup);
        unsafe {
            fuse_reply_none(req);
        }
    }
    fn getattr<T: FileSystem>(req: *mut FuseReq, ino: u64, fi: *mut FuseFileInfo) {
        let file_system = filesystem!(req);
        let ctx = ctx!(req);
        match file_system.getattr(ctx, ino, unsafe { fi.as_mut() }) {
            Ok((attr, timeout)) => unsafe {
                let _ret = fuse_reply_attr(req, attr.convert().borrow(), timeout);
            },
            Err(e) => unsafe {
                fuse_reply_err(req, e);
            },
        }
    }
    fn setattr<T: FileSystem>(
        req: *mut FuseReq,
        ino: u64,
        attr: *mut stat,
        to_set: i16,
        fi: *mut FuseFileInfo,
    ) {
        let file_system = filesystem!(req);
        let ctx = ctx!(req);
        match file_system.setattr(
            ctx,
            ino,
            &FuseAttr::new(unsafe { attr.as_ref().unwrap() }),
            to_set,
            unsafe { fi.as_mut() },
        ) {
            Ok((attr, timeout)) => unsafe {
                let _ret = fuse_reply_attr(req, attr.convert().borrow(), timeout);
            },
            Err(e) => unsafe {
                fuse_reply_err(req, e);
            },
        }
    }
    fn readlink<T: FileSystem>(req: *mut FuseReq, ino: u64) {
        let file_system = filesystem!(req);
        let ctx = ctx!(req);
        match file_system.readlink(ctx, ino) {
            Ok(link) => unsafe {
                let _ret = fuse_reply_readlink(req, CString::new(link).unwrap().as_ptr());
            },
            Err(e) => unsafe {
                fuse_reply_err(req, e);
            },
        }
    }
    fn mknod<T: FileSystem>(
        req: *mut FuseReq,
        parent: u64,
        name: *const c_char,
        mode: mode_t,
        rdev: dev_t,
    ) {
        let file_system = filesystem!(req);
        let ctx = ctx!(req);
        match file_system.mknod(ctx, parent, unsafe { to_str!(name) }, mode, rdev) {
            Ok(entry_param) => unsafe {
                let _ret = fuse_reply_entry(req, entry_param.borrow());
            },
            Err(e) => unsafe {
                fuse_reply_err(req, e);
            },
        }
    }
    fn mkdir<T: FileSystem>(req: *mut FuseReq, parent: u64, name: *const c_char, mode: mode_t) {
        let file_system = filesystem!(req);
        let ctx = ctx!(req);
        match file_system.mkdir(ctx, parent, unsafe { to_str!(name) }, mode) {
            Ok(entry_param) => unsafe {
                let _ret = fuse_reply_entry(req, entry_param.borrow());
            },
            Err(e) => unsafe {
                fuse_reply_err(req, e);
            },
        }
    }
    fn unlink<T: FileSystem>(req: *mut FuseReq, parent: u64, name: *const c_char) {
        let file_system = filesystem!(req);
        let ctx = ctx!(req);
        match file_system.unlink(ctx, parent, unsafe { to_str!(name) }) {
            Ok(..) => {}
            Err(e) => unsafe {
                fuse_reply_err(req, e);
            },
        }
    }
    fn rmdir<T: FileSystem>(req: *mut FuseReq, parent: u64, name: *const c_char) {
        let file_system = filesystem!(req);
        let ctx = ctx!(req);
        match file_system.rmdir(ctx, parent, unsafe { to_str!(name) }) {
            Ok(..) => {}
            Err(e) => unsafe {
                fuse_reply_err(req, e);
            },
        }
    }
    fn symlink<T: FileSystem>(
        req: *mut FuseReq,
        link: *const c_char,
        parent: u64,
        name: *const c_char,
    ) {
        let file_system = filesystem!(req);
        let ctx = ctx!(req);
        match file_system.symlink(ctx, unsafe { to_str!(link) }, parent, unsafe {
            to_str!(name)
        }) {
            Ok(entry_param) => unsafe {
                let _ret = fuse_reply_entry(req, entry_param.borrow());
            },
            Err(e) => unsafe {
                fuse_reply_err(req, e);
            },
        }
    }
    fn rename<T: FileSystem>(
        req: *mut FuseReq,
        parent: u64,
        name: *const c_char,
        newparent: u64,
        newname: *const c_char,
        flags: u16,
    ) {
        let file_system = filesystem!(req);
        let ctx = ctx!(req);
        match file_system.rename(
            ctx,
            parent,
            unsafe { to_str!(name) },
            newparent,
            unsafe { to_str!(newname) },
            flags,
        ) {
            Ok(..) => {}
            Err(e) => unsafe {
                fuse_reply_err(req, e);
            },
        }
    }
    fn link<T: FileSystem>(req: *mut FuseReq, ino: u64, newparent: u64, newname: *const c_char) {
        let file_system = filesystem!(req);
        let ctx = ctx!(req);
        match file_system.link(ctx, ino, newparent, unsafe { to_str!(newname) }) {
            Ok(entry_param) => unsafe {
                let _ret = fuse_reply_entry(req, entry_param.borrow());
            },
            Err(e) => unsafe {
                fuse_reply_err(req, e);
            },
        }
    }
    fn open<T: FileSystem>(req: *mut FuseReq, ino: u64, fi: *mut FuseFileInfo) {
        let file_system = filesystem!(req);
        let ctx = ctx!(req);
        match file_system.open(ctx, ino, unsafe { fi.as_mut().unwrap() }) {
            Ok(fi) => unsafe {
                let _ret = fuse_reply_open(req, fi.borrow());
            },
            Err(e) => unsafe {
                fuse_reply_err(req, e);
            },
        }
    }
    fn read<T: FileSystem>(
        req: *mut FuseReq,
        ino: u64,
        size: size_t,
        off: off_t,
        fi: *mut FuseFileInfo,
    ) {
        let file_system = filesystem!(req);
        let ctx = ctx!(req);
        match file_system.read(ctx, ino, size, off, unsafe { fi.as_mut().unwrap() }) {
            Ok(message) => unsafe {
                let buf = CString::new(message).unwrap();
                let _ret = fuse_reply_buf(req, buf.as_ptr(), message.len());
            },
            Err(e) => unsafe {
                fuse_reply_err(req, e);
            },
        }
    }
    fn write<T: FileSystem>(
        req: *mut FuseReq,
        ino: u64,
        buf: *const c_char,
        size: size_t,
        off: off_t,
        fi: *mut FuseFileInfo,
    ) {
        let file_system = filesystem!(req);
        let ctx = ctx!(req);
        match file_system.write(ctx, ino, unsafe { to_str!(buf) }, size, off, unsafe {
            fi.as_mut().unwrap()
        }) {
            Ok(count) => unsafe {
                let _ret = fuse_reply_write(req, count);
            },
            Err(e) => unsafe {
                fuse_reply_err(req, e);
            },
        }
    }
    fn flush<T: FileSystem>(req: *mut FuseReq, ino: u64, fi: *mut FuseFileInfo) {
        let file_system = filesystem!(req);
        let ctx = ctx!(req);
        match file_system.flush(ctx, ino, unsafe { fi.as_mut().unwrap() }) {
            Ok(..) => {}
            Err(e) => unsafe {
                fuse_reply_err(req, e);
            },
        }
    }
    fn release<T: FileSystem>(req: *mut FuseReq, ino: u64, fi: *mut FuseFileInfo) {
        let file_system = filesystem!(req);
        let ctx = ctx!(req);
        match file_system.release(ctx, ino, unsafe { fi.as_mut().unwrap() }) {
            Ok(..) => {}
            Err(e) => unsafe {
                fuse_reply_err(req, e);
            },
        }
    }
    fn fsync<T: FileSystem>(req: *mut FuseReq, ino: u64, datasync: c_int, fi: *mut FuseFileInfo) {
        let file_system = filesystem!(req);
        let ctx = ctx!(req);
        match file_system.fsync(ctx, ino, datasync, unsafe { fi.as_mut().unwrap() }) {
            Ok(..) => {}
            Err(e) => unsafe {
                fuse_reply_err(req, e);
            },
        }
    }
    fn opendir<T: FileSystem>(req: *mut FuseReq, ino: u64, fi: *mut FuseFileInfo) {
        let file_system = filesystem!(req);
        let ctx = ctx!(req);
        match file_system.opendir(ctx, ino, unsafe { fi.as_mut().unwrap() }) {
            Ok(fi) => unsafe {
                let _ret = fuse_reply_open(req, fi.borrow());
            },
            Err(e) => unsafe {
                fuse_reply_err(req, e);
            },
        }
    }
    fn readdir<T: FileSystem>(
        req: *mut FuseReq,
        ino: u64,
        size: size_t,
        off: off_t,
        fi: *mut FuseFileInfo,
    ) {
        let file_system = filesystem!(req);
        let ctx = ctx!(req);
        match file_system.readdir(ctx, ino, size, off, unsafe { fi.as_mut().unwrap() }) {
            Ok(dirs) => unsafe {
                let mut buf = Vec::<u8>::new();
                let mut buf_size: usize = 0;
                for dir in dirs {
                    let name_buf = CString::new(dir.name.as_str()).unwrap();
                    let tmp_buf_size =
                        fuse_add_direntry(req, null_mut(), 0, name_buf.as_ptr(), null(), 0);
                    buf_size += tmp_buf_size;
                    let mut tmp_buf = Vec::<u8>::with_capacity(tmp_buf_size);
                    let ret = fuse_add_direntry(
                        req,
                        tmp_buf.as_mut_ptr() as *mut c_char,
                        tmp_buf_size,
                        name_buf.as_ptr(),
                        dir.attr().convert().borrow(),
                        buf_size as i64,
                    );
                    assert_eq!(tmp_buf_size, ret);
                    tmp_buf.set_len(ret);
                    buf.extend(tmp_buf);
                }
                if off == 0 {
                    let _ret =
                        fuse_reply_buf(req, buf.as_ptr() as *const c_char, min(buf.len(), size));
                } else if (off as usize) < buf_size {
                    let out: Vec<_> = buf.drain((off as usize)..).collect();
                    let _ret =
                        fuse_reply_buf(req, out.as_ptr() as *const c_char, min(out.len(), size));
                } else {
                    let _ret = fuse_reply_buf(req, null(), 0);
                }
            },
            Err(e) => unsafe {
                fuse_reply_err(req, e);
            },
        }
    }
    fn releasedir<T: FileSystem>(req: *mut FuseReq, ino: u64, fi: *mut FuseFileInfo) {
        let file_system = filesystem!(req);
        let ctx = ctx!(req);
        match file_system.releasedir(ctx, ino, unsafe { fi.as_mut().unwrap() }) {
            Ok(..) => {}
            Err(e) => unsafe {
                fuse_reply_err(req, e);
            },
        }
    }
    fn fsyncdir<T: FileSystem>(
        req: *mut FuseReq,
        ino: u64,
        datasync: c_int,
        fi: *mut FuseFileInfo,
    ) {
        let file_system = filesystem!(req);
        let ctx = ctx!(req);
        match file_system.fsyncdir(ctx, ino, datasync, unsafe { fi.as_mut().unwrap() }) {
            Ok(..) => {}
            Err(e) => unsafe {
                fuse_reply_err(req, e);
            },
        }
    }
    fn statfs<T: FileSystem>(req: *mut FuseReq, ino: u64) {
        let file_system = filesystem!(req);
        let ctx = ctx!(req);
        match file_system.statfs(ctx, ino) {
            Ok(stbuf) => unsafe {
                let _ret = fuse_reply_statfs(req, stbuf.convert().borrow());
            },
            Err(e) => unsafe {
                fuse_reply_err(req, e);
            },
        }
    }
    fn setxattr<T: FileSystem>(
        req: *mut FuseReq,
        ino: u64,
        name: *const c_char,
        value: *const c_char,
        size: size_t,
        flags: c_int,
    ) {
        let file_system = filesystem!(req);
        let ctx = ctx!(req);
        match file_system.setxattr(
            ctx,
            ino,
            unsafe { to_str!(name) },
            unsafe { to_str!(value) },
            size,
            flags,
        ) {
            Ok(..) => {}
            Err(e) => unsafe {
                fuse_reply_err(req, e);
            },
        }
    }
    fn getxattr<T: FileSystem>(req: *mut FuseReq, ino: u64, name: *const c_char, size: size_t) {
        let file_system = filesystem!(req);
        let ctx = ctx!(req);
        match file_system.getxattr(ctx, ino, unsafe { to_str!(name) }, size) {
            Ok(message) => unsafe {
                if size == 0 {
                    let _ret = fuse_reply_xattr(req, size);
                } else {
                    let buf = CString::new(message).unwrap();
                    let _ret = fuse_reply_buf(req, buf.as_ptr(), message.len());
                }
            },
            Err(e) => unsafe {
                fuse_reply_err(req, e);
            },
        }
    }
    fn listxattr<T: FileSystem>(req: *mut FuseReq, ino: u64, size: size_t) {
        let file_system = filesystem!(req);
        let ctx = ctx!(req);
        match file_system.listxattr(ctx, ino, size) {
            Ok(message) => unsafe {
                if size == 0 {
                    let _ret = fuse_reply_xattr(req, size);
                } else {
                    let buf = CString::new(message).unwrap();
                    let _ret = fuse_reply_buf(req, buf.as_ptr(), message.len());
                }
            },
            Err(e) => unsafe {
                fuse_reply_err(req, e);
            },
        }
    }
    fn removexattr<T: FileSystem>(req: *mut FuseReq, ino: u64, name: *const c_char) {
        let file_system = filesystem!(req);
        let ctx = ctx!(req);
        match file_system.removexattr(ctx, ino, unsafe { to_str!(name) }) {
            Ok(..) => {}
            Err(e) => unsafe {
                fuse_reply_err(req, e);
            },
        }
    }
    fn access<T: FileSystem>(req: *mut FuseReq, ino: u64, mask: c_int) {
        let file_system = filesystem!(req);
        let ctx = ctx!(req);
        match file_system.access(ctx, ino, mask) {
            Ok(..) => {}
            Err(e) => unsafe {
                fuse_reply_err(req, e);
            },
        }
    }
    fn create<T: FileSystem>(
        req: *mut FuseReq,
        parent: u64,
        name: *const c_char,
        mode: mode_t,
        fi: *mut FuseFileInfo,
    ) {
        let file_system = filesystem!(req);
        let ctx = ctx!(req);
        match file_system.create(ctx, parent, unsafe { to_str!(name) }, mode, unsafe {
            fi.as_mut().unwrap()
        }) {
            Ok(entry_param) => unsafe {
                let _ret = fuse_reply_create(req, entry_param.borrow(), fi.as_ref().unwrap());
            },
            Err(e) => unsafe {
                fuse_reply_err(req, e);
            },
        }
    }
    fn getlk<T: FileSystem>(req: *mut FuseReq, ino: u64, fi: *mut FuseFileInfo, lock: *mut flock) {
        let file_system = filesystem!(req);
        let ctx = ctx!(req);
        match file_system.getlk(
            ctx,
            ino,
            unsafe { fi.as_mut().unwrap() },
            &mut FuseLock::new(unsafe { lock.as_ref().unwrap() }),
        ) {
            Ok(lock) => unsafe {
                let _ret = fuse_reply_lock(req, lock.convert().borrow());
            },
            Err(e) => unsafe {
                fuse_reply_err(req, e);
            },
        }
    }
    fn setlk<T: FileSystem>(
        req: *mut FuseReq,
        ino: u64,
        fi: *mut FuseFileInfo,
        lock: *mut flock,
        sleep: c_int,
    ) {
        let file_system = filesystem!(req);
        let ctx = ctx!(req);
        match file_system.setlk(
            ctx,
            ino,
            unsafe { fi.as_mut().unwrap() },
            &mut FuseLock::new(unsafe { lock.as_ref().unwrap() }),
            sleep,
        ) {
            Ok(..) => {}
            Err(e) => unsafe {
                fuse_reply_err(req, e);
            },
        }
    }
    fn bmap<T: FileSystem>(req: *mut FuseReq, ino: u64, blocksize: size_t, idx: u64) {
        let file_system = filesystem!(req);
        let ctx = ctx!(req);
        match file_system.bmap(ctx, ino, blocksize, idx) {
            Ok(idx) => unsafe {
                let _ret = fuse_reply_bmap(req, idx);
            },
            Err(e) => unsafe {
                fuse_reply_err(req, e);
            },
        }
    }
    fn poll<T: FileSystem>(
        req: *mut FuseReq,
        ino: u64,
        fi: *mut FuseFileInfo,
        ph: *mut FusePollhandle,
    ) {
        let file_system = filesystem!(req);
        let ctx = ctx!(req);
        match file_system.poll(ctx, ino, unsafe { fi.as_mut().unwrap() }, unsafe {
            ph.as_mut().unwrap()
        }) {
            Ok(revents) => unsafe {
                let _ret = fuse_reply_poll(req, revents);
            },
            Err(e) => unsafe {
                fuse_reply_err(req, e);
            },
        }
    }
    fn write_buf<T: FileSystem>(
        req: *mut FuseReq,
        ino: u64,
        bufv: *mut FuseBufvec,
        off: off_t,
        fi: *mut FuseFileInfo,
    ) {
        let file_system = filesystem!(req);
        let ctx = ctx!(req);
        match file_system.write_buf(ctx, ino, unsafe { bufv.as_mut().unwrap() }, off, unsafe {
            fi.as_mut().unwrap()
        }) {
            Ok(count) => unsafe {
                let _ret = fuse_reply_write(req, count);
            },
            Err(e) => unsafe {
                fuse_reply_err(req, e);
            },
        }
    }
    fn forget_multi<T: FileSystem>(req: *mut FuseReq, count: size_t, forgets: *mut FuseForgetData) {
        let file_system = filesystem!(req);
        let ctx = ctx!(req);
        file_system.forget_multi(ctx, count, unsafe { forgets.as_mut().unwrap() });
        unsafe {
            fuse_reply_none(req);
        }
    }
    fn flock<T: FileSystem>(req: *mut FuseReq, ino: u64, fi: *mut FuseFileInfo, op: c_int) {
        let file_system = filesystem!(req);
        let ctx = ctx!(req);
        match file_system.flock(ctx, ino, unsafe { fi.as_mut().unwrap() }, op) {
            Ok(..) => {}
            Err(e) => unsafe {
                fuse_reply_err(req, e);
            },
        }
    }
    fn fallocate<T: FileSystem>(
        req: *mut FuseReq,
        ino: u64,
        mode: c_int,
        offset: off_t,
        length: off_t,
        fi: *mut FuseFileInfo,
    ) {
        let file_system = filesystem!(req);
        let ctx = ctx!(req);
        match file_system.fallocate(ctx, ino, mode, offset, length, unsafe {
            fi.as_mut().unwrap()
        }) {
            Ok(..) => {}
            Err(e) => unsafe {
                fuse_reply_err(req, e);
            },
        }
    }
    fn readdirplus<T: FileSystem>(
        req: *mut FuseReq,
        ino: u64,
        size: size_t,
        off: off_t,
        fi: *mut FuseFileInfo,
    ) {
        let file_system = filesystem!(req);
        let ctx = ctx!(req);
        match file_system.readdirplus(ctx, ino, size, off, unsafe { fi.as_mut().unwrap() }) {
            Ok(message) => unsafe {
                let buf = CString::new(message).unwrap();
                let _ret = fuse_reply_buf(req, buf.as_ptr(), message.len());
            },
            Err(e) => unsafe {
                fuse_reply_err(req, e);
            },
        }
    }
    fn copy_file_range<T: FileSystem>(
        req: *mut FuseReq,
        ino_in: u64,
        off_in: off_t,
        fi_in: *mut FuseFileInfo,
        ino_out: u64,
        off_out: off_t,
        fi_out: *mut FuseFileInfo,
        len: size_t,
        flags: c_int,
    ) {
        let file_system = filesystem!(req);
        let ctx = ctx!(req);
        match file_system.copy_file_range(
            ctx,
            ino_in,
            off_in,
            unsafe { fi_in.as_mut().unwrap() },
            ino_out,
            off_out,
            unsafe { fi_out.as_mut().unwrap() },
            len,
            flags,
        ) {
            Ok(count) => unsafe {
                let _ret = fuse_reply_write(req, count);
            },
            Err(e) => unsafe {
                fuse_reply_err(req, e);
            },
        }
    }
    fn lseek<T: FileSystem>(
        req: *mut FuseReq,
        ino: u64,
        off: off_t,
        whence: c_int,
        fi: *mut FuseFileInfo,
    ) {
        let file_system = filesystem!(req);
        let ctx = ctx!(req);
        match file_system.lseek(ctx, ino, off, whence, unsafe { fi.as_mut().unwrap() }) {
            Ok(off) => unsafe {
                let _ret = fuse_reply_lseek(req, off);
            },
            Err(e) => unsafe {
                fuse_reply_err(req, e);
            },
        }
    }
}

pub fn fuse_loop<T: FileSystem>(mountpoint: &str, file_system: &mut T, ops: u64) {
    let arg0 = CString::new(env::args().nth(0).unwrap()).unwrap();
    let mountpoint = CString::new(mountpoint).unwrap();
    let c_argv: Vec<*const c_char> = vec![arg0.as_ptr()];
    let mut fuse_args = FuseArgs {
        argc: 1 as c_int,
        argv: c_argv.as_ptr(),
        allocated: 0 as c_int,
    };
    let op = FuseOps::fuse_low_level_ops::<T>(ops);

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

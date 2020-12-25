use std::borrow::{Borrow, BorrowMut};
use std::env;
use std::ffi::{CStr, CString};
use std::mem::size_of;

use libc::{c_char, c_int, c_void, dev_t, flock, mode_t, off_t, size_t, stat};

mod filesystem;
mod fuse;
mod utils;

pub use crate::filesystem::FileSystem;
use crate::fuse::{
    fuse_reply_attr, fuse_reply_bmap, fuse_reply_buf, fuse_reply_create, fuse_reply_entry,
    fuse_reply_err, fuse_reply_lock, fuse_reply_lseek, fuse_reply_none, fuse_reply_open,
    fuse_reply_poll, fuse_reply_readlink, fuse_reply_statfs, fuse_reply_write, fuse_reply_xattr,
    fuse_req_ctx, fuse_req_userdata, fuse_session_destroy, fuse_session_loop, fuse_session_mount,
    fuse_session_new, fuse_session_unmount, FuseArgs, FuseConnInfo, FuseLowLevelOps, FuseReq,
};
pub use crate::fuse::{
    FuseAttr, FuseBufvec, FuseCtx, FuseEntryParam, FuseFileInfo, FuseForgetData, FuseLock,
    FusePollhandle, FuseStatvfs,
};

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
            // retrieve_reply: FuseLowLevelOps::retrieve_reply::<T>,
            forget_multi: FuseLowLevelOps::forget_multi::<T>,
            flock: FuseLowLevelOps::flock::<T>,
            fallocate: FuseLowLevelOps::fallocate::<T>,
            readdirplus: FuseLowLevelOps::readdirplus::<T>,
            copy_file_range: FuseLowLevelOps::copy_file_range::<T>,
            lseek: FuseLowLevelOps::lseek::<T>,
        }
    }
    pub fn init<T: FileSystem>(userdata: *mut c_void, _conn: *mut FuseConnInfo) {
        let file_system = unsafe { (userdata as *mut T).as_mut().unwrap() };
        let _ = file_system.init();
    }
    pub fn destroy<T: FileSystem>(userdata: *mut c_void) {
        let file_system = unsafe { (userdata as *mut T).as_mut().unwrap() };
        let _ = file_system.destroy();
    }
    pub fn lookup<T: FileSystem>(req: *mut FuseReq, parent: u64, name: *const c_char) {
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
    pub fn forget<T: FileSystem>(req: *mut FuseReq, ino: u64, nlookup: u64) {
        let file_system = filesystem!(req);
        let ctx = ctx!(req);
        file_system.forget(ctx, ino, nlookup);
        unsafe {
            fuse_reply_none(req);
        }
    }
    pub fn getattr<T: FileSystem>(req: *mut FuseReq, ino: u64, fi: *mut FuseFileInfo) {
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
    pub fn setattr<T: FileSystem>(
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
    pub fn readlink<T: FileSystem>(req: *mut FuseReq, ino: u64) {
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
    pub fn mknod<T: FileSystem>(
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
    pub fn mkdir<T: FileSystem>(req: *mut FuseReq, parent: u64, name: *const c_char, mode: mode_t) {
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
    pub fn unlink<T: FileSystem>(req: *mut FuseReq, parent: u64, name: *const c_char) {
        let file_system = filesystem!(req);
        let ctx = ctx!(req);
        match file_system.unlink(ctx, parent, unsafe { to_str!(name) }) {
            Ok(..) => {}
            Err(e) => unsafe {
                fuse_reply_err(req, e);
            },
        }
    }
    pub fn rmdir<T: FileSystem>(req: *mut FuseReq, parent: u64, name: *const c_char) {
        let file_system = filesystem!(req);
        let ctx = ctx!(req);
        match file_system.rmdir(ctx, parent, unsafe { to_str!(name) }) {
            Ok(..) => {}
            Err(e) => unsafe {
                fuse_reply_err(req, e);
            },
        }
    }
    pub fn symlink<T: FileSystem>(
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
    pub fn rename<T: FileSystem>(
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
    pub fn link<T: FileSystem>(
        req: *mut FuseReq,
        ino: u64,
        newparent: u64,
        newname: *const c_char,
    ) {
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
    pub fn open<T: FileSystem>(req: *mut FuseReq, ino: u64, fi: *mut FuseFileInfo) {
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
    pub fn read<T: FileSystem>(
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
    pub fn write<T: FileSystem>(
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
    pub fn flush<T: FileSystem>(req: *mut FuseReq, ino: u64, fi: *mut FuseFileInfo) {
        let file_system = filesystem!(req);
        let ctx = ctx!(req);
        match file_system.flush(ctx, ino, unsafe { fi.as_mut().unwrap() }) {
            Ok(..) => {}
            Err(e) => unsafe {
                fuse_reply_err(req, e);
            },
        }
    }
    pub fn release<T: FileSystem>(req: *mut FuseReq, ino: u64, fi: *mut FuseFileInfo) {
        let file_system = filesystem!(req);
        let ctx = ctx!(req);
        match file_system.release(ctx, ino, unsafe { fi.as_mut().unwrap() }) {
            Ok(..) => {}
            Err(e) => unsafe {
                fuse_reply_err(req, e);
            },
        }
    }
    pub fn fsync<T: FileSystem>(
        req: *mut FuseReq,
        ino: u64,
        datasync: c_int,
        fi: *mut FuseFileInfo,
    ) {
        let file_system = filesystem!(req);
        let ctx = ctx!(req);
        match file_system.fsync(ctx, ino, datasync, unsafe { fi.as_mut().unwrap() }) {
            Ok(..) => {}
            Err(e) => unsafe {
                fuse_reply_err(req, e);
            },
        }
    }
    pub fn opendir<T: FileSystem>(req: *mut FuseReq, ino: u64, fi: *mut FuseFileInfo) {
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
    pub fn readdir<T: FileSystem>(
        req: *mut FuseReq,
        ino: u64,
        size: size_t,
        off: off_t,
        fi: *mut FuseFileInfo,
    ) {
        let file_system = filesystem!(req);
        let ctx = ctx!(req);
        match file_system.readdir(ctx, ino, size, off, unsafe { fi.as_mut().unwrap() }) {
            Ok(message) => unsafe {
                let buf = CString::new(message).unwrap();
                let _ret = fuse_reply_buf(req, buf.as_ptr(), message.len());
            },
            Err(e) => unsafe {
                fuse_reply_err(req, e);
            },
        }
    }
    pub fn releasedir<T: FileSystem>(req: *mut FuseReq, ino: u64, fi: *mut FuseFileInfo) {
        let file_system = filesystem!(req);
        let ctx = ctx!(req);
        match file_system.releasedir(ctx, ino, unsafe { fi.as_mut().unwrap() }) {
            Ok(..) => {}
            Err(e) => unsafe {
                fuse_reply_err(req, e);
            },
        }
    }
    pub fn fsyncdir<T: FileSystem>(
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
    pub fn statfs<T: FileSystem>(req: *mut FuseReq, ino: u64) {
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
    pub fn setxattr<T: FileSystem>(
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
    pub fn getxattr<T: FileSystem>(req: *mut FuseReq, ino: u64, name: *const c_char, size: size_t) {
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
    pub fn listxattr<T: FileSystem>(req: *mut FuseReq, ino: u64, size: size_t) {
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
    pub fn removexattr<T: FileSystem>(req: *mut FuseReq, ino: u64, name: *const c_char) {
        let file_system = filesystem!(req);
        let ctx = ctx!(req);
        match file_system.removexattr(ctx, ino, unsafe { to_str!(name) }) {
            Ok(..) => {}
            Err(e) => unsafe {
                fuse_reply_err(req, e);
            },
        }
    }
    pub fn access<T: FileSystem>(req: *mut FuseReq, ino: u64, mask: c_int) {
        let file_system = filesystem!(req);
        let ctx = ctx!(req);
        match file_system.access(ctx, ino, mask) {
            Ok(..) => {}
            Err(e) => unsafe {
                fuse_reply_err(req, e);
            },
        }
    }
    pub fn create<T: FileSystem>(
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
    pub fn getlk<T: FileSystem>(
        req: *mut FuseReq,
        ino: u64,
        fi: *mut FuseFileInfo,
        lock: *mut flock,
    ) {
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
    pub fn setlk<T: FileSystem>(
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
    pub fn bmap<T: FileSystem>(req: *mut FuseReq, ino: u64, blocksize: size_t, idx: u64) {
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
    pub fn poll<T: FileSystem>(
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
    pub fn write_buf<T: FileSystem>(
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
    // pub fn retrieve_reply<T: FileSystem>(
    //     req: *mut FuseReq,
    //     _cookie: *mut c_void,
    //     _ino: u64,
    //     _off: off_t,
    //     _bufv: *mut FuseBufvec,
    // ) {
    //     let file_system = filesystem!(req);
    //     let ctx = ctx!(req);
    //     file_system.retrieve_reply(ctx, ...);
    //     fuse_reply_none(req);
    // }
    pub fn forget_multi<T: FileSystem>(
        req: *mut FuseReq,
        count: size_t,
        forgets: *mut FuseForgetData,
    ) {
        let file_system = filesystem!(req);
        let ctx = ctx!(req);
        file_system.forget_multi(ctx, count, unsafe { forgets.as_mut().unwrap() });
        unsafe {
            fuse_reply_none(req);
        }
    }
    pub fn flock<T: FileSystem>(req: *mut FuseReq, ino: u64, fi: *mut FuseFileInfo, op: c_int) {
        let file_system = filesystem!(req);
        let ctx = ctx!(req);
        match file_system.flock(ctx, ino, unsafe { fi.as_mut().unwrap() }, op) {
            Ok(..) => {}
            Err(e) => unsafe {
                fuse_reply_err(req, e);
            },
        }
    }
    pub fn fallocate<T: FileSystem>(
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
    pub fn readdirplus<T: FileSystem>(
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
    pub fn copy_file_range<T: FileSystem>(
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
    pub fn lseek<T: FileSystem>(
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

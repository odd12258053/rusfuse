use std::borrow::{Borrow, BorrowMut};
use std::env;
use std::ffi::CString;
use std::mem;
use std::mem::size_of;
use std::slice;

use libc::{
    self,
    c_char, c_int, c_uint, c_void, dev_t, flock, gid_t, iovec, mode_t, off_t, pid_t, size_t, stat,
    statvfs, uid_t, ENOSYS,
    ino_t,
    nlink_t,
    blksize_t,
    blkcnt_t,
    time_t,
};

#[repr(C)]
pub struct FuseArgs {
    pub argc: c_int,
    pub argv: *const *const c_char,
    pub allocated: c_int,
}

#[repr(C)]
pub struct FuseConnInfo {
    proto_major: c_uint,
    proto_minor: c_uint,
    max_write: c_uint,
    max_read: c_uint,
    max_readahead: c_uint,
    capable: c_uint,
    want: c_uint,
    max_background: c_uint,
    congestion_threshold: c_uint,
    time_gran: c_uint,
    reserved: [c_uint; 22],
}

#[repr(C)]
pub struct FuseReq;

#[repr(C)]
pub struct FuseSession;

pub enum FuseBufFlags {
    FuseBufIsFd = 1 << 1,
    FuseBufFdSeek = 1 << 2,
    FuseBufFdRetry = 1 << 3,
}

#[repr(C)]
pub struct FuseBuf {
    size: size_t,
    flags: FuseBufFlags,
    mem: *mut c_void,
    fd: c_int,
    pop: c_uint,
}

#[repr(C)]
pub struct FuseFileInfo {
    flags: i16,
    writepage: u16,
    direct_io: u16,
    keep_cache: u16,
    flush: u16,
    nonseekable: u16,
    flock_release: u16,
    cache_readdir: u16,
    padding: u16,
    padding2: u16,
    fh: u64,
    lock_owner: u64,
    poll_events: u32,
}

#[repr(C)]
pub enum FuseReaddirFlags {
    FuseReaddirPlus = 1 << 0,
}

#[repr(C)]
pub enum FuseFillDirFlags {
    FuseFillDirPlus = 1 << 1,
}

#[repr(C)]
pub struct FuseConfig {
    set_gid: i16,
    gid: u16,
    set_uid: i16,
    uid: u16,
    set_mode: i16,
    umask: u16,
    entry_timeout: f64,
    negative_timeout: f64,
    attr_timeout: f64,
    intr: i16,
    intr_signal: i16,
    remember: i16,
    hard_remove: i16,
    use_ino: i16,
    readdir_ino: i16,
    direct_io: i16,
    kernel_cache: i16,
    auto_cache: i16,
    ac_attr_timeout_set: i16,
    ac_attr_timeout: f64,
    nullpath_ok: i16,
    show_help: i16,
    modules: *mut c_char,
    debug: i16,
}

#[repr(C)]
pub struct FusePollhandle;

#[repr(C)]
pub struct FuseForgetData {
    ino: u64,
    nlookup: u64,
}

#[repr(C)]
pub struct FuseCtx {
    uid: uid_t,
    gid: gid_t,
    pid: pid_t,
    umask: mode_t,
}

#[repr(C)]
pub struct FuseBufvec {
    count: size_t,
    idx: size_t,
    off: size_t,
    buf: [FuseBuf; 1],
}

#[repr(C)]
pub struct FuseEntryParam {
    ino: u64,
    generation: u64,
    attr: stat,
    attr_timeout: f64,
    entry_timeout: f64,
}

#[repr(C)]
pub struct FuseLowLevelOps {
    // void (*init) (void *userdata, struct fuse_conn_info *conn);
    init: fn(*mut c_void, *mut FuseConnInfo),

    // void (*destroy) (void *userdata);
    destroy: fn(*mut c_void),

    // void (*lookup) (fuse_req_t req, fuse_ino_t parent, const char *name);
    lookup: fn(*mut FuseReq, u64, *const c_char),

    // void (*forget) (fuse_req_t req, fuse_ino_t ino, uint64_t nlookup);
    forget: fn(*mut FuseReq, u64, u64),

    // void (*getattr) (fuse_req_t req, fuse_ino_t ino, struct fuse_file_info *fi);
    getattr: fn(*mut FuseReq, u64, *mut FuseFileInfo),

    // void (*setattr) (fuse_req_t req, fuse_ino_t ino, struct stat *attr, int to_set, struct fuse_file_info *fi);
    setattr: fn(*mut FuseReq, u64, *mut stat, i16, *mut FuseFileInfo),

    // void (*readlink) (fuse_req_t req, fuse_ino_t ino);
    readlink: fn(*mut FuseReq, u64),

    // void (*mknod) (fuse_req_t req, fuse_ino_t parent, const char *name, mode_t mode, dev_t rdev);
    mknod: fn(*mut FuseReq, u64, *const c_char, mode_t, dev_t),

    // void (*mkdir) (fuse_req_t req, fuse_ino_t parent, const char *name, mode_t mode);
    mkdir: fn(*mut FuseReq, u64, *const c_char, mode_t),

    // void (*unlink) (fuse_req_t req, fuse_ino_t parent, const char *name);
    unlink: fn(*mut FuseReq, u64, *const c_char),

    // void (*rmdir) (fuse_req_t req, fuse_ino_t parent, const char *name);
    rmdir: fn(*mut FuseReq, u64, *const c_char),

    // void (*symlink) (fuse_req_t req, const char *link, fuse_ino_t parent, const char *name);
    symlink: fn(*mut FuseReq, *const c_char, u64, *const c_char),

    // void (*rename) (fuse_req_t req, fuse_ino_t parent, const char *name, fuse_ino_t newparent, const char *newname, unsigned int flags);
    rename: fn(*mut FuseReq, u64, *const c_char, u64, *const c_char, u16),

    // void (*link) (fuse_req_t req, fuse_ino_t ino, fuse_ino_t newparent, const char *newname);
    link: fn(*mut FuseReq, u64, u64, *const c_char),

    // void (*open) (fuse_req_t req, fuse_ino_t ino, struct fuse_file_info *fi);
    open: fn(*mut FuseReq, u64, *mut FuseFileInfo),

    // void (*read) (fuse_req_t req, fuse_ino_t ino, size_t size, off_t off, struct fuse_file_info *fi);
    read: fn(*mut FuseReq, u64, size_t, off_t, *mut FuseFileInfo),

    // void (*write) (fuse_req_t req, fuse_ino_t ino, const char *buf, size_t size, off_t off, struct fuse_file_info *fi);
    write: fn(*mut FuseReq, u64, *const c_char, size_t, off_t, *mut FuseFileInfo),

    // void (*flush) (fuse_req_t req, fuse_ino_t ino, struct fuse_file_info *fi);
    flush: fn(*mut FuseReq, u64, *mut FuseFileInfo),

    // void (*release) (fuse_req_t req, fuse_ino_t ino, struct fuse_file_info *fi);
    release: fn(*mut FuseReq, u64, *mut FuseFileInfo),

    // void (*fsync) (fuse_req_t req, fuse_ino_t ino, int datasync, struct fuse_file_info *fi);
    fsync: fn(*mut FuseReq, u64, c_int, *mut FuseFileInfo),

    // void (*opendir) (fuse_req_t req, fuse_ino_t ino, struct fuse_file_info *fi);
    opendir: fn(*mut FuseReq, u64, *mut FuseFileInfo),

    // void (*readdir) (fuse_req_t req, fuse_ino_t ino, size_t size, off_t off, struct fuse_file_info *fi);
    readdir: fn(*mut FuseReq, u64, size_t, off_t, *mut FuseFileInfo),

    // void (*releasedir) (fuse_req_t req, fuse_ino_t ino, struct fuse_file_info *fi);
    releasedir: fn(*mut FuseReq, u64, *mut FuseFileInfo),

    // void (*fsyncdir) (fuse_req_t req, fuse_ino_t ino, int datasync, struct fuse_file_info *fi);
    fsyncdir: fn(*mut FuseReq, u64, c_int, *mut FuseFileInfo),

    // void (*statfs) (fuse_req_t req, fuse_ino_t ino);
    statfs: fn(*mut FuseReq, u64),

    // void (*setxattr) (fuse_req_t req, fuse_ino_t ino, const char *name, const char *value, size_t size, int flags);
    setxattr: fn(*mut FuseReq, u64, *const c_char, *const c_char, size_t, c_int),

    // void (*getxattr) (fuse_req_t req, fuse_ino_t ino, const char *name, size_t size);
    getxattr: fn(*mut FuseReq, u64, *const c_char, size_t),

    // void (*listxattr) (fuse_req_t req, fuse_ino_t ino, size_t size);
    listxattr: fn(*mut FuseReq, u64, size_t),

    // void (*removexattr) (fuse_req_t req, fuse_ino_t ino, const char *name);
    removexattr: fn(*mut FuseReq, u64, *const c_char),

    // void (*access) (fuse_req_t req, fuse_ino_t ino, int mask);
    access: fn(*mut FuseReq, u64, c_int),

    // void (*create) (fuse_req_t req, fuse_ino_t parent, const char *name, mode_t mode, struct fuse_file_info *fi);
    create: fn(*mut FuseReq, u64, *const c_char, mode_t, *mut FuseFileInfo),

    // void (*getlk) (fuse_req_t req, fuse_ino_t ino, struct fuse_file_info *fi, struct flock *lock);
    getlk: fn(*mut FuseReq, u64, *mut FuseFileInfo, *mut flock),

    // void (*setlk) (fuse_req_t req, fuse_ino_t ino, struct fuse_file_info *fi, struct flock *lock, int sleep);
    setlk: fn(*mut FuseReq, u64, *mut FuseFileInfo, *mut flock, c_int),

    // void (*bmap) (fuse_req_t req, fuse_ino_t ino, size_t blocksize, uint64_t idx);
    bmap: fn(*mut FuseReq, u64, size_t, u64),

    // TODO: Ioctl

    // void (*poll) (fuse_req_t req, fuse_ino_t ino, struct fuse_file_info *fi, struct fuse_pollhandle *ph);
    poll: fn(*mut FuseReq, u64, *mut FuseFileInfo, *mut FusePollhandle),

    // void (*write_buf) (fuse_req_t req, fuse_ino_t ino, struct fuse_bufvec *bufv, off_t off, struct fuse_file_info *fi);
    write_buf: fn(*mut FuseReq, u64, *mut FuseBufvec, off_t, *mut FuseFileInfo),

    // void (*retrieve_reply) (fuse_req_t req, void *cookie, fuse_ino_t ino, off_t offset, struct fuse_bufvec *bufv);
    retrieve_reply: fn(*mut FuseReq, *mut c_void, u64, off_t, *mut FuseBufvec),

    // void (*forget_multi) (fuse_req_t req, size_t count, struct fuse_forget_data *forgets);
    forget_multi: fn(*mut FuseReq, size_t, *mut FuseForgetData),

    // void (*flock) (fuse_req_t req, fuse_ino_t ino, struct fuse_file_info *fi, int op);
    flock: fn(*mut FuseReq, u64, *mut FuseFileInfo, c_int),

    // void (*fallocate) (fuse_req_t req, fuse_ino_t ino, int mode, off_t offset, off_t length, struct fuse_file_info *fi);
    fallocate: fn(*mut FuseReq, u64, c_int, off_t, off_t, *mut FuseFileInfo),

    // void (*readdirplus) (fuse_req_t req, fuse_ino_t ino, size_t size, off_t off, struct fuse_file_info *fi);
    readdirplus: fn(*mut FuseReq, u64, size_t, off_t, *mut FuseFileInfo),

    // void (*copy_file_range) (fuse_req_t req, fuse_ino_t ino_in,
    //              off_t off_in, struct fuse_file_info *fi_in,
    //              fuse_ino_t ino_out, off_t off_out,
    //              struct fuse_file_info *fi_out, size_t len,
    //              int flags);
    copy_file_range: fn(
        *mut FuseReq,
        u64,
        off_t,
        *mut FuseFileInfo,
        u64,
        off_t,
        *mut FuseFileInfo,
        size_t,
        c_int,
    ),

    // void (*lseek) (fuse_req_t req, fuse_ino_t ino, off_t off, int whence, struct fuse_file_info *fi);
    lseek: fn(*mut FuseReq, u64, off_t, c_int, *mut FuseFileInfo),
}


#[repr(C)]
pub struct FuseAttr {
    pub ino: u64,
    pub size: u64,
    pub blocks: u64,
    pub atime: u64,
    pub atimensec: u32,
    pub mtime: u64,
    pub mtimensec: u32,
    pub ctime: u64,
    pub ctimensec: u32,
    pub mode: u32,
    pub nlink: u32,
    pub uid: u32,
    pub gid: u32,
    pub rdev: u32,
    pub blksize: u32,
    pub padding: u32,
}

impl FuseAttr {
    pub fn new(attr: &stat) -> FuseAttr {
        unsafe {
            FuseAttr {
                ino: attr.st_ino as u64,
                nlink: attr.st_nlink as u32,
                size: attr.st_size as u64,
                blocks: attr.st_blocks as u64,
                atime: attr.st_atime as u64,
                atimensec: attr.st_atime_nsec as u32,
                mtime: attr.st_mtime as u64,
                mtimensec: attr.st_mtime_nsec as u32,
                ctime: attr.st_ctime as u64,
                ctimensec: attr.st_ctime_nsec as u32,
                mode: attr.st_mode as u32,
                uid: attr.st_uid as u32,
                gid: attr.st_gid as u32,
                rdev: attr.st_rdev as u32,
                blksize: attr.st_blksize as u32,
                padding: 0,
            }
        }
    }
    pub fn convert(&self) -> stat {
        let mut sb = unsafe {mem::zeroed::<stat>()};
        sb.st_dev = 0 as dev_t;
        sb.st_ino = self.ino as ino_t;
        sb.st_nlink = self.nlink as nlink_t;
        sb.st_mode = self.mode as mode_t;
        sb.st_uid = self.uid as uid_t;
        sb.st_gid = self.gid as gid_t;
        sb.st_rdev = self.rdev as dev_t;
        sb.st_size = self.size as off_t;
        sb.st_blksize = self.blksize as blksize_t;
        sb.st_blocks = self.blocks as blkcnt_t;
        sb.st_atime = self.atime as time_t;
        sb.st_atime_nsec = self.atimensec as i64;
        sb.st_mtime = self.mtime as time_t;
        sb.st_mtime_nsec = self.mtimensec as i64;
        sb.st_ctime = self.ctime as time_t;
        sb.st_ctime_nsec = self.ctimensec as i64;
        sb
    }
}


#[link(name = "fuse3")]
extern "C" {
    pub fn fuse_session_new(
        args: *mut FuseArgs,
        op: *const FuseLowLevelOps,
        op_size: size_t,
        userdata: *mut c_void,
    ) -> *mut FuseSession;
    pub fn fuse_session_loop(fuse_session: *mut FuseSession) -> c_int;
    pub fn fuse_session_destroy(fuse_session: *mut FuseSession);
    pub fn fuse_session_mount(fuse_session: *mut FuseSession, mountpoint: *const c_char);
    pub fn fuse_session_unmount(fuse_session: *mut FuseSession);
    pub fn fuse_session_exit(fuse_session: *mut FuseSession);
    pub fn fuse_session_exited(fuse_session: *mut FuseSession) -> c_int;
    pub fn fuse_session_reset(fuse_session: *mut FuseSession);
    pub fn fuse_req_userdata(req: *mut FuseReq) -> *mut c_void;
    pub fn fuse_req_ctx(req: *mut FuseReq) -> *const FuseCtx;
    pub fn fuse_reply_open(req: *mut FuseReq, fi: *const FuseFileInfo) -> c_int;
    pub fn fuse_reply_write(req: *mut FuseReq, count: size_t) -> c_int;
    pub fn fuse_reply_err(req: *mut FuseReq, err: c_int) -> c_int;
    pub fn fuse_reply_none(req: *mut FuseReq);
    pub fn fuse_reply_entry(req: *mut FuseReq, e: *const FuseEntryParam) -> c_int;
    pub fn fuse_reply_create(
        req: *mut FuseReq,
        e: *const FuseEntryParam,
        fi: *const FuseFileInfo,
    ) -> c_int;
    pub fn fuse_reply_attr(req: *mut FuseReq, attr: *const stat, attr_timeout: f64) -> c_int;
    pub fn fuse_reply_readlink(req: *mut FuseReq, link: *const c_char) -> c_int;
    pub fn fuse_reply_buf(req: *mut FuseReq, buf: *const c_char, size: size_t) -> c_int;
    // pub fn fuse_reply_data(req: *mut FuseReq, bufv: *mut FuseBufvec, flags: FuseBufCopyFlags) -> c_int;
    pub fn fuse_reply_iov(req: *mut FuseReq, iov: *const iovec, count: c_int) -> c_int;
    pub fn fuse_reply_statfs(req: *mut FuseReq, stbuf: *const statvfs) -> c_int;
    pub fn fuse_reply_xattr(req: *mut FuseReq, count: size_t) -> c_int;
    pub fn fuse_reply_lock(req: *mut FuseReq, lock: *const flock) -> c_int;
    pub fn fuse_reply_bmap(req: *mut FuseReq, idx: u64) -> c_int;
}

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

pub fn to_bytes<T>(data: &T) -> &[u8] {
    let v = data as *const T as *const u8;
    let s = size_of::<T>();
    return unsafe { slice::from_raw_parts(v, s) };
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



#[cfg(test)]
mod tests {
    use crate::FuseAttr;
    use libc::{stat, S_IFREG, S_IRUSR, S_IWUSR, S_IRGRP, S_IWGRP, S_IROTH, S_IWOTH};
    use std::mem;
    use std::ffi::CString;
    use std::borrow::BorrowMut;
    #[test]
    fn fuse_attr_new() {
        let sb = unsafe {
            let mut sb: stat = unsafe {mem::zeroed::<stat>()};
            let ret = unsafe {
                stat(
                    CString::new("./tests/resource/a").unwrap().as_ptr(),
                    sb.borrow_mut()
                )
            };
            assert_eq!(ret, 0);
            sb
        };
        let attr = FuseAttr::new(&sb);
        assert_eq!(attr.size, 13);
        assert_eq!(attr.blocks, 8);
        assert_eq!(attr.mode, S_IFREG | S_IRUSR | S_IWUSR | S_IRGRP | S_IWGRP | S_IROTH);
        assert_eq!(attr.nlink, 1);
        assert_eq!(attr.rdev, 0);
        assert_eq!(attr.padding, 0);

        let sc = attr.convert();
        assert_eq!(sb.st_ino, sc.st_ino);
        assert_eq!(sb.st_nlink, sc.st_nlink);
        assert_eq!(sb.st_mode, sc.st_mode);
        assert_eq!(sb.st_uid, sc.st_uid);
        assert_eq!(sb.st_gid, sc.st_gid);
        assert_eq!(sb.st_rdev, sc.st_rdev);
        assert_eq!(sb.st_size, sc.st_size);
        assert_eq!(sb.st_blksize, sc.st_blksize);
        assert_eq!(sb.st_blocks, sc.st_blocks);
        assert_eq!(sb.st_atime, sc.st_atime);
        assert_eq!(sb.st_atime_nsec, sc.st_atime_nsec);
        assert_eq!(sb.st_mtime, sc.st_mtime);
        assert_eq!(sb.st_mtime_nsec, sc.st_mtime_nsec);
        assert_eq!(sb.st_ctime, sc.st_ctime);
        assert_eq!(sb.st_ctime_nsec, sc.st_ctime_nsec);
    }
}


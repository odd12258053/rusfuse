use std::mem;
use std::os::raw::{c_short, c_ulong};
use std::ptr::null_mut;

use libc::{
    blkcnt_t, blksize_t, c_char, c_int, c_uint, c_void, dev_t, flock, fsblkcnt_t, fsfilcnt_t,
    gid_t, ino_t, mode_t, nlink_t, off_t, pid_t, size_t, stat, statvfs, time_t, uid_t, S_IFBLK,
    S_IFCHR, S_IFDIR, S_IFIFO, S_IFLNK, S_IFREG, S_IFSOCK,
};

#[repr(C)]
#[derive(Debug)]
pub struct FuseArgs {
    pub argc: c_int,
    pub argv: *const *const c_char,
    pub allocated: c_int,
}

#[repr(C)]
#[derive(Debug)]
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

#[derive(Debug)]
pub enum FuseBufFlags {
    Default = 0,
    #[allow(dead_code)]
    FuseBufIsFd = 1 << 1,
    #[allow(dead_code)]
    FuseBufFdSeek = 1 << 2,
    #[allow(dead_code)]
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

impl FuseBuf {
    pub(crate) fn new() -> Self {
        Self {
            size: 0,
            flags: FuseBufFlags::Default,
            mem: null_mut(),
            fd: 0,
            pop: 0,
        }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct FuseFileInfo {
    pub flags: i16,
    pub writepage: u16,
    pub direct_io: u16,
    pub keep_cache: u16,
    pub flush: u16,
    pub nonseekable: u16,
    pub flock_release: u16,
    pub cache_readdir: u16,
    pub padding: u16,
    pub padding2: u16,
    pub fh: u64,
    pub lock_owner: u64,
    pub poll_events: u32,
}

#[repr(C)]
pub struct FusePollhandle;

#[repr(C)]
#[derive(Debug)]
pub struct FuseForgetData {
    pub ino: u64,
    pub nlookup: u64,
}

#[repr(C)]
#[derive(Debug)]
pub struct FuseCtx {
    pub uid: uid_t,
    pub gid: gid_t,
    pub pid: pid_t,
    pub umask: mode_t,
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
    pub ino: u64,
    pub generation: u64,
    pub attr: stat,
    pub attr_timeout: f64,
    pub entry_timeout: f64,
}

impl FuseEntryParam {
    pub fn new(attr: FuseAttr, generation: u64, attr_timeout: f64, entry_timeout: f64) -> Self {
        Self {
            ino: attr.ino,
            generation,
            attr: attr.convert(),
            attr_timeout,
            entry_timeout,
        }
    }
}

#[repr(C)]
pub(crate) struct FuseLowLevelOps {
    // void (*init) (void *userdata, struct fuse_conn_info *conn);
    pub(crate) init: *const fn(*mut c_void, *mut FuseConnInfo),

    // void (*destroy) (void *userdata);
    pub(crate) destroy: *const fn(*mut c_void),

    // void (*lookup) (fuse_req_t req, fuse_ino_t parent, const char *name);
    pub(crate) lookup: *const fn(*mut FuseReq, u64, *const c_char),

    // void (*forget) (fuse_req_t req, fuse_ino_t ino, uint64_t nlookup);
    pub(crate) forget: *const fn(*mut FuseReq, u64, u64),

    // void (*getattr) (fuse_req_t req, fuse_ino_t ino, struct fuse_file_info *fi);
    pub(crate) getattr: *const fn(*mut FuseReq, u64, *mut FuseFileInfo),

    // void (*setattr) (fuse_req_t req, fuse_ino_t ino, struct stat *attr, int to_set, struct fuse_file_info *fi);
    pub(crate) setattr: *const fn(*mut FuseReq, u64, *mut stat, i16, *mut FuseFileInfo),

    // void (*readlink) (fuse_req_t req, fuse_ino_t ino);
    pub(crate) readlink: *const fn(*mut FuseReq, u64),

    // void (*mknod) (fuse_req_t req, fuse_ino_t parent, const char *name, mode_t mode, dev_t rdev);
    pub(crate) mknod: *const fn(*mut FuseReq, u64, *const c_char, mode_t, dev_t),

    // void (*mkdir) (fuse_req_t req, fuse_ino_t parent, const char *name, mode_t mode);
    pub(crate) mkdir: *const fn(*mut FuseReq, u64, *const c_char, mode_t),

    // void (*unlink) (fuse_req_t req, fuse_ino_t parent, const char *name);
    pub(crate) unlink: *const fn(*mut FuseReq, u64, *const c_char),

    // void (*rmdir) (fuse_req_t req, fuse_ino_t parent, const char *name);
    pub(crate) rmdir: *const fn(*mut FuseReq, u64, *const c_char),

    // void (*symlink) (fuse_req_t req, const char *link, fuse_ino_t parent, const char *name);
    pub(crate) symlink: *const fn(*mut FuseReq, *const c_char, u64, *const c_char),

    // void (*rename) (fuse_req_t req, fuse_ino_t parent, const char *name, fuse_ino_t newparent, const char *newname, unsigned int flags);
    pub(crate) rename: *const fn(*mut FuseReq, u64, *const c_char, u64, *const c_char, u16),

    // void (*link) (fuse_req_t req, fuse_ino_t ino, fuse_ino_t newparent, const char *newname);
    pub(crate) link: *const fn(*mut FuseReq, u64, u64, *const c_char),

    // void (*open) (fuse_req_t req, fuse_ino_t ino, struct fuse_file_info *fi);
    pub(crate) open: *const fn(*mut FuseReq, u64, *mut FuseFileInfo),

    // void (*read) (fuse_req_t req, fuse_ino_t ino, size_t size, off_t off, struct fuse_file_info *fi);
    pub(crate) read: *const fn(*mut FuseReq, u64, size_t, off_t, *mut FuseFileInfo),

    // void (*write) (fuse_req_t req, fuse_ino_t ino, const char *buf, size_t size, off_t off, struct fuse_file_info *fi);
    pub(crate) write: *const fn(*mut FuseReq, u64, *const c_char, size_t, off_t, *mut FuseFileInfo),

    // void (*flush) (fuse_req_t req, fuse_ino_t ino, struct fuse_file_info *fi);
    pub(crate) flush: *const fn(*mut FuseReq, u64, *mut FuseFileInfo),

    // void (*release) (fuse_req_t req, fuse_ino_t ino, struct fuse_file_info *fi);
    pub(crate) release: *const fn(*mut FuseReq, u64, *mut FuseFileInfo),

    // void (*fsync) (fuse_req_t req, fuse_ino_t ino, int datasync, struct fuse_file_info *fi);
    pub(crate) fsync: *const fn(*mut FuseReq, u64, c_int, *mut FuseFileInfo),

    // void (*opendir) (fuse_req_t req, fuse_ino_t ino, struct fuse_file_info *fi);
    pub(crate) opendir: *const fn(*mut FuseReq, u64, *mut FuseFileInfo),

    // void (*readdir) (fuse_req_t req, fuse_ino_t ino, size_t size, off_t off, struct fuse_file_info *fi);
    pub(crate) readdir: *const fn(*mut FuseReq, u64, size_t, off_t, *mut FuseFileInfo),

    // void (*releasedir) (fuse_req_t req, fuse_ino_t ino, struct fuse_file_info *fi);
    pub(crate) releasedir: *const fn(*mut FuseReq, u64, *mut FuseFileInfo),

    // void (*fsyncdir) (fuse_req_t req, fuse_ino_t ino, int datasync, struct fuse_file_info *fi);
    pub(crate) fsyncdir: *const fn(*mut FuseReq, u64, c_int, *mut FuseFileInfo),

    // void (*statfs) (fuse_req_t req, fuse_ino_t ino);
    pub(crate) statfs: *const fn(*mut FuseReq, u64),

    // void (*setxattr) (fuse_req_t req, fuse_ino_t ino, const char *name, const char *value, size_t size, int flags);
    pub(crate) setxattr: *const fn(*mut FuseReq, u64, *const c_char, *const c_char, size_t, c_int),

    // void (*getxattr) (fuse_req_t req, fuse_ino_t ino, const char *name, size_t size);
    pub(crate) getxattr: *const fn(*mut FuseReq, u64, *const c_char, size_t),

    // void (*listxattr) (fuse_req_t req, fuse_ino_t ino, size_t size);
    pub(crate) listxattr: *const fn(*mut FuseReq, u64, size_t),

    // void (*removexattr) (fuse_req_t req, fuse_ino_t ino, const char *name);
    pub(crate) removexattr: *const fn(*mut FuseReq, u64, *const c_char),

    // void (*access) (fuse_req_t req, fuse_ino_t ino, int mask);
    pub(crate) access: *const fn(*mut FuseReq, u64, c_int),

    // void (*create) (fuse_req_t req, fuse_ino_t parent, const char *name, mode_t mode, struct fuse_file_info *fi);
    pub(crate) create: *const fn(*mut FuseReq, u64, *const c_char, mode_t, *mut FuseFileInfo),

    // void (*getlk) (fuse_req_t req, fuse_ino_t ino, struct fuse_file_info *fi, struct flock *lock);
    pub(crate) getlk: *const fn(*mut FuseReq, u64, *mut FuseFileInfo, *mut flock),

    // void (*setlk) (fuse_req_t req, fuse_ino_t ino, struct fuse_file_info *fi, struct flock *lock, int sleep);
    pub(crate) setlk: *const fn(*mut FuseReq, u64, *mut FuseFileInfo, *mut flock, c_int),

    // void (*bmap) (fuse_req_t req, fuse_ino_t ino, size_t blocksize, uint64_t idx);
    pub(crate) bmap: *const fn(*mut FuseReq, u64, size_t, u64),

    // TODO: Ioctl

    // void (*poll) (fuse_req_t req, fuse_ino_t ino, struct fuse_file_info *fi, struct fuse_pollhandle *ph);
    pub(crate) poll: *const fn(*mut FuseReq, u64, *mut FuseFileInfo, *mut FusePollhandle),

    // void (*write_buf) (fuse_req_t req, fuse_ino_t ino, struct fuse_bufvec *bufv, off_t off, struct fuse_file_info *fi);
    pub(crate) write_buf: *const fn(*mut FuseReq, u64, *mut FuseBufvec, off_t, *mut FuseFileInfo),

    // TODO
    // void (*retrieve_reply) (fuse_req_t req, void *cookie, fuse_ino_t ino, off_t offset, struct fuse_bufvec *bufv);
    // pub(crate) retrieve_reply: *const fn(*mut FuseReq, *mut c_void, u64, off_t, *mut FuseBufvec),

    // void (*forget_multi) (fuse_req_t req, size_t count, struct fuse_forget_data *forgets);
    pub(crate) forget_multi: *const fn(*mut FuseReq, size_t, *mut FuseForgetData),

    // void (*flock) (fuse_req_t req, fuse_ino_t ino, struct fuse_file_info *fi, int op);
    pub(crate) flock: *const fn(*mut FuseReq, u64, *mut FuseFileInfo, c_int),

    // void (*fallocate) (fuse_req_t req, fuse_ino_t ino, int mode, off_t offset, off_t length, struct fuse_file_info *fi);
    pub(crate) fallocate: *const fn(*mut FuseReq, u64, c_int, off_t, off_t, *mut FuseFileInfo),

    // void (*readdirplus) (fuse_req_t req, fuse_ino_t ino, size_t size, off_t off, struct fuse_file_info *fi);
    pub(crate) readdirplus: *const fn(*mut FuseReq, u64, size_t, off_t, *mut FuseFileInfo),

    // void (*copy_file_range) (fuse_req_t req, fuse_ino_t ino_in,
    //              off_t off_in, struct fuse_file_info *fi_in,
    //              fuse_ino_t ino_out, off_t off_out,
    //              struct fuse_file_info *fi_out, size_t len,
    //              int flags);
    pub(crate) copy_file_range: *const fn(
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
    pub(crate) lseek: *const fn(*mut FuseReq, u64, off_t, c_int, *mut FuseFileInfo),
}

#[repr(C)]
#[derive(Debug)]
pub struct FuseAttr {
    pub dev: u64,
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
}

impl FuseAttr {
    pub fn new(attr: &stat) -> Self {
        Self {
            dev: attr.st_dev as u64,
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
        }
    }
    pub fn convert(&self) -> stat {
        let mut sb = unsafe { mem::zeroed::<stat>() };
        sb.st_dev = self.dev as dev_t;
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

#[repr(C)]
pub struct FuseStatvfs {
    pub bsize: u64,
    pub frsize: u64,
    pub blocks: u64,
    pub bfree: u64,
    pub bavail: u64,
    pub files: u64,
    pub ffree: u64,
    pub favail: u64,
    pub fsid: u64,
    pub flag: u64,
    pub namemax: u64,
}

impl FuseStatvfs {
    pub fn new(st: &statvfs) -> Self {
        Self {
            bsize: st.f_bsize as u64,
            frsize: st.f_frsize as u64,
            blocks: st.f_blocks as u64,
            bfree: st.f_bfree as u64,
            bavail: st.f_bavail as u64,
            files: st.f_files as u64,
            ffree: st.f_ffree as u64,
            favail: st.f_favail as u64,
            fsid: st.f_fsid as u64,
            flag: st.f_flag as u64,
            namemax: st.f_namemax as u64,
        }
    }
    pub fn convert(&self) -> statvfs {
        let mut st = unsafe { mem::zeroed::<statvfs>() };
        st.f_bsize = self.bsize as c_ulong;
        st.f_frsize = self.frsize as c_ulong;
        st.f_blocks = self.blocks as fsblkcnt_t;
        st.f_bfree = self.bfree as fsblkcnt_t;
        st.f_bavail = self.bavail as fsblkcnt_t;
        st.f_files = self.files as fsfilcnt_t;
        st.f_ffree = self.ffree as fsfilcnt_t;
        st.f_favail = self.favail as fsfilcnt_t;
        st.f_fsid = self.fsid as c_ulong;
        st.f_flag = self.flag as c_ulong;
        st.f_namemax = self.namemax as c_ulong;
        st
    }
}

#[repr(C)]
pub struct FuseLock {
    type_: i16,
    whence: i16,
    start: i64,
    len: i64,
    pid: i32,
}

impl FuseLock {
    pub fn new(lock: &flock) -> Self {
        Self {
            type_: lock.l_type as i16,
            whence: lock.l_whence as i16,
            start: lock.l_start as i64,
            len: lock.l_len as i64,
            pid: lock.l_pid as i32,
        }
    }
    pub fn convert(&self) -> flock {
        let mut lock = unsafe { mem::zeroed::<flock>() };
        lock.l_type = self.type_ as c_short;
        lock.l_whence = self.whence as c_short;
        lock.l_start = self.start as off_t;
        lock.l_len = self.len as off_t;
        lock.l_pid = self.pid as pid_t;
        lock
    }
}

#[derive(Debug, Copy, Clone)]
pub enum FileType {
    Socket,
    SymbolicLink,
    RegularFile,
    BlockDevice,
    Directory,
    CharacterDevice,
    FIFO,
}

impl FileType {
    pub fn to_mode(&self) -> u32 {
        match self {
            FileType::Socket => S_IFSOCK,
            FileType::SymbolicLink => S_IFLNK,
            FileType::RegularFile => S_IFREG,
            FileType::BlockDevice => S_IFBLK,
            FileType::Directory => S_IFDIR,
            FileType::CharacterDevice => S_IFCHR,
            FileType::FIFO => S_IFIFO,
        }
    }
    pub fn new(value: u32) -> FileType {
        if value & S_IFSOCK == S_IFSOCK {
            FileType::Socket
        } else if value & S_IFSOCK == S_IFLNK {
            FileType::SymbolicLink
        } else if value & S_IFSOCK == S_IFBLK {
            FileType::BlockDevice
        } else if value & S_IFSOCK == S_IFDIR {
            FileType::Directory
        } else if value & S_IFSOCK == S_IFCHR {
            FileType::CharacterDevice
        } else if value & S_IFSOCK == S_IFIFO {
            FileType::FIFO
        } else {
            FileType::RegularFile
        }
    }
}

pub struct FuseDirectory {
    pub name: Vec<u8>,
    pub file_type: FileType,
    pub ino: u64,
}

impl FuseDirectory {
    pub fn attr(&self) -> FuseAttr {
        FuseAttr {
            dev: 0,
            ino: self.ino,
            size: 0,
            blocks: 0,
            atime: 0,
            atimensec: 0,
            mtime: 0,
            mtimensec: 0,
            ctime: 0,
            ctimensec: 0,
            mode: self.file_type.to_mode(),
            nlink: 0,
            uid: 0,
            gid: 0,
            rdev: 0,
            blksize: 0,
        }
    }
}

#[link(name = "fuse3")]
extern "C" {
    #[allow(improper_ctypes)]
    pub(crate) fn fuse_session_new(
        args: *mut FuseArgs,
        op: *const FuseLowLevelOps,
        op_size: size_t,
        userdata: *mut c_void,
    ) -> *mut FuseSession;
    #[allow(improper_ctypes)]
    pub(crate) fn fuse_session_destroy(fuse_session: *mut FuseSession);
    #[allow(improper_ctypes)]
    pub(crate) fn fuse_session_mount(fuse_session: *mut FuseSession, mountpoint: *const c_char);
    #[allow(improper_ctypes)]
    pub(crate) fn fuse_session_unmount(fuse_session: *mut FuseSession);
    // #[allow(improper_ctypes)]
    // pub fn fuse_session_exit(fuse_session: *mut FuseSession);
    #[allow(improper_ctypes)]
    pub fn fuse_session_exited(fuse_session: *mut FuseSession) -> c_int;
    #[allow(improper_ctypes)]
    pub fn fuse_session_reset(fuse_session: *mut FuseSession);
    #[allow(improper_ctypes)]
    pub(crate) fn fuse_req_userdata(req: *mut FuseReq) -> *mut c_void;
    #[allow(improper_ctypes)]
    pub(crate) fn fuse_req_ctx(req: *mut FuseReq) -> *const FuseCtx;
    #[allow(improper_ctypes)]
    pub(crate) fn fuse_reply_open(req: *mut FuseReq, fi: *const FuseFileInfo) -> c_int;
    #[allow(improper_ctypes)]
    pub(crate) fn fuse_reply_write(req: *mut FuseReq, count: size_t) -> c_int;
    #[allow(improper_ctypes)]
    pub(crate) fn fuse_reply_err(req: *mut FuseReq, err: c_int) -> c_int;
    #[allow(improper_ctypes)]
    pub(crate) fn fuse_reply_none(req: *mut FuseReq);
    #[allow(improper_ctypes)]
    pub(crate) fn fuse_reply_entry(req: *mut FuseReq, e: *const FuseEntryParam) -> c_int;
    #[allow(improper_ctypes)]
    pub(crate) fn fuse_reply_create(
        req: *mut FuseReq,
        e: *const FuseEntryParam,
        fi: *const FuseFileInfo,
    ) -> c_int;
    #[allow(improper_ctypes)]
    pub(crate) fn fuse_reply_attr(req: *mut FuseReq, attr: *const stat, attr_timeout: f64)
        -> c_int;
    #[allow(improper_ctypes)]
    pub(crate) fn fuse_reply_readlink(req: *mut FuseReq, link: *const c_char) -> c_int;
    #[allow(improper_ctypes)]
    pub(crate) fn fuse_reply_buf(req: *mut FuseReq, buf: *const c_char, size: size_t) -> c_int;
    // #[allow(improper_ctypes)]
    // pub(crate) fn fuse_reply_data(req: *mut FuseReq, bufv: *mut FuseBufvec, flags: FuseBufCopyFlags) -> c_int;
    // #[allow(improper_ctypes)]
    // pub(crate) fn fuse_reply_iov(req: *mut FuseReq, iov: *const iovec, count: c_int) -> c_int;
    #[allow(improper_ctypes)]
    pub(crate) fn fuse_reply_statfs(req: *mut FuseReq, stbuf: *const statvfs) -> c_int;
    #[allow(improper_ctypes)]
    pub(crate) fn fuse_reply_xattr(req: *mut FuseReq, count: size_t) -> c_int;
    #[allow(improper_ctypes)]
    pub(crate) fn fuse_reply_lock(req: *mut FuseReq, lock: *const flock) -> c_int;
    #[allow(improper_ctypes)]
    pub(crate) fn fuse_reply_bmap(req: *mut FuseReq, idx: u64) -> c_int;
    #[allow(improper_ctypes)]
    pub(crate) fn fuse_reply_poll(req: *mut FuseReq, revents: u32) -> c_int;
    #[allow(improper_ctypes)]
    pub(crate) fn fuse_reply_lseek(req: *mut FuseReq, off: off_t) -> c_int;
    #[allow(improper_ctypes)]
    pub(crate) fn fuse_add_direntry(
        req: *mut FuseReq,
        buf: *mut c_char,
        bufsize: size_t,
        name: *const c_char,
        stbuf: *const stat,
        off: off_t,
    ) -> size_t;
    #[allow(improper_ctypes)]
    pub(crate) fn fuse_session_receive_buf(
        fuse_session: *mut FuseSession,
        buf: *mut FuseBuf,
    ) -> c_int;
    #[allow(improper_ctypes)]
    pub(crate) fn fuse_session_process_buf(
        fuse_session: *mut FuseSession,
        buf: *const FuseBuf,
    ) -> c_int;
    #[allow(improper_ctypes)]
    pub(crate) fn fuse_set_signal_handlers(fuse_session: *mut FuseSession) -> c_int;
    #[allow(improper_ctypes)]
    pub(crate) fn fuse_remove_signal_handlers(fuse_session: *mut FuseSession);

}

#[cfg(test)]
mod tests {
    use crate::FuseAttr;
    use libc::{stat, S_IFREG, S_IRGRP, S_IROTH, S_IRUSR, S_IWGRP, S_IWOTH, S_IWUSR};
    use std::borrow::BorrowMut;
    use std::ffi::CString;
    use std::mem;
    #[test]
    fn fuse_attr_new() {
        let sb = unsafe {
            let mut sb: stat = unsafe { mem::zeroed::<stat>() };
            let ret = unsafe {
                stat(
                    CString::new("./tests/resource/a").unwrap().as_ptr(),
                    sb.borrow_mut(),
                )
            };
            assert_eq!(ret, 0);
            sb
        };
        let attr = FuseAttr::new(&sb);
        assert_eq!(attr.size, 13);
        assert_eq!(attr.blocks, 8);
        assert_eq!(
            attr.mode,
            S_IFREG | S_IRUSR | S_IWUSR | S_IRGRP | S_IWGRP | S_IROTH
        );
        assert_eq!(attr.nlink, 1);
        assert_eq!(attr.rdev, 0);

        let sc = attr.convert();
        assert_eq!(sb.st_dev, sc.st_dev);
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

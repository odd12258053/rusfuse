use std::env;

use rusfuse::*;

#[derive(Debug)]
struct HelloFs;

const ENOENT: i32 = 2;
const ENOSYS: i32 = 38;

const FILE_NAME: &[u8] = b"hello";
const TEXT: &[u8] = b"Hello World!\n";

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

impl rusfuse::FileSystem for HelloFs {
    fn lookup(&mut self, _ctx: &FuseCtx, parent: u64, name: &[u8]) -> Result<FuseEntryParam, i32> {
        println!("call lookup parent: {:?} name: {:?}", parent, name);
        if parent == 1 && name == FILE_NAME {
            Ok(rusfuse::FuseEntryParam::new(TEST_FILE_ATTR, 0, 10.0, 10.0))
        } else {
            Err(ENOENT)
        }
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
    fn read(
        &mut self,
        _ctx: &FuseCtx,
        ino: u64,
        _size: usize,
        _off: i64,
        _fi: &mut FuseFileInfo,
    ) -> Result<Vec<u8>, i32> {
        println!("call read");
        if ino == 2 {
            Ok(TEXT.to_vec())
        } else {
            Err(ENOSYS)
        }
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
                    name: b".".to_vec(),
                    file_type: FileType::Directory,
                    ino: 1,
                },
                FuseDirectory {
                    name: b"..".to_vec(),
                    file_type: FileType::Directory,
                    ino: 1,
                },
                FuseDirectory {
                    name: FILE_NAME.to_vec(),
                    file_type: FileType::RegularFile,
                    ino: 2,
                },
            ])
        }
    }
}

fn main() {
    let mountpoint: String = env::args().nth(1).unwrap();
    let mut file_system = HelloFs {};
    let ops = FuseOpFlag::Lookup | FuseOpFlag::Readdir | FuseOpFlag::Read | FuseOpFlag::Getattr;
    Fuse::new(&mountpoint, &mut file_system, ops).run();
}

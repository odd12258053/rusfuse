#[macro_use]
extern crate log;

use env_logger::Env;
use libc;
use rusfuse::*;
use std::env;
use std::str::from_utf8;

const TIMEOUT: f64 = 1.0;

pub mod file_tree {
    use std::collections::{BTreeMap, BTreeSet};
    use std::fmt::{Debug, Formatter};
    use std::time::{Duration, SystemTime};

    use colored::Colorize;

    use rusfuse::{FileType, FuseAttr, FuseDirectory};
    use std::str::from_utf8;

    pub(crate) type InoType = u32;
    const INO_MAX_SIZE: InoType = InoType::max_value();

    #[derive(Debug)]
    pub(crate) struct Node {
        pub(crate) name: Vec<u8>,
        pub(crate) data: Vec<u8>,
        pub(self) parent: InoType,
        pub(self) children: BTreeSet<InoType>,
        pub(self) file_type: FileType,
        pub(self) ino: InoType,
        pub(self) atime: SystemTime,
        pub(self) mtime: SystemTime,
        pub(self) ctime: SystemTime,
        pub(crate) mode: u32,
        pub(crate) uid: u32,
        pub(crate) gid: u32,
        pub(crate) nlink: u32,
    }

    impl Node {
        pub(crate) fn new(
            name: Vec<u8>,
            parent: InoType,
            file_type: FileType,
            mode: u32,
            uid: u32,
            gid: u32,
        ) -> Self {
            Node {
                name,
                file_type,
                parent,
                data: vec![0; 0],
                children: BTreeSet::new(),
                ino: 0,
                atime: SystemTime::now(),
                mtime: SystemTime::now(),
                ctime: SystemTime::now(),
                mode,
                uid,
                gid,
                nlink: 0,
            }
        }
        pub(crate) fn parent(&self) -> &InoType {
            &(self.parent)
        }
        pub(crate) fn children(&self) -> &BTreeSet<InoType> {
            &(self.children)
        }
        pub(crate) fn file_type(&self) -> &FileType {
            &(self.file_type)
        }

        pub(crate) fn to_attr(&self) -> FuseAttr {
            let (atime, atimensec) = {
                let d = self.atime.duration_since(SystemTime::UNIX_EPOCH).unwrap();
                (d.as_secs(), d.subsec_nanos())
            };
            let (mtime, mtimensec) = {
                let d = self.mtime.duration_since(SystemTime::UNIX_EPOCH).unwrap();
                (d.as_secs(), d.subsec_nanos())
            };
            let (ctime, ctimensec) = {
                let d = self.ctime.duration_since(SystemTime::UNIX_EPOCH).unwrap();
                (d.as_secs(), d.subsec_nanos())
            };

            FuseAttr {
                dev: 0,
                ino: self.ino as u64,
                size: self.data.len() as u64,
                blocks: 8,
                atime,
                atimensec,
                mtime,
                mtimensec,
                ctime,
                ctimensec,
                mode: self.mode + self.file_type.to_mode(),
                nlink: self.nlink,
                uid: self.uid,
                gid: self.uid,
                rdev: 0,
                blksize: 4096,
            }
        }
        pub(self) fn update_ino(&mut self, ino: InoType) {
            self.ino = ino;
        }
        pub(crate) fn stamp_atime(&mut self) {
            self.atime = SystemTime::now()
        }
        pub(crate) fn stamp_mtime(&mut self) {
            self.mtime = SystemTime::now()
        }
        pub(crate) fn stamp_ctime(&mut self) {
            self.ctime = SystemTime::now()
        }
    }

    impl std::fmt::Display for Node {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            match from_utf8(&self.name[..]) {
                Ok(name) => {
                    match self.file_type {
                        FileType::Directory => {
                            write!(f, "{}", name.truecolor(75, 85, 255))
                            // write!(f, "{}", name.as_str().truecolor(85, 255, 75))
                        }
                        _ => {
                            write!(f, "{}", name)
                        }
                    }
                }
                Err(_) => {
                    write!(f, "{:?}", self.name)
                }
            }
        }
    }

    #[derive(Debug)]
    pub(crate) struct Tree {
        pub(self) nodes: BTreeMap<InoType, Node>,
    }

    impl Tree {
        pub(crate) fn new() -> Self {
            Tree {
                nodes: BTreeMap::new(),
            }
        }
        pub(crate) fn push(&mut self, mut node: Node) -> InoType {
            let ino: InoType = (|| {
                for i in (node.parent + 1)..INO_MAX_SIZE {
                    if !self.nodes.contains_key(&i) {
                        return i;
                    }
                }
                for i in (1..node.parent).rev() {
                    if !self.nodes.contains_key(&i) {
                        return i;
                    }
                }
                panic!("Fail to allocate a new node because the tree is full.")
            })();
            if let Some(parent) = self.get_mut(&node.parent) {
                parent.children.insert(ino);
            }
            node.ino = ino;
            self.nodes.insert(ino, node);
            ino
        }
        pub(crate) fn remove(&mut self, ino: &InoType) -> Option<Node> {
            match self.nodes.remove(&ino) {
                Some(node) => {
                    if let Some(parent) = self.get_mut(&node.parent) {
                        parent.children.remove(ino);
                    }
                    for i in node.children.iter() {
                        self.remove(i);
                    }
                    Some(node)
                }
                None => None,
            }
        }
        pub(crate) fn get(&self, ino: &InoType) -> Option<&Node> {
            self.nodes.get(ino)
        }
        pub(crate) fn get_mut(&mut self, ino: &InoType) -> Option<&mut Node> {
            self.nodes.get_mut(ino)
        }
        pub(crate) fn search(&self, parent: &InoType, name: &[u8]) -> Option<(&InoType, &Node)> {
            self.get_children(parent).and_then(|children| {
                for (ino, ch) in children {
                    if ch.name == name {
                        return Some((ino, ch));
                    }
                }
                None
            })
        }
        pub(crate) fn len(&self) -> usize {
            self.nodes.len()
        }
        pub(crate) fn maxsize(&self) -> InoType {
            INO_MAX_SIZE
        }
        pub(crate) fn usage(&self) -> u8 {
            (self.len() * 100 / self.maxsize() as usize) as u8
        }
        #[allow(dead_code)]
        pub(crate) fn get_children(&self, ino: &InoType) -> Option<Vec<(&InoType, &Node)>> {
            match self.get(ino) {
                Some(node) => {
                    let mut nodes: Vec<(&InoType, &Node)> = node
                        .children
                        .iter()
                        .map(|i| (i, self.get(i).unwrap()))
                        .collect();
                    nodes.sort_by(|(_, a), (_, b)| a.name.cmp(&b.name));
                    Some(nodes)
                }
                None => None,
            }
        }
        pub(self) fn get_children_with_node(&self, node: &Node) -> Vec<&Node> {
            let mut nodes: Vec<&Node> =
                node.children.iter().map(|i| self.get(i).unwrap()).collect();
            nodes.sort_by(|a, b| a.name.cmp(&b.name));
            nodes
        }
        pub(crate) fn contains(&self, ino: &InoType) -> bool {
            self.nodes.contains_key(&ino)
        }
        pub(crate) fn move_node(&mut self, ino: &InoType, parent: &InoType) -> Option<&Node> {
            if self.contains(ino) && self.contains(parent) {
                let parent_ino = self.get(ino).unwrap().parent;
                if self.contains(&parent_ino) {
                    {
                        let parent_node = self.get_mut(&parent_ino).unwrap();
                        parent_node.children.remove(ino);
                    }
                    {
                        let new_parent_node = self.get_mut(parent).unwrap();
                        new_parent_node.children.insert(*ino);
                    }
                    let node = self.get_mut(ino).unwrap();
                    node.parent = *parent;
                    return Some(node);
                }
            }
            None
        }
    }

    const RULED_LINE_EMP: &str = "";
    const RULED_LINE_PIP: &str = "│ ";
    const RULED_LINE_TRI: &str = "├─";
    const RULED_LINE_END: &str = "└─";

    impl Tree {
        fn tree(
            &self,
            node: &Node,
            prefix: &str,
            ruled_line: &str,
            depth: u32,
            limit: u32,
        ) -> String {
            let mut buf = String::new();

            buf.push_str(
                (if ruled_line == RULED_LINE_EMP {
                    format!("{}{}\n", prefix, node)
                } else {
                    format!("{}{} {}\n", prefix, ruled_line, node)
                })
                .as_str(),
            );

            if limit != depth {
                let len = node.children.len();
                let mut cnt: usize = 0;
                for n in self.get_children_with_node(node) {
                    cnt += 1;
                    let pre = if ruled_line == RULED_LINE_TRI {
                        format!("{}{} ", prefix, RULED_LINE_PIP)
                    } else if ruled_line == RULED_LINE_EMP {
                        format!("{}", prefix)
                    } else {
                        format!("{}   ", prefix)
                    };
                    let ruled = if cnt == len {
                        RULED_LINE_END
                    } else {
                        RULED_LINE_TRI
                    };
                    let tmp_buf = self.tree(n, pre.as_str(), ruled, depth + 1, limit);
                    buf.push_str(tmp_buf.as_str());
                }
            }
            buf
        }
    }

    impl std::fmt::Display for Tree {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            match self.get(&1) {
                None => {
                    write!(f, "Usage:{:3}%", self.usage(),)
                }
                Some(node) => {
                    write!(
                        f,
                        "Usage:{:3}%\n{}",
                        self.usage(),
                        self.tree(node, "", "", 1, 0)
                    )
                }
            }
        }
    }
}

use self::file_tree::*;

struct InMemoryFs {
    tree: Tree,
}

impl InMemoryFs {
    fn new() -> Self {
        let mut tree = Tree::new();
        tree.push(Node::new(
            b"/".to_vec(),
            0,
            FileType::Directory,
            0o555,
            0,
            0,
        ));
        InMemoryFs { tree }
    }
}

impl rusfuse::FileSystem for InMemoryFs {
    fn init(&mut self) -> Result<(), i32> {
        debug!("init");
        Ok(())
    }
    fn destroy(&mut self) -> Result<(), i32> {
        debug!("destroy");
        Ok(())
    }
    fn lookup(&mut self, _ctx: &FuseCtx, parent: u64, name: &[u8]) -> Result<FuseEntryParam, i32> {
        debug!("lookup: parent={},name={:?}", parent, from_utf8(name));
        match self.tree.get_children(&(parent as InoType)) {
            Some(children) => {
                for (_, ch) in children {
                    if ch.name == name {
                        return Ok((FuseEntryParam::new(ch.to_attr(), 0, 0.0, 0.0)));
                    }
                }
                Err(libc::ENOENT)
            }
            None => Err(libc::ENOENT),
        }
    }
    fn mkdir(
        &mut self,
        ctx: &FuseCtx,
        parent: u64,
        name: &[u8],
        mode: u32,
    ) -> Result<FuseEntryParam, i32> {
        debug!(
            "mkdir: parent={},name={:?},mode={}",
            parent,
            from_utf8(name),
            mode
        );
        let access_mode = mode & 0o777;
        let ino = self.tree.push(Node::new(
            name.to_vec(),
            parent as InoType,
            FileType::Directory,
            access_mode,
            ctx.uid,
            ctx.gid,
        ));
        match self.tree.get(&ino) {
            Some(node) => Ok((FuseEntryParam::new(node.to_attr(), 0, 0.0, 0.0))),
            None => Err(libc::ENOSYS),
        }
    }
    fn mknod(
        &mut self,
        ctx: &FuseCtx,
        parent: u64,
        name: &[u8],
        mode: u32,
        rdev: u64,
    ) -> Result<FuseEntryParam, i32> {
        debug!(
            "mknod: parent={},name={:?},mode={},rdev={}",
            parent,
            from_utf8(name),
            mode,
            rdev
        );
        let access_mode = mode & 0o777;
        let ino = self.tree.push(Node::new(
            name.to_vec(),
            parent as InoType,
            FileType::new(mode),
            access_mode,
            ctx.uid,
            ctx.gid,
        ));
        match self.tree.get(&ino) {
            Some(node) => Ok((FuseEntryParam::new(node.to_attr(), 0, 0.0, 0.0))),
            None => Err(libc::ENOSYS),
        }
    }
    // TODO
    fn symlink(
        &mut self,
        _ctx: &FuseCtx,
        _link: &[u8],
        _parent: u64,
        _name: &[u8],
    ) -> Result<FuseEntryParam, i32> {
        debug!("symlink");
        Err(libc::ENOSYS)
    }
    // TODO
    fn link(
        &mut self,
        _ctx: &FuseCtx,
        _ino: u64,
        _newparent: u64,
        _newname: &[u8],
    ) -> Result<FuseEntryParam, i32> {
        debug!("link");
        Err(libc::ENOSYS)
    }
    fn unlink(&mut self, _ctx: &FuseCtx, parent: u64, name: &[u8]) -> Result<(), i32> {
        debug!("unlink: parent={},name={:?}", parent, from_utf8(name));
        let ino = match self.tree.search(&(parent as InoType), name) {
            Some((ino, _)) => *ino,
            None => return Err(libc::ENOENT),
        };
        self.tree.remove(&ino);
        Ok(())
    }
    fn rmdir(&mut self, _ctx: &FuseCtx, parent: u64, name: &[u8]) -> Result<(), i32> {
        debug!("rmdir: parent={},name={:?}", parent, from_utf8(name));
        let ino = match self.tree.search(&(parent as InoType), name) {
            Some((ino, _node)) => *ino,
            None => return Err(libc::ENOENT),
        };
        self.tree.remove(&ino);
        Ok(())
    }
    fn rename(
        &mut self,
        _ctx: &FuseCtx,
        parent: u64,
        name: &[u8],
        newparent: u64,
        newname: &[u8],
        flags: u16,
    ) -> Result<(), i32> {
        debug!(
            "rename: parent={},name={:?},newparent={},newname={:?},flags={}",
            parent,
            from_utf8(name),
            newparent,
            from_utf8(newname),
            flags
        );
        let ino: InoType = match self.tree.search(&(parent as InoType), name) {
            Some((ino, _node)) => *ino,
            None => return Err(libc::ENOENT),
        };
        if let Some(node) = self.tree.get_mut(&ino) {
            node.name = newname.to_vec();
        }
        match self.tree.move_node(&ino, &(newparent as InoType)) {
            Some(_node) => Ok(()),
            None => Err(libc::ENOENT),
        }
    }
    fn forget(&mut self, _ctx: &FuseCtx, forget: FuseForgetData) {
        debug!("forget forget={:?}", forget);
    }
    fn forget_multi(&mut self, _ctx: &FuseCtx, forgets: Vec<FuseForgetData>) {
        debug!("forget_multi forgets={:?}", forgets);
    }
    fn getattr(
        &mut self,
        _ctx: &FuseCtx,
        ino: u64,
        _fi: Option<&mut FuseFileInfo>,
    ) -> Result<(FuseAttr, f64), i32> {
        debug!("getattr: ino={}", ino);
        match self.tree.get(&(ino as InoType)) {
            Some(node) => Ok((node.to_attr(), TIMEOUT)),
            None => Err(libc::ENOENT),
        }
    }
    fn setattr(
        &mut self,
        _ctx: &FuseCtx,
        ino: u64,
        attr: &FuseAttr,
        to_set: i16,
        _fi: Option<&mut FuseFileInfo>,
    ) -> Result<(FuseAttr, f64), i32> {
        debug!("setattr: ino={},attr={:?},to_set={}", ino, attr, to_set);
        match self.tree.get_mut(&(ino as InoType)) {
            Some(node) => {
                if to_set == 1 {
                    node.mode = attr.mode & 0o777;
                }
                Ok((node.to_attr(), TIMEOUT))
            }
            None => Err(libc::ENOENT),
        }
    }
    // TODO
    fn readlink(&mut self, _ctx: &FuseCtx, _ino: u64) -> Result<Vec<u8>, i32> {
        debug!("readlink");
        Err(libc::ENOSYS)
    }
    // TODO
    fn opendir(
        &mut self,
        _ctx: &FuseCtx,
        _ino: u64,
        _fi: &mut FuseFileInfo,
    ) -> Result<FuseFileInfo, i32> {
        debug!("opendir");
        Err(libc::ENOSYS)
    }
    fn readdir(
        &mut self,
        _ctx: &FuseCtx,
        ino: u64,
        size: usize,
        off: i64,
        _fi: &mut FuseFileInfo,
    ) -> Result<Vec<FuseDirectory>, i32> {
        debug!("readdir: ino={},size={},off={}", ino, size, off);
        match self.tree.get(&(ino as InoType)) {
            Some(top) => {
                let mut dirs = Vec::new();
                dirs.push(FuseDirectory {
                    name: b".".to_vec(),
                    file_type: *top.file_type(),
                    ino,
                });
                if let Some(parent) = self.tree.get(top.parent()) {
                    dirs.push(FuseDirectory {
                        name: b"..".to_vec(),
                        file_type: *parent.file_type(),
                        ino: *top.parent() as u64,
                    });
                }
                for (i, ch) in self.tree.get_children(&(ino as InoType)).unwrap() {
                    dirs.push(FuseDirectory {
                        name: ch.name.to_vec(),
                        file_type: *ch.file_type(),
                        ino: *i as u64,
                    });
                }
                Ok(dirs)
            }
            None => Err(libc::ENOENT),
        }
    }
    // TODO
    fn readdirplus(
        &mut self,
        _ctx: &FuseCtx,
        _ino: u64,
        _size: usize,
        _off: i64,
        _fi: &mut FuseFileInfo,
    ) -> Result<Vec<u8>, i32> {
        debug!("readdirplus");
        Err(libc::ENOSYS)
    }
    // TODO
    fn releasedir(&mut self, _ctx: &FuseCtx, _ino: u64, _fi: &mut FuseFileInfo) -> Result<(), i32> {
        debug!("releasedir");
        Err(libc::ENOSYS)
    }
    // TODO
    fn fsyncdir(
        &mut self,
        _ctx: &FuseCtx,
        _ino: u64,
        _datasync: i32,
        _fi: &mut FuseFileInfo,
    ) -> Result<(), i32> {
        debug!("fsyncdir");
        Err(libc::ENOSYS)
    }
    // TODO
    fn open(&mut self, _ctx: &FuseCtx, ino: u64, fi: FuseFileInfo) -> Result<FuseFileInfo, i32> {
        debug!("open ino={} fi={:?}", ino, fi);
        Ok(fi)
    }
    // TODO
    fn release(&mut self, _ctx: &FuseCtx, _ino: u64, _fi: &mut FuseFileInfo) -> Result<(), i32> {
        debug!("release");
        Ok(())
    }
    // TODO
    fn flush(&mut self, _ctx: &FuseCtx, ino: u64, fi: &mut FuseFileInfo) -> Result<(), i32> {
        debug!("flush: ino={},fi={:?}", ino, fi);
        Ok(())
    }
    // TODO
    fn fsync(
        &mut self,
        _ctx: &FuseCtx,
        _ino: u64,
        _datasync: i32,
        _fi: &mut FuseFileInfo,
    ) -> Result<(), i32> {
        debug!("fsync");
        Err(libc::ENOSYS)
    }
    fn read(
        &mut self,
        _ctx: &FuseCtx,
        ino: u64,
        size: usize,
        off: i64,
        _fi: &mut FuseFileInfo,
    ) -> Result<Vec<u8>, i32> {
        debug!("read ino={},size={},off={}", ino, size, off);
        match self.tree.get(&(ino as InoType)) {
            Some(node) => Ok(node
                .data
                .splitn(2, |c| *c == 0)
                .next()
                .and_then(|cs| Some(cs.to_vec()))
                .unwrap_or(vec![0u8; 0])),
            None => Err(libc::ENOENT),
        }
    }
    fn write(
        &mut self,
        _ctx: &FuseCtx,
        ino: u64,
        buf: &[u8],
        size: usize,
        off: i64,
        _fi: &mut FuseFileInfo,
    ) -> Result<usize, i32> {
        debug!("write: ino={},size={},off={}", ino, size, off);
        match self.tree.get_mut(&(ino as InoType)) {
            Some(node) => {
                if node.data.len() < size + (off as usize) {
                    node.data.resize(size + (off as usize), 0);
                }
                info!("size: {}", node.data.len());
                let mut i = 0;
                for c in buf {
                    node.data[i + (off as usize)] = *c;
                    i += 1;
                }
                Ok(size)
            }
            None => Err(libc::ENOENT),
        }
    }
    // TODO
    fn write_buf(
        &mut self,
        _ctx: &FuseCtx,
        _ino: u64,
        _bufv: &mut FuseBufvec,
        _off: i64,
        _fi: &FuseFileInfo,
    ) -> Result<usize, i32> {
        debug!("write_buf");
        Err(libc::ENOSYS)
    }
    // TODO
    fn statfs(&mut self, _ctx: &FuseCtx, _ino: u64) -> Result<FuseStatvfs, i32> {
        debug!("statfs");
        Err(libc::ENOSYS)
    }
    // TODO
    fn fallocate(
        &mut self,
        _ctx: &FuseCtx,
        _ino: u64,
        _mode: i32,
        _offset: i64,
        _length: i64,
        _fi: &mut FuseFileInfo,
    ) -> Result<(), i32> {
        debug!("fallocate");
        Err(libc::ENOSYS)
    }
    // TODO
    fn flock(
        &mut self,
        _ctx: &FuseCtx,
        _ino: u64,
        _fi: &mut FuseFileInfo,
        _op: i32,
    ) -> Result<(), i32> {
        debug!("flock");
        Err(libc::ENOSYS)
    }
    // TODO
    fn getxattr(
        &mut self,
        _ctx: &FuseCtx,
        ino: u64,
        name: &[u8],
        size: usize,
    ) -> Result<Vec<u8>, i32> {
        debug!(
            "getxattr: ino={},name={:?},size={}",
            ino,
            from_utf8(name),
            size
        );
        Ok(b"".to_vec())
    }
    // TODO
    fn listxattr(&mut self, _ctx: &FuseCtx, _ino: u64, _size: usize) -> Result<Vec<u8>, i32> {
        debug!("listxattr");
        Err(libc::ENOSYS)
    }
    // TODO
    fn setxattr(
        &mut self,
        _ctx: &FuseCtx,
        _ino: u64,
        _name: &[u8],
        _value: &[u8],
        _size: usize,
        _flags: i32,
    ) -> Result<(), i32> {
        debug!("setxattr");
        Err(libc::ENOSYS)
    }
    // TODO
    fn removexattr(&mut self, _ctx: &FuseCtx, _ino: u64, _name: &[u8]) -> Result<(), i32> {
        debug!("removexattr");
        Err(libc::ENOSYS)
    }
}

fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("debug"))
        .format_timestamp_micros()
        .format_module_path(true)
        .init();

    let mountpoint: String = env::args().nth(1).unwrap();
    let mut file_system = InMemoryFs::new();
    let ops = FuseOpFlag::Init
        | FuseOpFlag::Destroy
        | FuseOpFlag::Lookup
        | FuseOpFlag::Mkdir
        | FuseOpFlag::Mknod
        | FuseOpFlag::Symlink
        | FuseOpFlag::Link
        | FuseOpFlag::Unlink
        | FuseOpFlag::Rmdir
        | FuseOpFlag::Rename
        // | FuseOpFlag::Forget
        // | FuseOpFlag::ForgetMulti
        | FuseOpFlag::Getattr
        | FuseOpFlag::Setattr
        | FuseOpFlag::Readlink
        | FuseOpFlag::Opendir
        | FuseOpFlag::Readdir
        | FuseOpFlag::Readdirplus
        | FuseOpFlag::Releasedir
        | FuseOpFlag::Fsyncdir
        | FuseOpFlag::Open
        | FuseOpFlag::Release
        | FuseOpFlag::Flush
        | FuseOpFlag::Fsync
        | FuseOpFlag::Read
        | FuseOpFlag::Write
        | FuseOpFlag::WriteBuf
        | FuseOpFlag::Statfs
        | FuseOpFlag::Fallocate
        | FuseOpFlag::Flock
        | FuseOpFlag::Getxattr
        | FuseOpFlag::Listxattr
        | FuseOpFlag::Setxattr
        | FuseOpFlag::Removexattr;
    Fuse::new(&mountpoint, &mut file_system, ops).run();
}

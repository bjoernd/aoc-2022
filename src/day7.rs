use std::cell::RefCell;
use std::rc::Rc;

use crate::{DaySolution, FromInput};

#[derive(Clone)]
enum FileObject {
    FILE(FileNode),
    DIR(Rc<RefCell<DirNode>>),
}

#[derive(Clone)]
struct FileNode {
    name: String,
    size: usize,
}

#[derive(Clone)]
struct DirNode {
    name: String,
    entries: Vec<FileObject>,
}

pub struct Day7 {
    root: Rc<RefCell<DirNode>>,
}

impl Day7 {}

impl FromInput for Day7 {
    fn from_lines(lines: impl Iterator<Item = String>) -> Self {
        let root = Rc::new(RefCell::new(DirNode {
            name: String::from("/"),
            entries: vec![],
        }));
        let mut cwd = root.clone();

        let mut dir_stack = vec![];
        dir_stack.push(root.clone());

        for l in lines {
            if l.starts_with("$") {
                let mut items = l.split(" ");
                let _ = items.next();
                let cmd = items.next().unwrap();

                if cmd == "cd" {
                    let target = items.next().unwrap();
                    if target == ".." {
                        cwd = dir_stack.pop().unwrap();
                    } else if target == "/" {
                        /* we already covered root node */
                    } else {
                        let new_dir = DirNode {
                            name: String::from(target),
                            entries: vec![],
                        };
                        let rf = Rc::new(RefCell::new(new_dir));
                        cwd.borrow_mut().entries.push(FileObject::DIR(rf.clone()));
                        //println!("{}", rf.borrow_mut().name);
                        dir_stack.push(cwd);
                        cwd = rf;
                    }
                }
            } else {
                let mut dir = cwd.borrow_mut();
                let mut items = l.split(" ");
                let first = items.next().unwrap();
                if first == "dir" {
                    continue;
                } else {
                    let fname = items.next().unwrap();
                    let fnode = FileNode {
                        name: String::from(fname),
                        size: usize::from_str_radix(first, 10).unwrap(),
                    };
                    //println!("Adding file {}", fnode.name);
                    let fo = FileObject::FILE(fnode);
                    dir.entries.push(fo);
                }
            }
        }

        Day7 { root: root }
    }
}

impl Day7 {
    fn fo_size(fo: &FileObject) -> usize {
        match fo {
            FileObject::FILE(f) => f.size,
            FileObject::DIR(d) => Day7::dir_size(&d.borrow_mut()),
        }
    }

    fn dir_size(dir: &DirNode) -> usize {
        let mut total: usize = 0;
        for e in &dir.entries {
            total += Day7::fo_size(e);
        }
        total
    }
}

impl DaySolution for Day7 {
    fn part_one(&self) -> String {
        let root = self.root.clone();
        let mut dir_list = vec![];

        for d in &root.borrow_mut().entries {
            dir_list.push(d.clone());
        }

        let mut total = 0;
        while !dir_list.is_empty() {
            let d = dir_list.pop().unwrap();
            match d {
                FileObject::FILE(_) => {}
                FileObject::DIR(dir) => {
                    for e in &dir.borrow_mut().entries {
                        dir_list.push(e.clone());
                    }
                    let s = Day7::dir_size(&dir.borrow_mut());
                    if s < 100000 {
                        total += s;
                    }
                }
            }
        }

        total.to_string()
    }

    fn part_two(&self) -> String {
        let root = self.root.clone();
        let mut dir_list = vec![];
        let mut candidates = vec![];

        for d in &root.borrow_mut().entries {
            dir_list.push(d.clone());
        }

        let used_size = Day7::dir_size(&root.borrow_mut());
        let still_free = 70000000 - used_size;
        let missing = 30000000 - still_free;
        println!("Used {} free {} missing {}", used_size, still_free, missing);

        while !dir_list.is_empty() {
            let d = dir_list.pop().unwrap();
            match d {
                FileObject::FILE(_) => {}
                FileObject::DIR(dir) => {
                    for e in &dir.borrow_mut().entries {
                        dir_list.push(e.clone());
                    }
                    let s = Day7::dir_size(&dir.borrow_mut());
                    if s > missing {
                        candidates.push(s);
                    }
                }
            }
        }

        let mut diff = used_size;
        let mut x = 0;
        for c in candidates {
            if c - missing < diff {
                diff = c - missing;
                x = c;
            }
        }

        x.to_string()
    }
}

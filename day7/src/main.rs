fn main() {
    part1();
    part2();
}

fn part1() {
    println!(
        "{:?}",
        commands(input())
            .heal()
            .sized()
            .linear_fold(vec![], &|mut v, x| if x <= 100000 {
                v.push(x);
                v
            } else {
                v
            })
            .iter()
            .sum::<usize>()
    );
}

fn part2() {
    let total = 70000000;
    let required = 30000000;
    let tree = commands(input()).heal().sized();
    let used = tree.tag();
    let unused = total - used;
    let tofree = required - unused;
    println!(
        "{:?}",
        tree.linear_fold(vec![], &|mut v, x| {
            if x >= tofree {
                v.push(x);
                v
            } else {
                v
            }
        })
        .iter()
        .fold(total, |x, y| x.min(*y))
    );
}

#[derive(Debug)]
enum ChangeDir {
    ChangeDirTo(String),
    ChangeDirToParent,
    ChangeDirToRoot,
}

#[derive(Debug)]
enum Command {
    ChangeDir(ChangeDir),
    List,
}

#[derive(Debug)]
enum TreeF<C> {
    File(String, usize),
    Directory(String, C),
}

impl TreeF<()> {
    fn _fill(&self) -> Tree {
        match self {
            TreeF::File(n, s) => file(n.clone(), *s),
            TreeF::Directory(n, _) => directory(n.clone(), vec![]),
        }
    }
}

impl<T> TreeF<T> {
    fn node_name(&self) -> &String {
        match self {
            TreeF::File(n, _) | TreeF::Directory(n, _) => n,
        }
    }
}

#[derive(Debug)]
enum Tree {
    Tree(TreeF<Vec<Tree>>),
}

impl Tree {
    fn sized(self) -> TaggedTree<usize> {
        match self {
            Tree::Tree(x) => match x {
                TreeF::File(n, s) => TaggedTree::TaggedTree(TreeF::File(n, s), s),
                TreeF::Directory(n, cs) => {
                    let cs1 = cs.into_iter().map(|y| y.sized()).collect::<Vec<_>>();
                    let csize = cs1.iter().map(|t| t.tag()).sum::<usize>();
                    TaggedTree::TaggedTree(TreeF::Directory(n, cs1), csize)
                }
            },
        }
    }
}

type TreeLayer = TreeF<()>;

#[derive(Debug)]
enum TaggedTree<T> {
    TaggedTree(TreeF<Vec<TaggedTree<T>>>, T),
}

impl<T: Copy> TaggedTree<T> {
    fn tag(&self) -> &T {
        match self {
            TaggedTree::TaggedTree(_, t) => t,
        }
    }

    fn linear_fold<A>(&self, init: A, f: &dyn Fn(A, T) -> A) -> A {
        match self {
            TaggedTree::TaggedTree(x, t) => match x {
                TreeF::Directory(_n, cs) => {
                    let init1 = f(init, *t);
                    cs.iter().fold(init1, |a, tree| match tree {
                        TaggedTree::TaggedTree(y, _) => match y {
                            TreeF::Directory(_, _) => tree.linear_fold(a, f),
                            _ => a,
                        },
                    })
                }
                _ => init,
            },
        }
    }
}

#[derive(Debug)]
struct ContextInfo {
    siblings: Vec<Tree>,
    parent_context: Box<Context>,
    name: String,
}

#[derive(Debug)]
enum Context {
    ContextRoot,
    ContextDirectory(ContextInfo),
}

#[derive(Debug)]
struct FocusedTree {
    context: Context,
    focus: Tree,
}

impl Tree {
    fn node_name(&self) -> &String {
        match self {
            Tree::Tree(x) => x.node_name(),
        }
    }
}

fn focused(focus: Tree) -> FocusedTree {
    FocusedTree {
        context: Context::ContextRoot,
        focus,
    }
}

fn heal(context: Box<Context>, focus: Tree) -> Tree {
    match *context {
        Context::ContextRoot => focus,
        Context::ContextDirectory(mut info) => {
            let mut children = Vec::new();
            children.append(&mut info.siblings);
            children.push(focus);
            let new_focus = directory(info.name.clone(), children);
            heal(info.parent_context, new_focus)
        }
    }
}

impl FocusedTree {
    fn heal(self) -> Tree {
        heal(Box::new(self.context), self.focus)
    }

    fn list(self, results: Vec<TreeLayer>) -> Self {
        results.into_iter().fold(self, |ft, x| ft.node(x))
    }

    fn node(self, node: TreeLayer) -> Self {
        match node {
            TreeF::File(n, s) => self.file(n, s),
            TreeF::Directory(n, _) => self.dir(n),
        }
    }

    fn file(self, name: String, size: usize) -> Self {
        let mut res = self;
        if let Tree::Tree(TreeF::Directory(_, cs)) = &mut res.focus {
            if cs.iter().find(|x| *x.node_name() == name).is_none() {
                cs.push(file(name, size))
            }
        } else {
            panic!("Tried to create a node inside a file")
        }
        res
    }

    fn dir(self, name: String) -> Self {
        let mut res = self;
        if let Tree::Tree(TreeF::Directory(_, cs)) = &mut res.focus {
            if cs.iter().find(|x| *x.node_name() == name).is_none() {
                cs.push(directory(name, vec![]))
            }
        } else {
            panic!("Tried to create a directory inside a file")
        }
        res
    }

    fn cd(self, command: ChangeDir) -> FocusedTree {
        match command {
            ChangeDir::ChangeDirTo(dest) => match self.focus {
                Tree::Tree(TreeF::File(_, _)) => self,
                Tree::Tree(TreeF::Directory(name, cs)) => {
                    let (mut elem, others): (Vec<_>, Vec<Tree>) =
                        cs.into_iter().partition(|x| *(x.node_name()) == dest);
                    // println!("{:?}, {:?}", others, elem);
                    let focus = if elem.len() > 0 {
                        elem.swap_remove(0)
                    } else {
                        directory(dest, Vec::new())
                    };
                    FocusedTree {
                        context: Context::ContextDirectory(ContextInfo {
                            siblings: others,
                            parent_context: Box::new(self.context),
                            name,
                        }),
                        focus,
                    }
                }
            },
            ChangeDir::ChangeDirToParent => match self.context {
                Context::ContextRoot => panic!("Cannot CD to the parent of root (/)"),
                Context::ContextDirectory(mut info) => {
                    let context = *info.parent_context;
                    info.siblings.push(self.focus);
                    let focus = directory(info.name.clone(), info.siblings);
                    FocusedTree { context, focus }
                }
            },
            ChangeDir::ChangeDirToRoot => FocusedTree {
                context: Context::ContextRoot,
                focus: heal(Box::new(self.context), self.focus),
            },
        }
    }
}

fn file(name: String, size: usize) -> Tree {
    Tree::Tree(TreeF::File(name, size))
}

fn directory(name: String, children: Vec<Tree>) -> Tree {
    Tree::Tree(TreeF::Directory(name, children))
}

fn input() -> Vec<&'static str> {
    include_str!("../input.txt")[2..]
        // include_str!("../test.txt")[2..]
        .split("\n$ ")
        .collect::<Vec<_>>()
}

fn parse_command(str: String) -> Command {
    match &str[..] {
        "ls" => Command::List,
        "cd /" => Command::ChangeDir(ChangeDir::ChangeDirToRoot),
        "cd .." => Command::ChangeDir(ChangeDir::ChangeDirToParent),
        s => {
            if let Some((a, b)) = s.split_once(" ") {
                if a == "cd" {
                    Command::ChangeDir(ChangeDir::ChangeDirTo(b.to_owned()))
                } else {
                    panic!("Invalid command: {}", str);
                }
            } else {
                panic!("Invalid command: {}", str);
            }
        }
    }
}

fn parse_node(str: String) -> TreeLayer {
    let (a, n) = str.split_once(" ").unwrap();
    if a == "dir" {
        TreeF::Directory(n.to_owned(), ())
    } else {
        TreeF::File(n.to_owned(), a.parse::<usize>().unwrap())
    }
}

fn commands(strs: Vec<&str>) -> FocusedTree {
    let tree = focused(directory("/".to_owned(), vec![]));
    strs.into_iter().fold(tree, |t, s| {
        let cmdout = s.lines().collect::<Vec<&str>>();
        let cmd = *cmdout.first().unwrap();
        match parse_command(cmd.to_owned()) {
            Command::ChangeDir(c) => t.cd(c),
            Command::List => {
                let results = cmdout[1..]
                    .iter()
                    .map(|x| parse_node((*x).to_owned()))
                    .collect::<Vec<_>>();
                t.list(results)
            }
        }
    })
}

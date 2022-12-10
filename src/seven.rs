use std::cell::{Ref, RefCell, RefMut};
use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::rc::Rc;

type Input = Vec<String>;

#[derive(Debug)]
enum ConsoleLine {
  CD(String),
  LS,
  Directory(String),
  File(String, usize),
}

impl ConsoleLine {
  fn from_line(l: &String) -> Self {
    let chunks: Vec<&str> = l.split(' ').collect();
    match chunks[0] {
      "$" => match chunks[1] {
        "cd" => ConsoleLine::CD(chunks[2].to_owned()),
        "ls" => ConsoleLine::LS,
        _ => unreachable!(),
      },
      "dir" => ConsoleLine::Directory(chunks[1].to_owned()),
      size => {
        let s: usize = size.parse().unwrap();
        let name: String = chunks[1].to_owned();
        ConsoleLine::File(name, s)
      }
    }
  }
}

type NodePointer = Rc<RefCell<Node>>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum NodeType {
  Folder,
  File,
}

struct Node {
  parent: Option<NodePointer>,
  size: Option<usize>,
  name: String,
  node_type: NodeType,
  children: HashMap<String, NodePointer>,
}

impl Node {
  fn create_root() -> Self {
    Self {
      parent: None,
      size: None,
      name: "/".to_owned(),
      node_type: NodeType::Folder,
      children: Default::default(),
    }
  }
}

fn print_path(node: &NodePointer) {
  let mut path: Vec<String> = vec![];
  let mut current = node.clone();
  path.push(current.borrow().name.clone());
  let mut hasParent = current.borrow().parent.is_some();
  while hasParent {
    current = {
      let current_node = current.borrow();
      let p = current_node.parent.as_ref().unwrap();
      path.push(p.borrow().name.clone());
      p.clone()
    };
    hasParent = current.borrow().parent.is_some();
  }

  let path_str: String = path
    .iter()
    .rev()
    .map(|s| s.to_owned())
    .collect::<Vec<String>>()
    .join("/");
  println!("{path_str}")
}

fn calculate_folder_size(np: &NodePointer) {
  let node: Ref<Node> = np.borrow();
  let total: usize = node
    .children
    .iter()
    .map(|(_k, v)| {
      let size_opt = { v.borrow().size };
      if size_opt.is_none() {
        calculate_folder_size(&v)
      }
      let size = { v.borrow().size.unwrap() };
      size
    })
    .sum();
  drop(node);
  let mut node: RefMut<Node> = np.borrow_mut();
  node.size = Some(total);
}

fn get_dir_list(root: &NodePointer) -> Vec<NodePointer> {
  let mut children_folders: Vec<NodePointer> = root
    .borrow()
    .children
    .iter()
    .filter(|(_k, v)| v.borrow().node_type == NodeType::Folder)
    .map(|(_k, v)| get_dir_list(v))
    .flatten()
    .collect();
  children_folders.push(root.clone());
  children_folders
}

fn read_data() -> Input {
  let filename = format!("./resources/7.txt");
  let file: File = File::open(&filename).expect(&format!("Cannot open file {}", &filename));
  let reader = BufReader::new(file);
  let line_iter = reader.lines();
  line_iter.map(|r| r.unwrap()).collect()
}

fn solve(input: Input) -> usize {
  let root: NodePointer = Rc::new(RefCell::new(Node::create_root()));
  let mut current: NodePointer = root.clone();
  for cmd_line in input {
    let cmd = ConsoleLine::from_line(&cmd_line);
    match cmd {
      ConsoleLine::CD(target) => match target.as_str() {
        "/" => {
          current = root.clone();
        }
        ".." => {
          let prev: NodePointer = current.borrow().parent.as_ref().unwrap().clone();
          current = prev;
        }
        other => {
          let next = current.borrow().children.get(other).unwrap().clone();
          current = next
        }
      },
      ConsoleLine::Directory(dir_name) => {
        let n = Node {
          parent: Some(current.clone()),
          size: None,
          name: dir_name.clone(),
          node_type: NodeType::Folder,
          children: Default::default(),
        };
        let n_ref = Rc::new(RefCell::new(n));

        let mut cnode: RefMut<_> = current.borrow_mut();
        cnode.children.insert(dir_name.clone(), n_ref);
      }
      ConsoleLine::File(file_name, size) => {
        let n = Node {
          parent: Some(current.clone()),
          size: Some(size),
          name: file_name.clone(),
          node_type: NodeType::File,
          children: Default::default(),
        };
        let n_ref = Rc::new(RefCell::new(n));

        let mut cnode: RefMut<_> = current.borrow_mut();
        cnode.children.insert(file_name.clone(), n_ref);
      }
      ConsoleLine::LS => {
        // print_path(&current)
      }
    }
  }
  calculate_folder_size(&root);
  let all = get_dir_list(&root);
  all
    .iter()
    .map(|n| n.borrow().size.unwrap())
    .filter(|&s| s <= LIMIT)
    .sum()
}

const LIMIT: usize = 100000;

pub fn seven() {
  let input = read_data();
  let score = solve(input);
  println!("{score}")
}

#[cfg(test)]
mod tests {

  #[test]
  fn simple() {
    assert!(true);
  }
}

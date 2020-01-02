const MOVE_COST: i32 = 10;

#[derive(Copy, Clone)]
struct Node {
  index: i32,
  parent_index: i32,
  traversable: bool,
  x: i32,
  y: i32,
  g: i32,
  h: i32,
  f: i32,
}

impl Node {
  fn new(index: i32, x: i32, y: i32) -> Node {
    Node {
      index: index,
      parent_index: -1,
      traversable: true,
      x: x,
      y: y,
      g: 0,
      h: 0,
      f: 0,
    }
  }

  fn set_h(&mut self, goal: Node) {
    if !self.traversable {
      return;
    }

    self.h = ((self.x - goal.x).abs() + (self.y - goal.y).abs()) * MOVE_COST
  }

  fn set_g_f(&mut self, parent: Node) {
    self.g = parent.g + MOVE_COST;
    self.f = self.g + self.h;
  }
}

impl PartialEq for Node {
  fn eq(&self, other: &Self) -> bool {
    self.index == other.index
  }
}

struct Grid {
  width: i32,
  height: i32,
  nodes: Vec<Node>,
}

impl Grid {
  fn new(width: i32, height: i32, walls: Vec<i32>) -> Grid {
    let mut nodes = Vec::new();
    let mut index = 0;

    for y in 0..height {
      for x in 0..width {
        let mut node: Node = Node::new(index, x as i32, y as i32);

        if walls.contains(&index) {
          node.traversable = false;
        }

        nodes.push(node);
        index += 1;
      }
    }

    Grid {
      width: width,
      height: height,
      nodes: nodes,
    }
  }

  fn get_nodes(&self) -> Vec<Node> {
    let mut nodes: Vec<Node> = Vec::new();

    for node in self.nodes.iter() {
      nodes.push(*node);
    }

    nodes
  }

  fn get_width(&self) -> i32 {
    self.width
  }

  fn get_neighbours(&self, node: Node) -> Vec<Node> {
    let mut neighbours: Vec<Node> = Vec::new();

    // left check
    if node.x - 1 >= 0 {
      let index = node.y * self.width + node.x - 1;
      if self.nodes[index as usize].traversable {
        neighbours.push(self.nodes[index as usize]);
      }
    }

    // right check
    if node.x + 1 < self.width {
      let index = node.y * self.width + node.x + 1;
      if self.nodes[index as usize].traversable {
        neighbours.push(self.nodes[index as usize]);
      }
    }

    // up check
    if node.y - 1 >= 0 {
      let index = (node.y - 1) * self.width + node.x;
      if self.nodes[index as usize].traversable {
        neighbours.push(self.nodes[index as usize]);
      }
    }

    // down check
    if node.y + 1 < self.height {
      let index = (node.y + 1) * self.width + node.x;
      if self.nodes[index as usize].traversable {
        neighbours.push(self.nodes[index as usize]);
      }
    }

    return neighbours;
  }
}

fn a_star(grid: Grid, mut start: Node, goal: Node) -> Vec<Node> {
  let mut open_nodes: Vec<Node> = vec![start];
  let mut closed_nodes: Vec<Node> = Vec::new();
  let mut current_node: Node;

  start.set_h(goal);
  start.f = start.g + start.h;

  while open_nodes.len() > 0 {
    open_nodes.sort_by(|x, y| x.f.cmp(&y.f));

    current_node = open_nodes.swap_remove(0);
    closed_nodes.push(current_node);

    if current_node.index == goal.index {
      return get_path(closed_nodes, current_node);
    }

    for neighbour in grid.get_neighbours(current_node).iter_mut() {
      neighbour.parent_index = current_node.index;

      if !closed_nodes.contains(neighbour) {
        neighbour.set_g_f(current_node);

        if open_nodes.contains(neighbour) {
          let mut open_neighbour: Node = Node::new(neighbour.index, neighbour.x, neighbour.y);
          for node in open_nodes.iter() {
            if node == neighbour {
              open_neighbour = *node;
            }
          }

          if neighbour.g < open_neighbour.g {
            open_neighbour.g = neighbour.g;
            open_neighbour.parent_index = neighbour.parent_index;
          }
        } else {
          open_nodes.push(*neighbour);
        }
      }
    }
  }

  Vec::new()
}

fn get_path(closed_nodes: Vec<Node>, mut node: Node) -> Vec<Node> {
  let mut path: Vec<Node> = vec![node];

  while node.parent_index != -1 {
    for closed_node in closed_nodes.iter() {
      if closed_node.index == node.parent_index {
        node = *closed_node;
      }
    }

    path.push(node);
  }

  path.reverse();
  path
}

fn main() {
  let grid: Grid = Grid::new(7, 7, vec![5, 10, 12, 17, 19, 26, 31, 38, 40, 45, 47]);
  let nodes = grid.get_nodes();
  let width = grid.get_width();
  let path: Vec<Node> = a_star(grid, nodes[42], nodes[6]);

  print!("Path: ");
  for (index, node) in path.iter().enumerate() {
    if index == path.len() - 1 {
      print!("{}\n", node.index);
    } else {
      print!("{} -> ", node.index);
    }
  }
  println!("Cost: {}\n", path.len() * 10);

  for node in nodes.iter() {
    if node.traversable {
      print!(
        "{:<3}{}",
        if path.contains(node) { '\u{25CF}' } else { '-' },
        if node.x == width - 1 { "\n" } else { "" }
      );
    } else {
      print!(
        "{:<3}{}",
        '\u{2588}',
        if node.x == width - 1 { "\n" } else { "" }
      );
    }
  }
}

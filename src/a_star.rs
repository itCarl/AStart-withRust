use std::collections::BinaryHeap;
use std::cmp::Ordering;
use rand::Rng;

pub struct Cell {
    obstacle: bool,
    visited: bool,
    path: bool,
    x: i32,
    y: i32,
    f: f64,
    g: f64,
    h: f64,
    neighbours: Vec<[i32;2]>,
    parent: [i32;2],
}

#[derive(PartialEq)]
struct Node {
    f: f64,
    x: i32,
    y: i32,
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.f.partial_cmp(&self.f)
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Eq for Node {
    fn assert_receiver_is_total_eq(&self) {}
}

const N: i32 = 17;
const M: i32 = 17;

pub fn a_star_search() -> Vec<Vec<Cell>> {
    let mut adjacency_list: Vec<Vec<Cell>> = generate_adjacency_list();
    let mut closed_list: Vec<Vec<bool>> = generate_closed_list();
    let mut open_list: BinaryHeap<Node> = BinaryHeap::<Node>::new();

    let start_node: Node = Node{ f: 0.0, x: 0, y: 0 };
    let destination_node: Node = Node {f: f64::MAX, x: 16, y: 16};

    adjacency_list[0][0].f = 0.0;
    adjacency_list[0][0].g = 0.0;
    adjacency_list[0][0].h = 32.0;
    adjacency_list[0][0].parent = [start_node.x, start_node.y];
    
    open_list.push(start_node);

    while !open_list.is_empty() {
        let current_node: Node = open_list.pop().unwrap();

        closed_list[current_node.x as usize][current_node.y as usize] = true;

        let mut g_new: f64;
        let mut h_new: f64;
        let mut f_new: f64;
        
        let neighbours: Vec<[i32; 2]> = adjacency_list[current_node.x as usize][current_node.y as usize].neighbours.clone();
        
        for neighbour in neighbours {
            if neighbour == [destination_node.x, destination_node.y] {
                println!("Destination found");
                adjacency_list[neighbour[0] as usize][neighbour[1] as usize].f = 32.0;
                adjacency_list[neighbour[0] as usize][neighbour[1] as usize].g = 32.0;
                adjacency_list[neighbour[0] as usize][neighbour[1] as usize].h = 0.0;
                adjacency_list[neighbour[0] as usize][neighbour[1] as usize].parent = [current_node.x, current_node.y];
                let path: Vec<[i32; 2]> = generate_path(&adjacency_list, &destination_node, [0,0]);
                for x in &path {
                    println!("{:?}", x);
                    adjacency_list[x[0] as usize][x[1] as usize].path = true;
                }
                // print_adjacency_list(&adjacency_list);
                //print_parents(&adjacency_list);
                return adjacency_list;
            }
            if closed_list[neighbour[0] as usize][neighbour[1] as usize] == true{
                continue;
            }
            if adjacency_list[neighbour[0] as usize][neighbour[1] as usize].obstacle {
                closed_list[neighbour[0] as usize][neighbour[1] as usize] = true;
                continue;
            }
            
            g_new = adjacency_list[current_node.x as usize][current_node.y as usize].g + 1.0;
            h_new = calculate_h_value(neighbour[0], neighbour[1], &destination_node);
            f_new = g_new + h_new;

            let adj_f = adjacency_list[neighbour[0] as usize][neighbour[1] as usize].f.clone();

            if adj_f == f64::MAX || adj_f > f_new {
                adjacency_list[neighbour[0] as usize][neighbour[1] as usize].f = f_new;
                adjacency_list[neighbour[0] as usize][neighbour[1] as usize].g = g_new;
                adjacency_list[neighbour[0] as usize][neighbour[1] as usize].h = h_new;
                open_list.push( Node {f: f_new, x: neighbour[0], y: neighbour[1]});
                adjacency_list[neighbour[0] as usize][neighbour[1] as usize].parent = [current_node.x, current_node.y];
            }
            closed_list[neighbour[0] as usize][neighbour[1] as usize] = true
        }
    }
    panic!()
}

fn generate_path(adjacency_list: &Vec<Vec<Cell>>, destination_node: &Node, start_node: [i32;2]) -> Vec<[i32;2]> {
    /*let mut path: Vec<[i32;2]> = Vec::new();
    path.push([destination_node.x, destination_node.y]);
    let mut start: [i32; 2] = adjacency_list[destination_node.x as usize][destination_node.y as usize].parent;
    while start != [start_node[0], start_node[1]] {
        path.push(start);
        start = adjacency_list[start[0] as usize][start[1] as usize].parent;
    }*/

    let mut path: Vec<[i32;2]> = Vec::new();
    let mut current = [destination_node.x, destination_node.y];
    path.push(current);

    while current != start_node
    {
        print!("Current Node: {:?}, f: {}, g: {}, h: {}",
            current, 
            adjacency_list[current[0] as usize][current[1] as usize].f,
            adjacency_list[current[0] as usize][current[1] as usize].g,
            adjacency_list[current[0] as usize][current[1] as usize].h
        );
        println!();

        // let neighbours = adjacency_list[current[0] as usize][current[1] as usize].neighbours.clone();
        for neighbour in &adjacency_list[current[0] as usize][current[1] as usize].neighbours
        {
            print!("Available neighbours: {:?}, f: {}, g: {}, h: {}",
                neighbour, 
                adjacency_list[neighbour[0] as usize][neighbour[1] as usize].f,
                adjacency_list[neighbour[0] as usize][neighbour[1] as usize].g,
                adjacency_list[neighbour[0] as usize][neighbour[1] as usize].h
            );
            println!();
            
            if (&adjacency_list[neighbour[0] as usize][neighbour[1] as usize].g < &adjacency_list[current[0] as usize][current[1] as usize].g) {
                print!("Chosen neighbour: {:?}", neighbour,);
                path.push(*neighbour);
                current = *neighbour;
                println!();
                break;
            }
        }
        println!();
    }
    return path;
}


fn generate_obstacle() -> bool {
    let mut rng: rand::rngs::ThreadRng = rand::thread_rng();
    let y: f64 = rng.gen();
    if y > 0.90 {
        return true;
    } else {
        return false;
    }
}

fn generate_adjacency_list() -> Vec<Vec<Cell>>{
    let mut adjacency_list: Vec<Vec<Cell>> = Vec::new();
    for x in 0..M {
        let mut row: Vec<Cell> = Vec::new();
        for y in 0..N {
            let mut neighbours: Vec<[i32;2]> = Vec::new();
            
            //Left, right, top and bottom neighbours
            if x - 1 >= 0 {
                neighbours.push([x-1, y])
            }
            if x + 1 <= N-1 {
                neighbours.push([x+1, y])
            }
            if y - 1 >= 0 {
                neighbours.push([x, y-1])
            }
            if y + 1 <= M-1 {
                neighbours.push([x, y+1])
            }

            let obstacle_gen = generate_obstacle();
            let cell: Cell = Cell { 
                obstacle: obstacle_gen, 
                visited: false,
                path: false,
                x, y,
                f: f64::MAX,
                g: f64::MAX, 
                h: f64::MAX, 
                neighbours: neighbours, 
                parent: [0, 0]
            };
            row.push(cell);
        }
        adjacency_list.push(row);
    }
    return adjacency_list;
}

fn generate_closed_list() -> Vec<Vec<bool>> {
    let mut closed_list: Vec<Vec<bool>> = Vec::new();
    for _y in 0..M {
        let mut row: Vec<bool> = Vec::new();
        for _x in 0..N {
            row.push(false);
        }
        closed_list.push(row);
    }
    closed_list
}

fn calculate_h_value(x: i32, y: i32, destination_node: &Node) -> f64 {
    //let number = (x-destination_node.x) * (x-destination_node.x) + (y-destination_node.y) * (y-destination_node.y);
    //f64::sqrt(number as f64)
    let number = (x-destination_node.x).abs() + (y-destination_node.y).abs();
    return number as f64;
}

pub fn get_path(adjacency_list: &Vec<Vec<Cell>>) -> Vec<[i32;2]> {
    let mut path: Vec<[i32; 2]> = Vec::new();
    for x in adjacency_list{
        for y in x {
            if y.path {
                path.push([y.x, y.y]);
            }
        }
    }
    return path;
}

// pub fn get_explored(adjacency_list: &Vec<Vec<Cell>>) -> Vec<[i32;2]> {
//     let mut explored: Vec<[i32;2]> = Vec::new();
//     for x in adjacency_list{
//         for y in x{
//             if y.visited{
//                 explored.push([y.x, y.y]);
//             }
//         }
//     }
//     return explored;
// }

pub fn get_obstacles(adjacency_list: &Vec<Vec<Cell>>) -> Vec<[i32;2]> {
    let mut obstacles: Vec<[i32; 2]> = Vec::new();
    for x in adjacency_list{
        for y in x {
            if y.obstacle {
                obstacles.push([y.x, y.y]);
            }
        }
    }
    return obstacles;
}

pub fn get_values(adjacency_list: &Vec<Vec<Cell>>) -> Vec<[f64;3]>{
    let mut values: Vec<[f64;3]> = Vec::new();
    for x in adjacency_list {
        for y in x {
            values.push([y.f, y.g, y.h]);
        }
    }
    return  values;
}


fn print_adjacency_list(adjacency_list: &Vec<Vec<Cell>>){
    for n in 0..adjacency_list.len() {
        for m in 0..adjacency_list[n].len() {
            println!("[{}, {}] -> {:?} ", adjacency_list[n][m].x, adjacency_list[n][m].y, adjacency_list[n][m].neighbours);
            //print!("({},{})", closed_List[n][m].x, closed_List[n][m].y);
        }
        println!();
    }
}

// fn print_closed_list(closed_list: Vec<Vec<bool>>) {
//     for n in 0..closed_list.len() {
//         for m in 0..closed_list[n].len() {
//             if closed_list[n][m] {
//                 print!("8")
//             }
//             else {
//                 print!("0")
//             }
//             //println!("[{}, {}] -> {:?} ", n, m, closed_list[n][m]);
//             //print!("({},{})", closed_List[n][m].x, closed_List[n][m].y);
//         }
//         println!();
//     }
// }


// fn print_parents(adjacency_list: &Vec<Vec<Cell>>){
//     for x in adjacency_list {
//         for y in x {
//             println!("[{}, {}] => {:?}", y.x, y.y, y.parent);
//         }
//     }
// }
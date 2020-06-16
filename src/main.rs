extern crate scanner_rust;

extern crate petgraph;

use std::io::{self, Write};

use petgraph::graph::Graph;
use scanner_rust::ScannerAscii;

fn main() {
    print!("Please input two integers, a and b: ");
    io::stdout().flush().unwrap();

    let mut sc = ScannerAscii::new(io::stdin());

    let n = read_int(&mut sc);

    // println!("{}", n);

    // simply bug
    // baca new line
    // println!("{}", read_str(&mut sc));
    // println!("ckckck2 {}", read_str(&mut sc));

    let mut roads = Vec::new();
    for _ in 0..n - 1 {
        // println!("{}", read_str(&mut sc));
        // let roads_row_items: Vec<usize> = read_str(&mut sc)
        //                     .split(" ")
        //                     .map(|s| { println!("{}", s); s.parse::<usize>().unwrap()} )
        //                     .collect();
        roads.push(vec![read_int(&mut sc), read_int(&mut sc)]);
    }

    kingdom_division(n, &roads)
}

fn kingdom_division(n: usize, roads: &Vec<Vec<usize>>) {
    let mut deps = Graph::<usize, usize>::new();
    let mut nodes = Vec::new();

    for i in 0..=n {
        nodes.push(deps.add_node(i));
    }

    for road_row_item in roads.iter() {
        deps.add_edge(nodes[road_row_item[0]], nodes[road_row_item[1]], 1);
        deps.add_edge(nodes[road_row_item[1]], nodes[road_row_item[0]], 1);
    }

    // println!("{:?}", deps);

    // reset
    const PARENT_SIZE: usize = 100001;
    let mut parent = [0; PARENT_SIZE];

    // dfs
    use petgraph::visit::Dfs;
    let mut dfs = Dfs::new(&deps, nodes[1]);
    while let Some(nx) = dfs.next(&deps) {
        // println!("current node {:?} with stack {:?}", nx, dfs.stack);
        for child in &dfs.stack {
            if parent[child.index()] == 0 {
                parent[child.index()] = nx.index();
            }
        }
    }

    // print parent for each node
    // for i in 0..=n {
    //     println!("{:?}", parent[i]);
    // }

    use std::i64::MAX;
    let mut dp = vec![vec![vec![MAX; M]; N]; P];

    // println!("{}", leaf(&deps, &parent, 3));

    
    println!("{}", (solve_for(&deps, &parent, &mut dp, 1, 0, 1) * 2)%MOD);
}

const P: usize = 100001;
const N: usize = 2;
const M: usize = 3;

fn solve_for(
    deps: &Graph<usize, usize>,
    parent: &[usize],
    dp: &mut Vec<Vec<Vec<i64>>>,
    root: usize,
    color: usize,
    streak: usize, // current node's parent same as the color of current node
) -> i64 {
    println!("enter {} {} {}", root, color, streak);
    use std::i64::MAX;
    if dp[root][color][streak] != MAX {
        return dp[root][color][streak];
    }

    let color_invert = |x| {
        if x == 0 {
            x + 1
        }else {
            x -1
        }
    };

    if leaf(deps, parent, root) {
        if streak == 1 { // different
            dp[root][color][streak] = 0;
            return dp[root][color][streak];
        }
        if streak == 2 { // same 
            dp[root][color][streak] = 1;
            return dp[root][color][streak];
        }
    } 

    let mut ans = 1i64;
    let mut valid = 1i64;
    let mut invalid = 1i64;

    use petgraph::graph::NodeIndex;
    for node in deps.neighbors(NodeIndex::new(root)) {
        if node.index() != parent[root] {
            valid=0;
            valid+=solve_for(deps, parent, dp, node.index(), color_invert(color),1);
            invalid=(invalid*solve_for(deps, parent, dp, node.index(), color_invert(color),1))%MOD;
            valid+=solve_for(deps, parent, dp, node.index(),color,2);
            ans=(ans*valid)%MOD;
\
            println!("{:?} color {} streak {} valid {} invalid {} ans {}",node, color,streak,valid,invalid, ans);
            // println!("{:?} ans {}", node, ans);
        }
        // println!("{:?}", node);
    }

    println!("root {} color {} streak {} valid {} invalid {} ans {}",root, color,streak,valid,invalid, ans);

    if streak==1 {
        ans=(ans-invalid+MOD)%MOD;
    }

    dp[root][color][streak]=ans;

    dp[root][color][streak]
}

const MOD : i64 =  1000000007;

fn leaf(deps: &Graph<usize, usize>, parent: &[usize], node: usize) -> bool {
    use petgraph::graph::NodeIndex;
    let mut l = true;
    for nei in deps.neighbors(NodeIndex::new(node)) {
        if nei.index() != parent[node]  {
            l = false
        }
        // println!("{:?}", nei);
    }
    l
}

use std::io::Read;

fn read_int<T>(sc: &mut ScannerAscii<T>) -> usize
where
    T: Read,
{
    return loop {
        match sc.next_usize() {
            Ok(i) => break i.unwrap_or(0),
            Err(_) => {
                print!("Re-input a and b: ");
                io::stdout().flush().unwrap();
            }
        }
    };
}

// there is a bug in here
// fn read_str<T> (sc : &mut  ScannerAscii<T>) -> String
//     where T : Read{
//         return loop {
//             match sc.next_line() {
//                 Ok(i) => break i.unwrap_or("kakakaak".to_owned()),
//                 Err(_) => {
//                     print!("Re-input a and b: ");
//                     io::stdout().flush().unwrap();
//                 }
//             }
//         }
// }

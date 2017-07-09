use std::fmt;

#[derive(Clone)] // give clone trait to this struct
struct Edge {
    // id: unique edge identifier
    // entry: entry point of the edge
    // exit: exit point of the edge
    id: u8, 
    entry: u8,
    exit: u8,
    weight: u32,
}

impl fmt::Display for Edge {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Edge {} ({} -> {}  w{})", self.id, self.entry, self.exit, self.weight)
    }
}

#[derive(Clone)]
struct Path { // path that holds several edges
    edges: Vec<Edge>,
    weight: u32,
    edge_ids: Vec<u8>,
}

impl Path {
    fn append(&mut self, edge: Edge) {
        self.edges.push(edge.clone());
        self.weight += edge.weight;
        self.edge_ids.push(edge.id);
    }
}



fn print_edge(edge: &Edge) {
    // print edge info to stdout
    return println!("{}", edge);
}

fn print_edge_vector(edge_vector: &Vec<Edge>) {
    // print all edge info of vector
    for edge in edge_vector {
        print!("{}", edge);
    }
}

fn verify_edges(edges: &Vec<Edge>)
{
    let mut edge_ids = Vec::new();
    for edge in edges {
        let id = edge.id;
        if edge_ids.contains(&id) {
            println!("ERROR: 2 edges with identical ID found: {}", id);
            std::process::exit(2);
        } else {
            edge_ids.push(id);
        }
    }
}

fn prune_edges(available_edges: &Vec<Edge>, start_floor: u8, end_floor: u8) -> Vec<Edge> {
    // prune edges that are unreachable or dead ends
    // fn does only as single pass!
    let mut exits = Vec::new();
    let mut entries = Vec::new();

    // make sure we don't have several edges with same id
    verify_edges(&available_edges);

    for edge in available_edges { // collect entries and exits
        if !exits.contains(&edge.exit) {
            exits.push(edge.exit);
        }
        if !entries.contains(&edge.entry) {
            entries.push(edge.entry);
        }
    }

    let mut usable_edges = Vec::new();

    for edge in available_edges {
        // make sure we don't remove our root and goal edge
        if !entries.contains(&((edge).exit)) && (edge).exit != end_floor  { // remove deadends
            println!("pruning dead end:    {}", edge);
        } else if !exits.contains(&((edge).entry)) && (edge).entry != start_floor { // remove unreachable
            println!("pruning unreachable: {}", edge);
        } else { // edge is neither dead end nor unreachable and thus usable
            usable_edges.push(edge.clone());
        }
    }
    return usable_edges;
}


fn prune_edges_recursively(edges: Vec<Edge>, start_floor: u8, end_floor: u8) -> Vec<Edge> {
    // prune edges recursively until we cannot prune any further
    println!("\n ===  pruning  ===");
    let mut a;
    let mut b;
    let mut c = edges;
    loop { // prune until failure
        a  = prune_edges(&c, start_floor, end_floor);
        b  = prune_edges(&a, start_floor, end_floor);
        if a.len() == b.len() { // before == after
            c = b;
            break;
        } else {
//        println!("a.len: {}  b.len: {}", a.len(), b.len());
            c = b;
        }
    }
    return c;
}

fn get_possible_new_connections(edge: &Edge, purged_edges: &Vec<Edge>) -> Vec<Edge> {
    let mut connection_vec = Vec::new();
    for poss_conn in purged_edges {
        if edge.exit == (&poss_conn).entry { // collect possible new connections
            connection_vec.push(poss_conn.clone()); 
        }
    }
    //println!("possible new connections for {}", get_edge_str(&edge));
    //print_edge_vector(&connection_vec);
    //println!("\n");
    return connection_vec;
}


fn print_shortest_paths(start_floor: u8, end_floor: u8, edgevec: Vec<Edge>) {
    // meh...
    #[allow(non_snake_case)]
    let  START_FLOOR = start_floor;
    #[allow(non_snake_case)]
    let  END_FLOOR = end_floor; 

    let edges = edgevec; // immutable now
    println!("Current edges: ");
        print_edge_vector(&edges); 

    // prune
    let purged_edges = prune_edges_recursively(edges, START_FLOOR, END_FLOOR);

    if purged_edges.len() == 0 {
        println!("No edges left to traverse!");
        std::process::exit(1);
    }

    println!("\nRemaining edges:");
        print_edge_vector(&purged_edges); // obtain paths

    let mut walked_edges = Vec::new(); // store numbers/ids of walked edges in this vector, 

    let mut vector_of_paths = Vec::new(); // store paths in this vector, this will be a vector of vectors


    println!("\nSearching for paths...");
    // find entry paths
    let mut initial_entries = Vec::new();
    for edge in &purged_edges {
        if edge.entry == START_FLOOR { // possible starting points
            println!("We can start at {}", edge);
            initial_entries.push(edge);
        }
    }
    let initial_entries = initial_entries; // make immutable


    // [vec1]  [vec2]  [vec3] ... ] => vector_of_paths
    //   ↓        ↓       ↓
    //  edge1   edge1   edge2
    //   ↓        ↓       ↓
    //  edge5   edge18  edge4 
    //   ...      ...
    for start_edge in initial_entries {
        let mut path_vect = Vec::new();
        path_vect.push((start_edge).clone()); // start a new path_vector
        walked_edges.push(start_edge.id); // mark edge as traversed
        vector_of_paths.push(path_vect.clone()); // save new path vector to VoP
    }
    'iterative_pathfinding_loop: loop {

        let mut vector_of_paths_tmp: Vec<Vec<Edge>> = Vec::new(); // vector containing vectors containing edges
        for subpath in &vector_of_paths {
            // collect ids of edges in this subpath
            let mut subpath_edge_ids = Vec::new();
            for edge in subpath {
                subpath_edge_ids.push(edge.id);
            }
            let last_edge_of_subpath = subpath.last().unwrap(); // get last edge of the subpath

            // and find new connections
            let new_conns = get_possible_new_connections(&last_edge_of_subpath, &purged_edges);
            //println!("found new connections: {}", new_conns.len());

            if new_conns.len() > 0 { // we have new connections
                for new_connection in new_conns.iter() {
                    if subpath_edge_ids.contains(&(new_connection.id)) { // avoid hang in circular paths (path( 5 -> 10) ; path(10 -> 5)
                        continue;
                    }
                    //println!("possible new connection: {}", get_edge_str(&new_connection));
                    let mut subpath_tmp = subpath.clone(); // clone current subpath
                    subpath_tmp.push(new_connection.clone()); // and append edge
                    vector_of_paths_tmp.push(subpath_tmp.clone()); // add the new subpath to the new vector
                    walked_edges.push(new_connection.id); // save edge nr as walked

                }
            } else { // we have no new connections, clone subpath anyway so it doesnt get dropped
                vector_of_paths_tmp.push(subpath.clone());
            }
        }

        vector_of_paths = vector_of_paths_tmp.clone(); // make vector_of_paths_tmp available in next loop iteration

        let mut break_loop = true;
        for subvector in &vector_of_paths {
            if subvector.last().unwrap().exit != END_FLOOR { // if we have one subpath end edge that has not reached end yet
                break_loop = false;                          // we must continue searching
            }
        } 
        if break_loop {
            //println!("breaking search loop");
            break 'iterative_pathfinding_loop;
        }

    } // iterative_path_finding_loop


    // debugging:
    print!("Walked edges:");
    for nr in &walked_edges {
        print!(" {}", nr);
    }
    println!();
    // print base vector:
    println!("Printing Vector of Paths");
    let mut it=0;
    for subpath in &vector_of_paths {
        it +=1;
        println!("\tsubpath {}", it);
        for edge in subpath  {
            println!("\t\t{}", edge);
        }
    }
    println!("\nshortest path(s) from {} to {}:", START_FLOOR, END_FLOOR);
    let mut shortes_paths = Vec::new();
    let mut index = 0; // dont add first index twice
    shortes_paths.push(vector_of_paths.first().unwrap());
    for subpath in &vector_of_paths {
        if subpath.len() < shortes_paths[0].len() { // found smaller path
            shortes_paths.clear();
            shortes_paths.push(subpath);
        } else if subpath.len() == shortes_paths[0].len() && index != 0 { // found path of equal size, don't add first path twice if it is the shortest
            shortes_paths.push(subpath);
        }
        index += 1;
    }
    for subpath in shortes_paths {
        for edge in subpath {
            println!("{}", edge);
        }
        println!("====");
    }
}





fn test_matthiaskrgr() {
    // Testing

    // path edges
    let edge_1 = Edge {id: 1, entry: 0, exit: 5, weight: 1};
    let edge_2 = Edge {id: 2, entry: 5, exit: 10, weight: 1};
    let edge_3 = Edge {id: 3, entry: 5, exit: 7, weight: 1};
    let edge_4 = Edge {id: 4, entry: 7, exit: 10, weight: 1};
    let edge_5 = Edge {id: 5, entry: 6, exit: 10, weight: 1}; // unreachable
    let edge_6 = Edge {id: 6, entry: 7, exit: 11, weight: 1}; // dead end after 7 is pruned
    let edge_7 = Edge {id: 7, entry: 11, exit: 20, weight: 1}; // dead end chain
    let edge_8 = Edge {id: 8, entry: 20, exit: 25, weight: 1};
    let edge_9 = Edge {id: 9, entry: 25, exit: 26, weight: 1};
    let edge_10 = Edge {id: 10, entry: 26, exit: 27, weight: 1};
    let edge_11 = Edge {id: 11, entry: 50, exit: 10, weight: 1}; // unreachable chain
    let edge_12 = Edge {id: 12, entry: 49, exit: 50, weight: 1};
    let edge_13 = Edge {id: 13, entry: 48, exit: 59, weight: 1};
    let edge_14 = Edge {id: 14, entry: 0, exit: 100, weight: 1}; // 0 -> 100
    let edge_15 = Edge {id: 15, entry: 100, exit: 10, weight: 1}; // 100 -> 10 // goal
    let edge_16 = Edge {id: 16, entry: 5, exit: 9, weight: 1}; 
    let edge_17 = Edge {id: 17, entry: 9, exit: 200, weight: 1}; 
    let edge_18 = Edge {id: 18, entry: 200, exit: 10, weight: 1};
    let edge_19 = Edge {id: 19, entry: 7, exit: 5, weight: 1}; // 7 -> 7, 5 -> 7 circular loop


    // move edges into vector
    let mut edgevec = Vec::new(); // hold all the edges
    edgevec.push(edge_1);
    edgevec.push(edge_2);
    edgevec.push(edge_3);
    edgevec.push(edge_4);
    edgevec.push(edge_5);
    edgevec.push(edge_6);
    edgevec.push(edge_7);
    edgevec.push(edge_8);
    edgevec.push(edge_9);
    edgevec.push(edge_10);
    edgevec.push(edge_11);
    edgevec.push(edge_12);
    edgevec.push(edge_13);
    edgevec.push(edge_14);
    edgevec.push(edge_15);
    edgevec.push(edge_16);
    edgevec.push(edge_17);
    edgevec.push(edge_18);
    edgevec.push(edge_19);

    let start_floor: u8 = 0; // starting position
    let end_floor: u8 = 10; // position we want to reach
    print_shortest_paths(start_floor, end_floor, edgevec);
}

// edges of prolog functions derived from prolog tasks by Wiebke Petersen (Uni Düsseldorf)
fn test_prolog1() {
    println!("prolog 1");
    let edge_1 = Edge {id: 1, entry: 0, exit: 5, weight: 1};
    let edge_2 = Edge {id: 2, entry: 5, exit: 10, weight: 1};
    let edge_3 = Edge {id: 3, entry: 5, exit: 7, weight: 1};
    let edge_4 = Edge {id: 4, entry: 7, exit: 10, weight: 1};
    let start_floor: u8 = 0;
    let end_floor: u8 = 10;
    let mut edgevec = Vec::new();
    edgevec.push(edge_1);
    edgevec.push(edge_2);
    edgevec.push(edge_3);
    edgevec.push(edge_4);
    print_shortest_paths(start_floor, end_floor, edgevec);
}


fn test_prolog2() {
    println!("prolog 2");
    let edge_1 = Edge {id: 1, entry: 0, exit: 5, weight: 1};
    let edge_2 = Edge {id: 2, entry: 5, exit: 10, weight: 1};
    let edge_3 = Edge {id: 3, entry: 5, exit: 8, weight: 1};
    let edge_4 = Edge {id: 4, entry: 8, exit: 10, weight: 1};
    let start_floor: u8 = 0;
    let end_floor: u8 = 10;
    let mut edgevec = Vec::new();
    edgevec.push(edge_1);
    edgevec.push(edge_2);
    edgevec.push(edge_3);
    edgevec.push(edge_4);
    print_shortest_paths(start_floor, end_floor, edgevec);
}

fn test_prolog3() {
    println!("prolog 3");
    let edge_1 = Edge {id: 1, entry: 0, exit: 6, weight: 1};
    let edge_2 = Edge {id: 2, entry: 6, exit: 19, weight: 1};
    let edge_3 = Edge {id: 3, entry: 3, exit: 6, weight: 1};
    let edge_4 = Edge {id: 4, entry: 3, exit: 9, weight: 1};
    let edge_5 = Edge {id: 5, entry: 9, exit: 19, weight: 1};
    let edge_6 = Edge {id: 6, entry: 3, exit: 13, weight: 1};
    let edge_7 = Edge {id: 7, entry: 13, exit: 17, weight: 1};
    let edge_8 = Edge {id: 8, entry: 17, exit: 19, weight: 1};
    let edge_9 = Edge {id: 9, entry: 9, exit: 17, weight: 1};
    let edge_10 = Edge {id: 10, entry: 6, exit: 17, weight: 1};
    let start_floor: u8 = 0;
    let end_floor: u8 = 19;
    let mut edgevec = Vec::new();
    edgevec.push(edge_1);
    edgevec.push(edge_2);
    edgevec.push(edge_3);
    edgevec.push(edge_4);
    edgevec.push(edge_5);
    edgevec.push(edge_6);
    edgevec.push(edge_7);
    edgevec.push(edge_8);
    edgevec.push(edge_9);
    edgevec.push(edge_10);
    print_shortest_paths(start_floor, end_floor, edgevec);
}


fn test_prolog4() {
    println!("prolog 4");
    let edge_1 = Edge {id: 1, entry: 0, exit: 6, weight: 1};
    let edge_2 = Edge {id: 2, entry: 2, exit: 6, weight: 1};
    let edge_3 = Edge {id: 3, entry: 6, exit: 8, weight: 1};
    let edge_4 = Edge {id: 4, entry: 8, exit: 10, weight: 1};
    let edge_5 = Edge {id: 5, entry: 6, exit: 10, weight: 1};

    let start_floor: u8 = 0;
    let end_floor: u8 = 10;
    let mut edgevec = Vec::new();
    edgevec.push(edge_1);
    edgevec.push(edge_2);
    edgevec.push(edge_3);
    edgevec.push(edge_4);
    edgevec.push(edge_5);
    print_shortest_paths(start_floor, end_floor, edgevec);
}

fn test_prolog5() {
    println!("prolog 5");
    let edge_1 = Edge {id: 1, entry: 0, exit: 3, weight: 1};
    let edge_2 = Edge {id: 2, entry: 2, exit: 6, weight: 1};
    let edge_3 = Edge {id: 3, entry: 0, exit: 2, weight: 1};
    let edge_4 = Edge {id: 4, entry: 3, exit: 10, weight: 1};
    let edge_5 = Edge {id: 5, entry: 6, exit: 10, weight: 1};

    let start_floor: u8 = 0;
    let end_floor: u8 = 10;
    let mut edgevec = Vec::new();
    edgevec.push(edge_1);
    edgevec.push(edge_2);
    edgevec.push(edge_3);
    edgevec.push(edge_4);
    edgevec.push(edge_5);
    print_shortest_paths(start_floor, end_floor, edgevec);
}


fn test_prolog6() {
    println!("prolog 6");
    let edge_1 = Edge {id: 1, entry: 0, exit: 3, weight: 1};
    let edge_2 = Edge {id: 2, entry: 5, exit: 10, weight: 1};
    let edge_3 = Edge {id: 3, entry: 3, exit: 8, weight: 1};
    let edge_4 = Edge {id: 4, entry: 8, exit: 12, weight: 1};
    let edge_5 = Edge {id: 5, entry: 8, exit: 12, weight: 1};

    let start_floor: u8 = 0;
    let end_floor: u8 = 12;
    let mut edgevec = Vec::new();
    edgevec.push(edge_1);
    edgevec.push(edge_2);
    edgevec.push(edge_3);
    edgevec.push(edge_4);
    edgevec.push(edge_5);
    print_shortest_paths(start_floor, end_floor, edgevec);
}

fn test_prolog7() {
    println!("prolog 7");
    let edge_1 = Edge {id: 1, entry: 0, exit: 6, weight: 1};
    let edge_2 = Edge {id: 2, entry: 0, exit: 8, weight: 1};
    let edge_3 = Edge {id: 3, entry: 3, exit: 8, weight: 1};
    let edge_4 = Edge {id: 4, entry: 1, exit: 3, weight: 1};
    let edge_5 = Edge {id: 5, entry: 6, exit: 15, weight: 1};
    let edge_6 = Edge {id: 6, entry: 8, exit: 15, weight: 1};

    let start_floor: u8 = 0;
    let end_floor: u8 = 15;
    let mut edgevec = Vec::new();
    edgevec.push(edge_1);
    edgevec.push(edge_2);
    edgevec.push(edge_3);
    edgevec.push(edge_4);
    edgevec.push(edge_5);
    edgevec.push(edge_6);
    print_shortest_paths(start_floor, end_floor, edgevec);
}

fn test_prolog8() {
    println!("prolog 8");
    let edge_1 = Edge {id: 1, entry: 0, exit: 3, weight: 1};
    let edge_2 = Edge {id: 2, entry: 7, exit: 10, weight: 1};
    let edge_3 = Edge {id: 3, entry: 3, exit: 7, weight: 1};
    let edge_4 = Edge {id: 4, entry: 3, exit: 10, weight: 1};
    let edge_5 = Edge {id: 5, entry: 10, exit: 15, weight: 1};

    let start_floor: u8 = 0;
    let end_floor: u8 = 15;
    let mut edgevec = Vec::new();
    edgevec.push(edge_1);
    edgevec.push(edge_2);
    edgevec.push(edge_3);
    edgevec.push(edge_4);
    edgevec.push(edge_5);
    print_shortest_paths(start_floor, end_floor, edgevec);
}

fn main() {
    test_matthiaskrgr();

    test_prolog1();
    test_prolog3();
    test_prolog2();
    test_prolog4();
    test_prolog5();
    test_prolog6();
    test_prolog7();
    test_prolog8();


/*
    let edge_1 = Edge {id: 1, entry: 0, exit: 3, weight: 1};
    let edge_2 = Edge {id: 2, entry: 7, exit: 10, weight: 3};
    let edge_3 = Edge {id: 3, entry: 3, exit: 11, weight: 2};
    let mut path = Path  { edges: Vec::new(), weight: 0, edge_ids: Vec::new() }; // init
    path.append(edge_1);
    path.append(edge_2);
    path.append(edge_3);
*/

}

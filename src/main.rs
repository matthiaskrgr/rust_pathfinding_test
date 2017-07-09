#[derive(Clone)] // give clone trait to this struct
struct Node {
    // id: unique node identifier
    // entry: entry point of the node
    // exit: exit point of the node
    id: u8, 
    entry: u8,
    exit: u8,
}


/*                 Printing                                      */
fn get_node_str(node: &Node) -> String  {
    // take node reference and return info as string for usage in print()
    return format!("node {}:  ({} -> {})", node.id, node.entry, node.exit);
}

fn print_node(node: &Node) {
    // print node info to stdout
    return println!("{}", get_node_str(&node));
}

fn print_node_vector(node_vector: &Vec<Node>) {
    // print all node info of vector
    for node in node_vector {
        print_node(&node);
    }
}

fn verify_nodes(nodes: &Vec<Node>)
{
    let mut node_ids = Vec::new();
    for node in nodes {
        let id = node.id;
        if node_ids.contains(&id) {
            println!("ERROR: 2 nodes with identical ID found: {}", id);
            std::process::exit(2);
        } else {
            node_ids.push(id);
        }
    }
}

fn prune_nodes(available_nodes: &Vec<Node>, start_floor: u8, end_floor: u8) -> Vec<Node> {
    // prune nodes that are unreachable or dead ends
    // fn does only as single pass!
    let mut exits = Vec::new();
    let mut entries = Vec::new();

    // make sure we don't have several nodes with same id
    verify_nodes(&available_nodes);

    for node in available_nodes { // collect entries and exits
        if !exits.contains(&node.exit) {
            exits.push(node.exit);
        }
        if !entries.contains(&node.entry) {
            entries.push(node.entry);
        }
    }

    let mut usable_nodes = Vec::new();

    for node in available_nodes {
        // make sure we don't remove our root and goal node
        if !entries.contains(&((node).exit)) && (node).exit != end_floor  { // remove deadends
            println!("pruning dead end:    {}", get_node_str(&node));
        } else if !exits.contains(&((node).entry)) && (node).entry != start_floor { // remove unreachable
            println!("pruning unreachable: {}", get_node_str(&node));
        } else { // node is neither dead end nor unreachable and thus usable
            usable_nodes.push(node.clone());
        }
    }
    return usable_nodes;
}


fn prune_nodes_recursively(nodes: Vec<Node>, start_floor: u8, end_floor: u8) -> Vec<Node> {
    // prune nodes recursively until we cannot prune any further
    println!("\n ===  pruning  ===");
    let mut a;
    let mut b;
    let mut c = nodes;
    loop { // prune until failure
        a  = prune_nodes(&c, start_floor, end_floor);
        b  = prune_nodes(&a, start_floor, end_floor);
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

fn get_possible_new_connections(node: &Node, purged_nodes: &Vec<Node>) -> Vec<Node> {
    let mut connection_vec = Vec::new();
    for poss_conn in purged_nodes {
        if node.exit == (&poss_conn).entry { // collect possible new connections
            connection_vec.push(poss_conn.clone()); 
        }
    }
    //println!("possible new connections for {}", get_node_str(&node));
    //print_node_vector(&connection_vec);
    //println!("\n");
    return connection_vec;
}


fn print_shortest_paths(start_floor: u8, end_floor: u8, nodevec: Vec<Node>) {
    // meh...
    #[allow(non_snake_case)]
    let  START_FLOOR = start_floor;
    #[allow(non_snake_case)]
    let  END_FLOOR = end_floor; 

    let nodes = nodevec; // immutable now
    println!("Current nodes: ");
        print_node_vector(&nodes); 

    // prune
    let purged_nodes = prune_nodes_recursively(nodes, START_FLOOR, END_FLOOR);

    if purged_nodes.len() == 0 {
        println!("No nodes left to traverse!");
        std::process::exit(1);
    }

    println!("\nRemaining nodes:");
        print_node_vector(&purged_nodes); // obtain paths

    let mut walked_nodes = Vec::new(); // store numbers/ids of walked nodes in this vector, 

    let mut vector_of_paths = Vec::new(); // store paths in this vector, this will be a vector of vectors


    println!("\nSearching for paths...");
    // find entry paths
    let mut initial_entries = Vec::new();
    for node in &purged_nodes {
        if node.entry == START_FLOOR { // possible starting points
            println!("We can start at {}", get_node_str(&node));
            initial_entries.push(node);
        }
    }
    let initial_entries = initial_entries; // make immutable


    // [vec1]  [vec2]  [vec3] ... ] => vector_of_paths
    //   ↓        ↓       ↓
    //  node1   node1   node2
    //   ↓        ↓       ↓
    //  node5   node18  node4 
    //   ...      ...
    for start_node in initial_entries {
        let mut path_vect = Vec::new();
        path_vect.push((start_node).clone()); // start a new path_vector
        walked_nodes.push(start_node.id); // mark node as traversed
        vector_of_paths.push(path_vect.clone()); // save new path vector to VoP
    }
    'iterative_pathfinding_loop: loop {

        let mut vector_of_paths_tmp: Vec<Vec<Node>> = Vec::new(); // vector containing vectors containing nodes
        for subpath in &vector_of_paths {
            // collect ids of nodes in this subpath
            let mut subpath_node_ids = Vec::new();
            for node in subpath {
                subpath_node_ids.push(node.id);
            }
            let last_node_of_subpath = subpath.last().unwrap(); // get last node of the subpath

            // and find new connections
            let new_conns = get_possible_new_connections(&last_node_of_subpath, &purged_nodes);
            //println!("found new connections: {}", new_conns.len());

            if new_conns.len() > 0 { // we have new connections
                for new_connection in new_conns.iter() {
                    if subpath_node_ids.contains(&(new_connection.id)) { // avoid hang in circular paths (path( 5 -> 10) ; path(10 -> 5)
                        continue;
                    }
                    //println!("possible new connection: {}", get_node_str(&new_connection));
                    let mut subpath_tmp = subpath.clone(); // clone current subpath
                    subpath_tmp.push(new_connection.clone()); // and append node
                    vector_of_paths_tmp.push(subpath_tmp.clone()); // add the new subpath to the new vector
                    walked_nodes.push(new_connection.id); // save node nr as walked

                }
            } else { // we have no new connections, clone subpath anyway so it doesnt get dropped
                vector_of_paths_tmp.push(subpath.clone());
            }
        }

        vector_of_paths = vector_of_paths_tmp.clone(); // make vector_of_paths_tmp available in next loop iteration

        let mut break_loop = true;
        for subvector in &vector_of_paths {
            if subvector.last().unwrap().exit != END_FLOOR { // if we have one subpath end node that has not reached end yet
                break_loop = false;                          // we must continue searching
            }
        } 
        if break_loop {
            //println!("breaking search loop");
            break 'iterative_pathfinding_loop;
        }

    } // iterative_path_finding_loop


    // debugging:
    print!("Walked nodes:");
    for nr in &walked_nodes {
        print!(" {}", nr);
    }
    println!();
    // print base vector:
    println!("Printing Vector of Paths");
    let mut it=0;
    for subpath in &vector_of_paths {
        it +=1;
        println!("\tsubpath {}", it);
        for node in subpath  {
            println!("\t\t{}", get_node_str(&node));
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
        for node in subpath {
            println!("{}", get_node_str(&node));
        }
        println!("====");
    }
}





fn test_matthiaskrgr() {
    // Testing

    // path nodes
    let node_1 = Node {id: 1, entry: 0, exit: 5};
    let node_2 = Node {id: 2, entry: 5, exit: 10};
    let node_3 = Node {id: 3, entry: 5, exit: 7};
    let node_4 = Node {id: 4, entry: 7, exit: 10};
    let node_5 = Node {id: 5, entry: 6, exit: 10}; // unreachable
    let node_6 = Node {id: 6, entry: 7, exit: 11}; // dead end after 7 is pruned
    let node_7 = Node {id: 7, entry: 11, exit: 20}; // dead end chain
    let node_8 = Node {id: 8, entry: 20, exit: 25};
    let node_9 = Node {id: 9, entry: 25, exit: 26};
    let node_10 = Node {id: 10, entry: 26, exit: 27};
    let node_11 = Node {id: 11, entry: 50, exit: 10}; // unreachable chain
    let node_12 = Node {id: 12, entry: 49, exit: 50};
    let node_13 = Node {id: 13, entry: 48, exit: 59};
    let node_14 = Node {id: 14, entry: 0, exit: 100}; // 0 -> 100
    let node_15 = Node {id: 15, entry: 100, exit: 10}; // 100 -> 10 // goal
    let node_16 = Node {id: 16, entry: 5, exit: 9}; 
    let node_17 = Node {id: 17, entry: 9, exit: 200}; 
    let node_18 = Node {id: 18, entry: 200, exit: 10};
    let node_19 = Node {id: 19, entry: 7, exit: 5}; // 7 -> 7, 5 -> 7 circular loop


    // move nodes into vector
    let mut nodevec = Vec::new(); // hold all the nodes
    nodevec.push(node_1);
    nodevec.push(node_2);
    nodevec.push(node_3);
    nodevec.push(node_4);
    nodevec.push(node_5);
    nodevec.push(node_6);
    nodevec.push(node_7);
    nodevec.push(node_8);
    nodevec.push(node_9);
    nodevec.push(node_10);
    nodevec.push(node_11);
    nodevec.push(node_12);
    nodevec.push(node_13);
    nodevec.push(node_14);
    nodevec.push(node_15);
    nodevec.push(node_16);
    nodevec.push(node_17);
    nodevec.push(node_18);
    nodevec.push(node_19);

    let start_floor: u8 = 0; // starting position
    let end_floor: u8 = 10; // position we want to reach
    print_shortest_paths(start_floor, end_floor, nodevec);
}

// nodes of prolog functions derived from prolog tasks by Wiebke Petersen (Uni Düsseldorf)
fn test_prolog1() {
    println!("prolog 1");
    let node_1 = Node {id: 1, entry: 0, exit: 5};
    let node_2 = Node {id: 2, entry: 5, exit: 10};
    let node_3 = Node {id: 3, entry: 5, exit: 7};
    let node_4 = Node {id: 4, entry: 7, exit: 10};
    let start_floor: u8 = 0;
    let end_floor: u8 = 10;
    let mut nodevec = Vec::new();
    nodevec.push(node_1);
    nodevec.push(node_2);
    nodevec.push(node_3);
    nodevec.push(node_4);
    print_shortest_paths(start_floor, end_floor, nodevec);
}


fn test_prolog2() {
    println!("prolog 2");
    let node_1 = Node {id: 1, entry: 0, exit: 5};
    let node_2 = Node {id: 2, entry: 5, exit: 10};
    let node_3 = Node {id: 3, entry: 5, exit: 8};
    let node_4 = Node {id: 4, entry: 8, exit: 10};
    let start_floor: u8 = 0;
    let end_floor: u8 = 10;
    let mut nodevec = Vec::new();
    nodevec.push(node_1);
    nodevec.push(node_2);
    nodevec.push(node_3);
    nodevec.push(node_4);
    print_shortest_paths(start_floor, end_floor, nodevec);
}

fn test_prolog3() {
    println!("prolog 3");
    let node_1 = Node {id: 1, entry: 0, exit: 6};
    let node_2 = Node {id: 2, entry: 6, exit: 19};
    let node_3 = Node {id: 3, entry: 3, exit: 6};
    let node_4 = Node {id: 4, entry: 3, exit: 9};
    let node_5 = Node {id: 5, entry: 9, exit: 19};
    let node_6 = Node {id: 6, entry: 3, exit: 13};
    let node_7 = Node {id: 7, entry: 13, exit: 17};
    let node_8 = Node {id: 8, entry: 17, exit: 19};
    let node_9 = Node {id: 9, entry: 9, exit: 17};
    let node_10 = Node {id: 10, entry: 6, exit: 17};
    let start_floor: u8 = 0;
    let end_floor: u8 = 19;
    let mut nodevec = Vec::new();
    nodevec.push(node_1);
    nodevec.push(node_2);
    nodevec.push(node_3);
    nodevec.push(node_4);
    nodevec.push(node_5);
    nodevec.push(node_6);
    nodevec.push(node_7);
    nodevec.push(node_8);
    nodevec.push(node_9);
    nodevec.push(node_10);
    print_shortest_paths(start_floor, end_floor, nodevec);
}


fn test_prolog4() {
    println!("prolog 4");
    let node_1 = Node {id: 1, entry: 0, exit: 6};
    let node_2 = Node {id: 2, entry: 2, exit: 6};
    let node_3 = Node {id: 3, entry: 6, exit: 8};
    let node_4 = Node {id: 4, entry: 8, exit: 10};
    let node_5 = Node {id: 5, entry: 6, exit: 10};

    let start_floor: u8 = 0;
    let end_floor: u8 = 10;
    let mut nodevec = Vec::new();
    nodevec.push(node_1);
    nodevec.push(node_2);
    nodevec.push(node_3);
    nodevec.push(node_4);
    nodevec.push(node_5);
    print_shortest_paths(start_floor, end_floor, nodevec);
}

fn test_prolog5() {
    println!("prolog 5");
    let node_1 = Node {id: 1, entry: 0, exit: 3};
    let node_2 = Node {id: 2, entry: 2, exit: 6};
    let node_3 = Node {id: 3, entry: 0, exit: 2};
    let node_4 = Node {id: 4, entry: 3, exit: 10};
    let node_5 = Node {id: 5, entry: 6, exit: 10};

    let start_floor: u8 = 0;
    let end_floor: u8 = 10;
    let mut nodevec = Vec::new();
    nodevec.push(node_1);
    nodevec.push(node_2);
    nodevec.push(node_3);
    nodevec.push(node_4);
    nodevec.push(node_5);
    print_shortest_paths(start_floor, end_floor, nodevec);
}


fn test_prolog6() {
    println!("prolog 6");
    let node_1 = Node {id: 1, entry: 0, exit: 3};
    let node_2 = Node {id: 2, entry: 5, exit: 10};
    let node_3 = Node {id: 3, entry: 3, exit: 8};
    let node_4 = Node {id: 4, entry: 8, exit: 12};
    let node_5 = Node {id: 5, entry: 8, exit: 12};

    let start_floor: u8 = 0;
    let end_floor: u8 = 12;
    let mut nodevec = Vec::new();
    nodevec.push(node_1);
    nodevec.push(node_2);
    nodevec.push(node_3);
    nodevec.push(node_4);
    nodevec.push(node_5);
    print_shortest_paths(start_floor, end_floor, nodevec);
}

fn test_prolog7() {
    println!("prolog 7");
    let node_1 = Node {id: 1, entry: 0, exit: 6};
    let node_2 = Node {id: 2, entry: 0, exit: 8};
    let node_3 = Node {id: 3, entry: 3, exit: 8};
    let node_4 = Node {id: 4, entry: 1, exit: 3};
    let node_5 = Node {id: 5, entry: 6, exit: 15};
    let node_6 = Node {id: 6, entry: 8, exit: 15};

    let start_floor: u8 = 0;
    let end_floor: u8 = 15;
    let mut nodevec = Vec::new();
    nodevec.push(node_1);
    nodevec.push(node_2);
    nodevec.push(node_3);
    nodevec.push(node_4);
    nodevec.push(node_5);
    nodevec.push(node_6);
    print_shortest_paths(start_floor, end_floor, nodevec);
}

fn test_prolog8() {
    println!("prolog 8");
    let node_1 = Node {id: 1, entry: 0, exit: 3};
    let node_2 = Node {id: 2, entry: 7, exit: 10};
    let node_3 = Node {id: 3, entry: 3, exit: 7};
    let node_4 = Node {id: 4, entry: 3, exit: 10};
    let node_5 = Node {id: 5, entry: 10, exit: 15};

    let start_floor: u8 = 0;
    let end_floor: u8 = 15;
    let mut nodevec = Vec::new();
    nodevec.push(node_1);
    nodevec.push(node_2);
    nodevec.push(node_3);
    nodevec.push(node_4);
    nodevec.push(node_5);
    print_shortest_paths(start_floor, end_floor, nodevec);
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
}

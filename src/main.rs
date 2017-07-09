#[derive(Clone)] // give clone trait to this struct
struct Edge {
    // id: unique egdge identifier
    // entry: entry point of the egdge
    // exit: exit point of the egdge
    id: u8, 
    entry: u8,
    exit: u8,
}


/*                 Printing                                      */
fn get_egdge_str(egdge: &Edge) -> String  {
    // take egdge reference and return info as string for usage in print()
    return format!("egdge {}:  ({} -> {})", egdge.id, egdge.entry, egdge.exit);
}

fn print_egdge(egdge: &Edge) {
    // print egdge info to stdout
    return println!("{}", get_egdge_str(&egdge));
}

fn print_egdge_vector(egdge_vector: &Vec<Edge>) {
    // print all egdge info of vector
    for egdge in egdge_vector {
        print_egdge(&egdge);
    }
}

fn verify_egdges(egdges: &Vec<Edge>)
{
    let mut egdge_ids = Vec::new();
    for egdge in egdges {
        let id = egdge.id;
        if egdge_ids.contains(&id) {
            println!("ERROR: 2 egdges with identical ID found: {}", id);
            std::process::exit(2);
        } else {
            egdge_ids.push(id);
        }
    }
}

fn prune_egdges(available_egdges: &Vec<Edge>, start_floor: u8, end_floor: u8) -> Vec<Edge> {
    // prune egdges that are unreachable or dead ends
    // fn does only as single pass!
    let mut exits = Vec::new();
    let mut entries = Vec::new();

    // make sure we don't have several egdges with same id
    verify_egdges(&available_egdges);

    for egdge in available_egdges { // collect entries and exits
        if !exits.contains(&egdge.exit) {
            exits.push(egdge.exit);
        }
        if !entries.contains(&egdge.entry) {
            entries.push(egdge.entry);
        }
    }

    let mut usable_egdges = Vec::new();

    for egdge in available_egdges {
        // make sure we don't remove our root and goal egdge
        if !entries.contains(&((egdge).exit)) && (egdge).exit != end_floor  { // remove deadends
            println!("pruning dead end:    {}", get_egdge_str(&egdge));
        } else if !exits.contains(&((egdge).entry)) && (egdge).entry != start_floor { // remove unreachable
            println!("pruning unreachable: {}", get_egdge_str(&egdge));
        } else { // egdge is neither dead end nor unreachable and thus usable
            usable_egdges.push(egdge.clone());
        }
    }
    return usable_egdges;
}


fn prune_egdges_recursively(egdges: Vec<Edge>, start_floor: u8, end_floor: u8) -> Vec<Edge> {
    // prune egdges recursively until we cannot prune any further
    println!("\n ===  pruning  ===");
    let mut a;
    let mut b;
    let mut c = egdges;
    loop { // prune until failure
        a  = prune_egdges(&c, start_floor, end_floor);
        b  = prune_egdges(&a, start_floor, end_floor);
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

fn get_possible_new_connections(egdge: &Edge, purged_egdges: &Vec<Edge>) -> Vec<Edge> {
    let mut connection_vec = Vec::new();
    for poss_conn in purged_egdges {
        if egdge.exit == (&poss_conn).entry { // collect possible new connections
            connection_vec.push(poss_conn.clone()); 
        }
    }
    //println!("possible new connections for {}", get_egdge_str(&egdge));
    //print_egdge_vector(&connection_vec);
    //println!("\n");
    return connection_vec;
}


fn print_shortest_paths(start_floor: u8, end_floor: u8, egdgevec: Vec<Edge>) {
    // meh...
    #[allow(non_snake_case)]
    let  START_FLOOR = start_floor;
    #[allow(non_snake_case)]
    let  END_FLOOR = end_floor; 

    let egdges = egdgevec; // immutable now
    println!("Current egdges: ");
        print_egdge_vector(&egdges); 

    // prune
    let purged_egdges = prune_egdges_recursively(egdges, START_FLOOR, END_FLOOR);

    if purged_egdges.len() == 0 {
        println!("No egdges left to traverse!");
        std::process::exit(1);
    }

    println!("\nRemaining egdges:");
        print_egdge_vector(&purged_egdges); // obtain paths

    let mut walked_egdges = Vec::new(); // store numbers/ids of walked egdges in this vector, 

    let mut vector_of_paths = Vec::new(); // store paths in this vector, this will be a vector of vectors


    println!("\nSearching for paths...");
    // find entry paths
    let mut initial_entries = Vec::new();
    for egdge in &purged_egdges {
        if egdge.entry == START_FLOOR { // possible starting points
            println!("We can start at {}", get_egdge_str(&egdge));
            initial_entries.push(egdge);
        }
    }
    let initial_entries = initial_entries; // make immutable


    // [vec1]  [vec2]  [vec3] ... ] => vector_of_paths
    //   ↓        ↓       ↓
    //  egdge1   egdge1   egdge2
    //   ↓        ↓       ↓
    //  egdge5   egdge18  egdge4 
    //   ...      ...
    for start_egdge in initial_entries {
        let mut path_vect = Vec::new();
        path_vect.push((start_egdge).clone()); // start a new path_vector
        walked_egdges.push(start_egdge.id); // mark egdge as traversed
        vector_of_paths.push(path_vect.clone()); // save new path vector to VoP
    }
    'iterative_pathfinding_loop: loop {

        let mut vector_of_paths_tmp: Vec<Vec<Edge>> = Vec::new(); // vector containing vectors containing egdges
        for subpath in &vector_of_paths {
            // collect ids of egdges in this subpath
            let mut subpath_egdge_ids = Vec::new();
            for egdge in subpath {
                subpath_egdge_ids.push(egdge.id);
            }
            let last_egdge_of_subpath = subpath.last().unwrap(); // get last egdge of the subpath

            // and find new connections
            let new_conns = get_possible_new_connections(&last_egdge_of_subpath, &purged_egdges);
            //println!("found new connections: {}", new_conns.len());

            if new_conns.len() > 0 { // we have new connections
                for new_connection in new_conns.iter() {
                    if subpath_egdge_ids.contains(&(new_connection.id)) { // avoid hang in circular paths (path( 5 -> 10) ; path(10 -> 5)
                        continue;
                    }
                    //println!("possible new connection: {}", get_egdge_str(&new_connection));
                    let mut subpath_tmp = subpath.clone(); // clone current subpath
                    subpath_tmp.push(new_connection.clone()); // and append egdge
                    vector_of_paths_tmp.push(subpath_tmp.clone()); // add the new subpath to the new vector
                    walked_egdges.push(new_connection.id); // save egdge nr as walked

                }
            } else { // we have no new connections, clone subpath anyway so it doesnt get dropped
                vector_of_paths_tmp.push(subpath.clone());
            }
        }

        vector_of_paths = vector_of_paths_tmp.clone(); // make vector_of_paths_tmp available in next loop iteration

        let mut break_loop = true;
        for subvector in &vector_of_paths {
            if subvector.last().unwrap().exit != END_FLOOR { // if we have one subpath end egdge that has not reached end yet
                break_loop = false;                          // we must continue searching
            }
        } 
        if break_loop {
            //println!("breaking search loop");
            break 'iterative_pathfinding_loop;
        }

    } // iterative_path_finding_loop


    // debugging:
    print!("Walked egdges:");
    for nr in &walked_egdges {
        print!(" {}", nr);
    }
    println!();
    // print base vector:
    println!("Printing Vector of Paths");
    let mut it=0;
    for subpath in &vector_of_paths {
        it +=1;
        println!("\tsubpath {}", it);
        for egdge in subpath  {
            println!("\t\t{}", get_egdge_str(&egdge));
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
        for egdge in subpath {
            println!("{}", get_egdge_str(&egdge));
        }
        println!("====");
    }
}





fn test_matthiaskrgr() {
    // Testing

    // path egdges
    let egdge_1 = Edge {id: 1, entry: 0, exit: 5};
    let egdge_2 = Edge {id: 2, entry: 5, exit: 10};
    let egdge_3 = Edge {id: 3, entry: 5, exit: 7};
    let egdge_4 = Edge {id: 4, entry: 7, exit: 10};
    let egdge_5 = Edge {id: 5, entry: 6, exit: 10}; // unreachable
    let egdge_6 = Edge {id: 6, entry: 7, exit: 11}; // dead end after 7 is pruned
    let egdge_7 = Edge {id: 7, entry: 11, exit: 20}; // dead end chain
    let egdge_8 = Edge {id: 8, entry: 20, exit: 25};
    let egdge_9 = Edge {id: 9, entry: 25, exit: 26};
    let egdge_10 = Edge {id: 10, entry: 26, exit: 27};
    let egdge_11 = Edge {id: 11, entry: 50, exit: 10}; // unreachable chain
    let egdge_12 = Edge {id: 12, entry: 49, exit: 50};
    let egdge_13 = Edge {id: 13, entry: 48, exit: 59};
    let egdge_14 = Edge {id: 14, entry: 0, exit: 100}; // 0 -> 100
    let egdge_15 = Edge {id: 15, entry: 100, exit: 10}; // 100 -> 10 // goal
    let egdge_16 = Edge {id: 16, entry: 5, exit: 9}; 
    let egdge_17 = Edge {id: 17, entry: 9, exit: 200}; 
    let egdge_18 = Edge {id: 18, entry: 200, exit: 10};
    let egdge_19 = Edge {id: 19, entry: 7, exit: 5}; // 7 -> 7, 5 -> 7 circular loop


    // move egdges into vector
    let mut egdgevec = Vec::new(); // hold all the egdges
    egdgevec.push(egdge_1);
    egdgevec.push(egdge_2);
    egdgevec.push(egdge_3);
    egdgevec.push(egdge_4);
    egdgevec.push(egdge_5);
    egdgevec.push(egdge_6);
    egdgevec.push(egdge_7);
    egdgevec.push(egdge_8);
    egdgevec.push(egdge_9);
    egdgevec.push(egdge_10);
    egdgevec.push(egdge_11);
    egdgevec.push(egdge_12);
    egdgevec.push(egdge_13);
    egdgevec.push(egdge_14);
    egdgevec.push(egdge_15);
    egdgevec.push(egdge_16);
    egdgevec.push(egdge_17);
    egdgevec.push(egdge_18);
    egdgevec.push(egdge_19);

    let start_floor: u8 = 0; // starting position
    let end_floor: u8 = 10; // position we want to reach
    print_shortest_paths(start_floor, end_floor, egdgevec);
}

// egdges of prolog functions derived from prolog tasks by Wiebke Petersen (Uni Düsseldorf)
fn test_prolog1() {
    println!("prolog 1");
    let egdge_1 = Edge {id: 1, entry: 0, exit: 5};
    let egdge_2 = Edge {id: 2, entry: 5, exit: 10};
    let egdge_3 = Edge {id: 3, entry: 5, exit: 7};
    let egdge_4 = Edge {id: 4, entry: 7, exit: 10};
    let start_floor: u8 = 0;
    let end_floor: u8 = 10;
    let mut egdgevec = Vec::new();
    egdgevec.push(egdge_1);
    egdgevec.push(egdge_2);
    egdgevec.push(egdge_3);
    egdgevec.push(egdge_4);
    print_shortest_paths(start_floor, end_floor, egdgevec);
}


fn test_prolog2() {
    println!("prolog 2");
    let egdge_1 = Edge {id: 1, entry: 0, exit: 5};
    let egdge_2 = Edge {id: 2, entry: 5, exit: 10};
    let egdge_3 = Edge {id: 3, entry: 5, exit: 8};
    let egdge_4 = Edge {id: 4, entry: 8, exit: 10};
    let start_floor: u8 = 0;
    let end_floor: u8 = 10;
    let mut egdgevec = Vec::new();
    egdgevec.push(egdge_1);
    egdgevec.push(egdge_2);
    egdgevec.push(egdge_3);
    egdgevec.push(egdge_4);
    print_shortest_paths(start_floor, end_floor, egdgevec);
}

fn test_prolog3() {
    println!("prolog 3");
    let egdge_1 = Edge {id: 1, entry: 0, exit: 6};
    let egdge_2 = Edge {id: 2, entry: 6, exit: 19};
    let egdge_3 = Edge {id: 3, entry: 3, exit: 6};
    let egdge_4 = Edge {id: 4, entry: 3, exit: 9};
    let egdge_5 = Edge {id: 5, entry: 9, exit: 19};
    let egdge_6 = Edge {id: 6, entry: 3, exit: 13};
    let egdge_7 = Edge {id: 7, entry: 13, exit: 17};
    let egdge_8 = Edge {id: 8, entry: 17, exit: 19};
    let egdge_9 = Edge {id: 9, entry: 9, exit: 17};
    let egdge_10 = Edge {id: 10, entry: 6, exit: 17};
    let start_floor: u8 = 0;
    let end_floor: u8 = 19;
    let mut egdgevec = Vec::new();
    egdgevec.push(egdge_1);
    egdgevec.push(egdge_2);
    egdgevec.push(egdge_3);
    egdgevec.push(egdge_4);
    egdgevec.push(egdge_5);
    egdgevec.push(egdge_6);
    egdgevec.push(egdge_7);
    egdgevec.push(egdge_8);
    egdgevec.push(egdge_9);
    egdgevec.push(egdge_10);
    print_shortest_paths(start_floor, end_floor, egdgevec);
}


fn test_prolog4() {
    println!("prolog 4");
    let egdge_1 = Edge {id: 1, entry: 0, exit: 6};
    let egdge_2 = Edge {id: 2, entry: 2, exit: 6};
    let egdge_3 = Edge {id: 3, entry: 6, exit: 8};
    let egdge_4 = Edge {id: 4, entry: 8, exit: 10};
    let egdge_5 = Edge {id: 5, entry: 6, exit: 10};

    let start_floor: u8 = 0;
    let end_floor: u8 = 10;
    let mut egdgevec = Vec::new();
    egdgevec.push(egdge_1);
    egdgevec.push(egdge_2);
    egdgevec.push(egdge_3);
    egdgevec.push(egdge_4);
    egdgevec.push(egdge_5);
    print_shortest_paths(start_floor, end_floor, egdgevec);
}

fn test_prolog5() {
    println!("prolog 5");
    let egdge_1 = Edge {id: 1, entry: 0, exit: 3};
    let egdge_2 = Edge {id: 2, entry: 2, exit: 6};
    let egdge_3 = Edge {id: 3, entry: 0, exit: 2};
    let egdge_4 = Edge {id: 4, entry: 3, exit: 10};
    let egdge_5 = Edge {id: 5, entry: 6, exit: 10};

    let start_floor: u8 = 0;
    let end_floor: u8 = 10;
    let mut egdgevec = Vec::new();
    egdgevec.push(egdge_1);
    egdgevec.push(egdge_2);
    egdgevec.push(egdge_3);
    egdgevec.push(egdge_4);
    egdgevec.push(egdge_5);
    print_shortest_paths(start_floor, end_floor, egdgevec);
}


fn test_prolog6() {
    println!("prolog 6");
    let egdge_1 = Edge {id: 1, entry: 0, exit: 3};
    let egdge_2 = Edge {id: 2, entry: 5, exit: 10};
    let egdge_3 = Edge {id: 3, entry: 3, exit: 8};
    let egdge_4 = Edge {id: 4, entry: 8, exit: 12};
    let egdge_5 = Edge {id: 5, entry: 8, exit: 12};

    let start_floor: u8 = 0;
    let end_floor: u8 = 12;
    let mut egdgevec = Vec::new();
    egdgevec.push(egdge_1);
    egdgevec.push(egdge_2);
    egdgevec.push(egdge_3);
    egdgevec.push(egdge_4);
    egdgevec.push(egdge_5);
    print_shortest_paths(start_floor, end_floor, egdgevec);
}

fn test_prolog7() {
    println!("prolog 7");
    let egdge_1 = Edge {id: 1, entry: 0, exit: 6};
    let egdge_2 = Edge {id: 2, entry: 0, exit: 8};
    let egdge_3 = Edge {id: 3, entry: 3, exit: 8};
    let egdge_4 = Edge {id: 4, entry: 1, exit: 3};
    let egdge_5 = Edge {id: 5, entry: 6, exit: 15};
    let egdge_6 = Edge {id: 6, entry: 8, exit: 15};

    let start_floor: u8 = 0;
    let end_floor: u8 = 15;
    let mut egdgevec = Vec::new();
    egdgevec.push(egdge_1);
    egdgevec.push(egdge_2);
    egdgevec.push(egdge_3);
    egdgevec.push(egdge_4);
    egdgevec.push(egdge_5);
    egdgevec.push(egdge_6);
    print_shortest_paths(start_floor, end_floor, egdgevec);
}

fn test_prolog8() {
    println!("prolog 8");
    let egdge_1 = Edge {id: 1, entry: 0, exit: 3};
    let egdge_2 = Edge {id: 2, entry: 7, exit: 10};
    let egdge_3 = Edge {id: 3, entry: 3, exit: 7};
    let egdge_4 = Edge {id: 4, entry: 3, exit: 10};
    let egdge_5 = Edge {id: 5, entry: 10, exit: 15};

    let start_floor: u8 = 0;
    let end_floor: u8 = 15;
    let mut egdgevec = Vec::new();
    egdgevec.push(egdge_1);
    egdgevec.push(egdge_2);
    egdgevec.push(egdge_3);
    egdgevec.push(egdge_4);
    egdgevec.push(egdge_5);
    print_shortest_paths(start_floor, end_floor, egdgevec);
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

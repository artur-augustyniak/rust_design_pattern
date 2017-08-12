fn iter(col: &[i32], state: Option<usize>) -> Option<usize> {
    match state {
        Some(v) => {
            let next_state;
            if v == (col.len() - 1) {
                next_state = None;
            } else {
                next_state = Some(v + 1);
            }
            println!("lazy iteration, this should be cps: {:?}", col[v]);
            next_state
        }
        _ => None
    }
}


pub fn run() {
    println!("-------------------- {} --------------------", file!());
    let coll = vec![4, 2, 78, 5, 33, 2];
    let init_state = Some(0);
    let rd_state = iter(coll.as_slice(), iter(coll.as_slice(), init_state));//revursive :)
    println!("{:?}", rd_state);
}
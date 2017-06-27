extern crate rand;

use self::rand::Rng;


fn forward_sort_strategy<T: Ord + Clone>(v: &Vec<T>) -> Vec<T> {
    let mut inner_v = v.clone();
    inner_v.sort();
    inner_v.to_vec()
}


fn reverse_sort_strategy<T: Ord + Clone>(v: &Vec<T>) -> Vec<T> {
    let mut inner_v = v.clone();
    inner_v.sort();
    inner_v.reverse();
    inner_v.to_vec()
}


fn sort<F, T: Ord + Clone>(v: &T, f: F) -> T where F: Fn(&T) -> T
{
    f(v)
}


pub fn run() {
    println!("-------------------- {} --------------------", file!());
    let mut rng = rand::thread_rng();
    for _ in 0..10 {
        if rng.gen() {
            let v = vec![4, 3, 45, 654, 2, 4, 5, 2];
            println!("{:?}", v);
            if rng.gen() {
                println!("{:?}", sort(&v, forward_sort_strategy));
            } else {
                println!("{:?}", sort(&v, reverse_sort_strategy));
            }
        } else {
            let v = vec!["a", "c", "b"];
            println!("{:?}", v);
            if rng.gen() {
                println!("{:?}", sort(&v, forward_sort_strategy));
            } else {
                println!("{:?}", sort(&v, reverse_sort_strategy));
            }
        }
    }
}
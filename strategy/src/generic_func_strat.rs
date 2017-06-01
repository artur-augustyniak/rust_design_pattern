fn b_sort(v: &Vec<i32>) -> Vec<i32> {
    vec![1, 2, 3]
}


fn q_sort(v: &Vec<i32>) -> Vec<i32> {
    vec![4, 5, 6]
}


fn sort<F, T>(v: &T, f: F) -> T where F: Fn(&T) -> T
{
    f(v)
}


pub fn run() {
    let v = vec![4, 3, 45, 654, 2, 4, 5, 2];
    let b_sorted = sort(&v, b_sort);
    let q_sorted = sort(&v, q_sort);
    println!("{:?}", v);
    println!("{:?}", b_sorted);
    println!("{:?}", q_sorted);
}
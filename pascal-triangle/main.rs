fn main(){
    let res = run_triangle(19, 7);

    println!("{}", res);
}

fn run_triangle(n: i32, k: i32) -> i32{
    if k > n {
        return 0;
    } else if k == 0 || k == n {
        return 1;
    } else {
        return run_triangle(n - 1, k - 1) + run_triangle(n - 1, k);
    }
}
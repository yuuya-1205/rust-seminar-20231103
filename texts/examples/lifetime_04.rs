fn pick1(x: &[i32], end: usize) -> &[i32] {
    &x[..end]
}

fn main() {
    let v1 = [1, 2, 3, 4, 5];
    let p = pick1(&v1, 2);

    for ss in p {
        println!("{}", ss);
    }
}

fn main() {
    // create
    let new_vec = (1..=10).collect::<Vec<i32>>();
    // let new_vec: Vec<i32> = (1..=10).collect();
    println!("{:?}", new_vec);
    let new_vec=new_vec.into_iter().skip(3).take(4).collect::<Vec<i32>>();
    println!("{:?}",new_vec);
}

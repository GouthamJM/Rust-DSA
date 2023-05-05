pub fn fibonacci(n:i32)->i32{
    let n = n -1 ;
    let mut prev = 0;
    let mut curr = 1;
    for _ in 1..n {
        let next = prev + curr;
        prev = curr;
        curr = next;
    }
    curr
}

pub fn fibonnaci_recurr(n:u8)->u64{
    match n {
        1 => 0,
        2 | 3 => 1,
        _ => fibonnaci_recurr(n-1) + fibonnaci_recurr(n-2)
    }
}
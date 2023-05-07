pub fn combine(l1:[i32;5],l2:[i32;5])->[i32; 10]{
    let mut result :[i32;10]=[0;10];
    let mut i = 0;
    for j in 0..5{
        result[i] = l1[j];
        result[i+1]=l2[j];
        i = i + 2;
    }
    result
}
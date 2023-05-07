pub fn print_times(t:usize,c:char,new_line:bool){
    for _ in 0..t{
        print!("{}",c);
    }
    if new_line {
        println!("");
    }
}

pub fn rect_frame(v:Vec<&str>){
    let longest = v.iter().map(|x| x.len()).max().unwrap_or(0);
    print_times(longest+4,'*',true);
    for w in v.iter(){
        print!("* ");
        print!("{}",w);
        let fill_space = longest - w.len();
        print_times(fill_space,' ',false);
        print!(" *");
        println!("");
    }
    print_times(longest+4,'*',true);
}
pub fn longest_common_sub(str1:&str,str2:&str)->String{
    if str1.len()==0 || str2.len() == 0 {
        return "".to_string()
    }
    let c1 = str1.chars().nth(str1.len()-1).unwrap();
    let c2  = str2.chars().nth(str2.len()-1).unwrap();

    let s1_short = &str1[0..str1.len()-1].to_string();
    let s2_short = &str2[0..str2.len()-1].to_string();

    if c1 == c2 {
        return format!("{}{}",longest_common_sub(s1_short,s2_short),c1);
    }

    let m1 = longest_common_sub(&str1,s2_short);
    let m2 = longest_common_sub(s1_short,&str2);

    if m2.len() > m1.len(){
        return m2;
    }
    m1
}
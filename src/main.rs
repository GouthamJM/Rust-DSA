
use crate::simple::create_acronym;

pub mod simple;

fn main() {
    let _val = create_acronym("Looks good to me");
    print!("The acronlym of Looks good to me is {_val}")
}

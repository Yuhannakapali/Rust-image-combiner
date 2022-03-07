pub fn get_nth_args(n:usize)  -> String {
    std::env::args().nth(n).unwrap()
}

#[derive(Debug)]
pub struct Args {
   pub first_image : String,
   pub second_image : String,
   pub output_image : String,
}

impl Args {
    pub fn new() -> Args {
         Args {
            first_image : get_nth_args(1),
            second_image : get_nth_args(2),
            output_image : get_nth_args(3),
        }
    }    
}
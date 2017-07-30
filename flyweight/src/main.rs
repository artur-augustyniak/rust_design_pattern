extern crate rand;

mod semi_oo;
mod func;

use semi_oo::run as oo_run;
use func::run as fn_run;

fn main(){
    oo_run();
    fn_run();
}
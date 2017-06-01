pub mod generic_func_strat;

use generic_func_strat::run as gfs_run;
//cargo rustc -- -Z unstable-options --pretty=expanded
fn main(){
    gfs_run();


}
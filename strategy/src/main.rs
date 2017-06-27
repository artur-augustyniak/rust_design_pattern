pub mod generic_func_strat;
pub mod functional_validation;

use generic_func_strat::run as gfs_run;
use functional_validation::fv_run;
//cargo rustc -- -Z unstable-options --pretty=expanded
fn main(){
    gfs_run();
    fv_run();

}
extern crate mediator;

use mediator::oo_mediator::run as oo_run;
use mediator::fn_mediator::run as fn_run;


fn main() {
    oo_run();
    fn_run();
}
extern crate rpn_interpreter;

use rpn_interpreter::oo_interpreter::run as oo_run;
use rpn_interpreter::fn_interpreter::run as fn_run;


fn main() {
    //RPN
    //(2 - 3) + 5 = 4
    //2 3 - 5 + = 4
    oo_run();
    fn_run();
}
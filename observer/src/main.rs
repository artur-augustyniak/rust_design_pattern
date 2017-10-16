mod functional;
mod objective;

use functional::run as f_run;

fn main() {
    f_run();
    objective::run();
}


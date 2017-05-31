pub mod builder_trait_obj_strategy_state;
pub mod classic_trait_objects_strategy_state;

use builder_trait_obj_strategy_state::run as builder_t_obj_run;
use classic_trait_objects_strategy_state::run as classic_t_obj_run;

fn main() {
    builder_t_obj_run();
    classic_t_obj_run();
}
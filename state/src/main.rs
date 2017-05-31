pub mod state_trait_objects;
pub mod state_type_encoded;
use state_trait_objects::run as t_obj_run;
use state_type_encoded::run as type_enc_run;


fn main() {
    t_obj_run();
    type_enc_run();

}
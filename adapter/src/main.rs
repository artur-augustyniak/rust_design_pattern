pub mod legacy_train;
pub mod current_train;
pub mod object_adapter;



use legacy_train::*;
use current_train::*;
use object_adapter::*;


fn main() {
    legacy_use();
    current_use();
    object_adapter_use();
}

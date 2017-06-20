pub mod legacy_train;
pub mod current_train;
pub mod current_functional_train;
pub mod object_adapter;
pub mod functional_adapter;


use legacy_train::*;
use current_train::*;
use object_adapter::*;
use current_functional_train::*;
use functional_adapter::*;


fn main() {
    legacy_use();
    current_use();
    current_functional_use();
    object_adapter_use();
    functional_adapter_use();
}

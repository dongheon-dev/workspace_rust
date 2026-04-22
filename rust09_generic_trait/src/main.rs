mod generics;
mod traits;

use generics::ge01_function::generic_function;
use generics::ge02_struct::generic_struct;
use generics::ge03_enum::generic_enum;
use generics::ge04_method::generic_method;

fn main() {
    generic_function();
    generic_struct();
    generic_enum();
    generic_method();

    traits::tr01_trait::trait_basic();
    traits::tr02_bound::trait_bound();
    traits::tr03_object::trait_object();
    traits::tr04_dispatch::trait_dispatch();
}

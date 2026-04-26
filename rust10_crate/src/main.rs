mod crate01_types;
mod crate02_dependency;

fn main() {
    crate01_types::binary_crate();
    crate01_types::library_crate();

    crate02_dependency::rand_test(5, 10);
}

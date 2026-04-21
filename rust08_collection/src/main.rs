mod collections;
mod iterators;

fn main() {
    // collections
    collections::col01_vector::vec_test();
    collections::col02_string::string_test();
    collections::col03_map::hashmap_test();
    collections::col03_map::btreemap_test();
    collections::col04_set::hashset_test();

    // iterators
    iterators::iter01_map_filter_fold::map_filter_fold();
    iterators::iter02_types::iter_types_test();
    iterators::iter02_types::flatmap_test();
    iterators::iter03_lazy::lazy_test();
}

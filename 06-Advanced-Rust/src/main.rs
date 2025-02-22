use unsafe_examples::unsafe_examples::{unsafe_functions, unsafe_raw_pointers, unsafe_union_field_access};
use typestate_examples::typestate_examples::{bad_http_builder_example, good_http_builder_example};

mod typestate_examples;
mod unsafe_examples;

fn main() {
    // Unsafe
    unsafe_functions();
    unsafe_raw_pointers();
    unsafe_union_field_access();

    // Typestate pattern
    bad_http_builder_example();
    good_http_builder_example();
}


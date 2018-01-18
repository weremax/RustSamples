#![allow(dead_code)]
#![allow(unused_variables)]
mod sh;
mod structures;
mod control;
mod types;
mod pm;
mod functions;

fn main() {
    // types::fundamental_data_types();
    // types::scope_and_shadowing();
    // types::operators();
    // types::outside();
    
    // sh::stack_and_heap();

    // control::if_statement();
    // control::while_and_loop();
    // control::for_loop();
    // control::match_statement();

    // structures::structures();
    // structures::enums();
    // structures::unions();
    // structures::option();
    // structures::arrays();
    // structures::vectors();
    // structures::slices();
    // structures::strings();
    // structures::tuples();
    // structures::generics();
    // pm::pattern_matching();

    // functions::functions();
    // functions::methods();
    // functions::closures();
    functions::hof();
}

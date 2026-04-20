mod control01_if;
mod control02_loop;
mod control03_while;
mod control04_for;
mod control05_match;
mod control06_let_expression;

use control05_match::*;
use control06_let_expression::{if_let, while_let, let_else};

fn main() {
    control01_if::if_expression();
    control02_loop::loop_expression();
    control03_while::while_statement();
    control04_for::for_statements();

    match_expression();
    match_guard();
    at_binding();

    if_let();
    while_let();
    let_else();
}

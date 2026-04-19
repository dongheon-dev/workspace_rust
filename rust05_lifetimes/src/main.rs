mod life01_block;
mod life02_function;
mod life03_borrow;
mod life04_closure;
mod life05_static;

use life02_function::scope_test;
use life03_borrow::borrow_scope;
use  life04_closure::*;
use  life05_static::*;

fn main() {
    life01_block::block_scope();

    scope_test();

    borrow_scope();

    closure_borrow();
    closure_move();

    static_scope();

    let s = "hello rust";
    println!("value : {}", s);
    println!("addr : {:p}", s);

}

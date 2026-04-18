mod owner01_move;
mod owner02_copy;
mod owner03_clone;
mod owner04_borrowing;

use owner02_copy::copy_str;
use owner03_clone::clone_str;

fn main() {
    owner01_move::move_str();
    copy_str();
    clone_str();
    owner04_borrowing::borrowing_str();
}

/*
mod hello01;

// 1. mod : module 정의 (or 파일 선언)
pub mod hello {

    // 2. pub : public (rust에선 기본적으로 private)
    pub fn greetings() {
        println!("hello, rust!");
    }

    fn world() {
        println!("Hello, world!");
    }
}

// 3. use : 경로 단축
use hello::greetings;

fn main() {
    // 짧게 사용 가능
    greetings();

    // pub이 아니기 때문에 사용 불가
    // hello::world();
}
*/
/*
// hello01 module
mod hello01;
// 경로 단축
use hello01::greetings;

fn main() {
    greetings()
}
*/
/*
mod hello02;

use hello02::communication::greetings;

fn main() {
    greetings()
}
*/

mod hello03;

use hello03::communication::greetings;

fn main() {
    greetings();
}

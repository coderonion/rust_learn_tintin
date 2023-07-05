mod module1;
use module1::module2;

fn main() {
    println!("添加一个一层子模块，循环打印从’a’~’Z’ 之间的所有字符：");
    module1::print_char();

    println!();

    println!("添加一个二层子模块，循环打印从’A’~’z’ 之间的所有字符：");
    module2::print_char();
}
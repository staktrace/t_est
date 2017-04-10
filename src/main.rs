mod foo;

use foo::*;

type MainEnum = MyEnum;

fn main() {
    println!("{:?}", MyEnum::One);      // This works
    println!("{:?}", MainEnum::One);    // This fails
}

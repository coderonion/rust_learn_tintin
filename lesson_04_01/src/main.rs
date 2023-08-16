// 使用枚举包裹三个不同的类型，并放入一个Vec中，对Vec进行遍历，调用三种不同类型的各自的方法。
struct Type1;
struct Type2;
struct Type3;

enum MyEnum {
    Type1(Type1),
    Type2(Type2),
    Type3(Type3),
}

impl MyEnum {
    fn do_something(&self) {
        match self {
            MyEnum::Type1(type1) => type1.do_something(),
            MyEnum::Type2(type2) => type2.do_something(),
            MyEnum::Type3(type3) => type3.do_something(),
        }
    }
}

impl Type1 {
    fn do_something(&self) {
        println!("Type1 is doing something.");
    }
}

impl Type2 {
    fn do_something(&self) {
        println!("Type2 is doing something.");
    }
}

impl Type3 {
    fn do_something(&self) {
        println!("Type3 is doing something.");
    }
}

///////////////////////////////////////////////////////////////////////////////////////////

// 定义三个不同的类型，使用Trait Object，将其放入一个Vec中，对Vec进行遍历，调用三种不同类型的各自的方法。
struct Type4;
struct Type5;
struct Type6;

trait MyTrait {
    fn do_something(&self);
}

impl MyTrait for Type4 {
    fn do_something(&self) {
        println!("Type4 is doing something.");
    }
}

impl MyTrait for Type5 {
    fn do_something(&self) {
        println!("Type5 is doing something.");
    }
}

impl MyTrait for Type6 {
    fn do_something(&self) {
        println!("Type6 is doing something.");
    }
}

fn main() {
    // 使用枚举包裹三个不同的类型，并放入一个Vec中，对Vec进行遍历，调用三种不同类型的各自的方法。
    let my_vec: Vec<MyEnum> = vec![
        MyEnum::Type1(Type1),
        MyEnum::Type2(Type2),
        MyEnum::Type3(Type3),
    ];
    for item in &my_vec {
        item.do_something();
    }

    // 定义三个不同的类型，使用Trait Object，将其放入一个Vec中，对Vec进行遍历，调用三种不同类型的各自的方法。
    let my_vec: Vec<Box<dyn MyTrait>> = vec![
        Box::new(Type4),
        Box::new(Type5),
        Box::new(Type6),
    ];
    for item in &my_vec {
        item.do_something();
    }
}

/*
使用枚举和使用 Trait Object 来将不同类型的对象放入 Vec 中，并对其进行遍历和方法调用。以下是这两种方法的主要区别：

    1. 枚举方法：

        在枚举方法中，创建了一个枚举类型（例如 MyEnum），用于包裹不同类型的实例。每个实例都有对应的枚举变体，这样可以在代码中清晰地看到每个类型的标识。

        优点：
            明确的枚举变体，易于理解和维护。
            在编译时进行类型检查，避免了运行时错误。

        缺点：
            增加了一层枚举的封装，可能使代码略显冗余。

    2. Trait Object 方法：

        在 Trait Object 方法中，定义了一个 trait（例如 MyTrait），然后让不同的类型实现这个 trait 的方法。使用 Box<dyn MyTrait> 将不同类型的对象装箱成 trait 对象，然后放入 Vec 中。

        优点：
            更灵活，可以将不同类型的对象存储在同一个容器中，无需显式的枚举封装。
            可以在运行时动态决定使用哪个实现，适用于更动态的场景。

        缺点：
            缺乏枚举方法中明确的类型标识，可能在代码中难以直观地知道每个对象的具体类型。
            由于是在运行时进行类型检查，因此存在一定的运行时开销和可能的类型错误。

    选择使用哪种方法取决于具体需求和项目的设计,通常情况下，这两种方法都有其适用的场景和优势。
    如果类型是固定的，并且希望在代码中清楚地看到每个类型的标识，枚举方法可能更合适。
    如果需要更大的灵活性，并且类型可能在运行时动态确定，那么 Trait Object 方法可能更适合。
*/







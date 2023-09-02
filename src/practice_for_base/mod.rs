// Box指针练习

/**
 * box是一个智能指针，用于在堆上分配内存，并持有其数据
 * 她允许你在编译时知道数据的大小，并且拥有所有权，而不需要
 * 手动管理内存的分配和释放
 *
 *
 *
 fn main() {
    // 在堆上分配一个整数，并将所有权移交给 box_pointer
    let box_pointer: Box<i32> = Box::new(42);

    // 使用 * 运算符来获取指针所指向的数据
    println!("Value: {}", *box_pointer);

    // 在这个代码块的末尾，box_pointer 将被释放，从而释放堆上的内存
}
 */
use std::fs::File;
struct Student {
    name: String,
    scores: Box<[u32]>,
}

impl Student {
    fn new(name: String, scores: Vec<u32>) -> Student {
        // into_boxed_slice() 方法用于将一个 Slice 转换为一个 Boxed Slice
        let boxed_scores: Box<[u32]> = scores.into_boxed_slice();

        Student {
            name: name,
            scores: boxed_scores,
        }
    }

    fn average_score(&self) -> f64 {
        let sum: u32 = self.scores.iter().sum();
        let count = self.scores.len() as f64;
        sum as f64 / count
    }
}

// move关键字捕获当前环境中变量的所有权

fn biBaoMove() {
    let num: u32 = 5;

    let add_Resure = move |x: u32| x + num;

    let y = add_Resure(1);

    println!("y:{}", y);

    // 下面的代码将报错，因为 num 的所有权已被 move 到闭包中
    // num = 10;
    // return y;
}

// impl 关键字
/**
 *
在Rust中，impl 关键字是实现对象行为的主要机制。
你可以使用它来定义和实现结构体(struct)、枚举(enum)或者特性(trait)的方法。
 */

// impl作用1：结构体和枚举的方法实现，可以使用impl为结构体或枚举定义方法
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn new(x: f64, y: f64) -> Point {
        Point { x, y }
    }

    fn distance(&self) -> f64 {
        (self.x.powf(2) + self.y.powf(2)).sqrt()
    }
}

// 特性实现:用impl关键字为特定类型实现trait
trait Drawable {
    fn draw(&self);
}

struct Circle {
    radius: i32,
}

impl Drawable for Circle {
    fn draw(&self) {
        // 绘制一个圆
    }
}

// 关联函数：使用impl定义一种不需要实例就可以调用的方法
// 类似与js的静态方法

struct Point {
    x: i32,
    y: i32,
}

impl Point {
    // 这就是一个关联函数
    fn origin() -> Point {
        Point { x: 0, y: 0 }
    }
}

// 使用关联函数
// let origin = Point::origin();

//默认方法 如果你在特性定义中使用 impl，你可以为方法提供默认的实现。
// 实现该特性的类型可以选择覆盖这个默认实现
trait Drawable {
    fn draw(&self) {
        // 提供默认的绘制实现
    }
}

struct Circle {
    radius: i32,
}

// 因为Drawable有默认实现，所以这里可以什么都不写
impl Drawable for Circle {}

/*

在 Rust 中，? 和 unwrap() 都是用来处理 Result 或 Option 类型的错误的方法。但是，它们的行为和使用场合略有不同。

unwrap()：当你有一个 Result<T, E> 或 Option<T> 类型的值，并且你确定这个值是 Ok 或者 Some 的时候，
你可以使用 unwrap() 来获取内部的 T 值。但是，如果你使用 unwrap() 在一个 Err 或 None 的值上，
程序会 panic，这意味着你的程序会立即停止执行，这通常不是一个好的处理错误的方式。

? 操作符：? 操作符可以用在返回 Result 或 Option 的函数中。如果你在 Result 或 Option 值上使用 ?，
它将执行一个条件性的操作。如果值是 Ok 或 Some，它就会解包值，就像 unwrap() 那样。
但是，如果值是 Err 或 None，它将早早返回错误或 None，而不是 panic。这使得错误可以在调用堆栈中向上冒泡，而不是立即引发 panic。

这就是为什么 ? 被认为是更安全、更方便的错误处理方式，因为它避免了可能的 panic。
然而，使用它要求你的函数必须返回 Result 或 Option 类型，否则你不能在函数内部使用 ?。
而 unwrap() 可以在任何地方使用，但你需要保证值不会是 Err 或 None，否则你的程序会 panic。

 */

// Result枚举
fn open_file(filename: &str) -> Result<(), std::io::Error> {
    let f = File::open(filename)?;

    Ok(())
}

// 在Rust中，Option是一个枚举类型，用于表示一个可能存在或可能不存在的值。
// Option枚举有两个成员：Some和None。

// Option的主要作用是处理可能为空的值，以避免使用null或undefined等空值引发的错误。
// 通过使用Option枚举，可以在编译时强制进行空值检查，以确保代码的安全性。

fn divide(numerator: i32, denominator: i32) -> Option<f64> {
    if denominator == 0 {
        none
    } else {
        Some(numerator as f64 / denominator as f64)
    }
}

// 包和 crate
/*
crate是一个二进制的库，crate root是一个源文件，rust编译器已他为起始点，并构成你的
crate根模块，包是提供一系列功能的一个或者多个crate,一个包会包含一个cargo.html文件，
阐述如何去构件这些crate

包中所包含的内容由几条规则来确立，一个包中至多只能包含一个库，包中可以包含多个crate库，包
中至少包含一个crate,无论是库还是二进制的

cargo new my-project

当我们输入了这条命令，
Cargo 会给我们的包创建一个 Cargo.toml 文件。查看 Cargo.toml 的内容，
会发现并没有提到 src/main.rs，因为 Cargo 遵循的一个约定：src/main.rs
 就是一个与包同名的二进制 crate 的 crate 根。同样的，Cargo 知道如果包目录中包含
 src/lib.rs，则包带有与其同名的库 crate，
且 src/lib.rs 是 crate 根。crate 根文件将由 Cargo 传递给 rustc 来实际构建库或者二进制项目。


在此，我们有了一个只包含 src/main.rs 的包，
意味着它只含有一个名为 my-project 的二进制 crate。
如果一个包同时含有 src/main.rs 和 src/lib.rs，则它有两个 crate：
一个库和一个二进制项，且名字都与包相同。通过将文件放在 src/bin 目录下，
一个包可以拥有多个二进制 crate：每个 src/bin 下的文件都会被编译成一个独立的二进制 crate。
*/

/**
 * Option 枚举和其相对于空值的优势

 问题不在于概念而在于具体的实现。为此，Rust 并没有空值，不过它确实拥有一个可以编码存在或不存在概念的枚举。这个枚举是 Option<T>
 空值的问题在于当你尝试像一个非空值那样使用一个空值，会出现某种形式的错误。因为空和非空的属性无处不在，非常容易出现这类错误。
enum Option<T> {
    Some(T),
    None,
}

https://rustwiki.org/zh-CN/std/option/enum.Option.html



/**所有权与栈和堆

 栈（Stack）与堆（Heap）
在很多语言中，你并不需要经常考虑到栈与堆。不过在像 Rust 这样的系统编程语言中，
值是位于栈上还是堆上在更大程度上影响了语言的行为以及为何必须做出这样的抉择。我们会在本章的稍后部分描述所有权与栈和堆相关的内容，
所以这里只是一个用来预热的简要解释。

栈和堆都是代码在运行时可供使用的内存，但是它们的结构不同。栈以放入值的顺序存储值并以相反顺序取出值。
这也被称作 后进先出（last in, first out）。想象一下一叠盘子：当增加更多盘子时，把它们放在盘子堆的顶部，
当需要盘子时，也从顶部拿走。不能从中间也不能从底部增加或拿走盘子！增加数据叫做 进栈（pushing onto the stack），而移出数据叫做 出栈（popping off the stack）。

栈中的所有数据都必须占用已知且固定的大小。在编译时大小未知或大小可能变化的数据，要改为存储在堆上。
堆是缺乏组织的：当向堆放入数据时，你要请求一定大小的空间。内存分配器（memory allocator）在堆的某处找到一块足够大的空位，
把它标记为已使用，并返回一个表示该位置地址的 指针（pointer）。这个过程称作 在堆上分配内存（allocating on the heap），有时简称为 “分配”（allocating）。将数据推入栈中并不被认为是分配。因为指针的大小是已知并且固定的，你可以将指针存储在栈上，不过当需要实际数据时，必须访问指针。

想象一下去餐馆就座吃饭。当进入时，你说明有几个人，餐馆员工会找到一个够大的空桌子并领你们过去。如果有人来迟了，他们也可以通过询问来找到你们坐在哪。

入栈比在堆上分配内存要快，因为（入栈时）分配器无需为存储新数据去搜索内存空间；其位置总是在栈顶。相比之下，在堆上分配内存则需要更多的工作，这是因为分配器必须首先找到一块足够存放数据的内存空间，并接着做一些记录为下一次分配做准备。

访问堆上的数据比访问栈上的数据慢，因为必须通过指针来访问。现代处理器在内存中跳转越少就越快（缓存）。继续类比，假设有一个服务员在餐厅里处理多个桌子的点菜。在一个桌子报完所有菜后再移动到下一个桌子是最有效率的。从桌子 A 听一个菜，接着桌子 B 听一个菜，然后再桌子 A，然后再桌子 B 这样的流程会更加缓慢。出于同样原因，处理器在处理的数据彼此较近的时候（比如在栈上）比较远的时候（比如可能在堆上）能更好的工作。在堆上分配大量的空间也可能消耗时间。

当你的代码调用一个函数时，传递给函数的值（包括可能指向堆上数据的指针）和函数的局部变量被压入栈中。当函数结束时，这些值被移出栈。

跟踪哪部分代码正在使用堆上的哪些数据，最大限度的减少堆上的重复数据的数量，以及清理堆上不再使用的数据确保不会耗尽空间，这些问题正是所有权系统要处理的。一旦理解了所有权，你就不需要经常考虑栈和堆了，不过明白了所有权的存在就是为了管理堆数据，能够帮助解释为什么所有权要以这种方式工作。
 */
 */

#[test]
fn Result_Options_test() {
    match open_file("non_existent_file.txt") {
        Ok(_) => println!("File opened successfully."),
        Err(err) => println!("Failed to open file: {}", err),
    }
}

#[test]

fn average_score_test() {
    let alice = Student::new(String::from("Alice"), vec![80, 90, 75, 85]);
    let avg_score: f64 = alice.average_score();
    assert_eq!(avg_score, avg_score);
    println!("average_score : {:?}", avg_score);
}

#[test]
fn move_test() {
    let a = biBaoMove();
}

#[test]
fn test_divide() {
    let a = 10;
    let b = 2;

    match divide(a, b) {
        Some(result) => println!("result: {:?}", result),
        None => println!("denominator cannot be zero"),
    }
}

// Box指针练习
use std::collections::HashMap;
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
把它标记为已使用，并返回一个表示该位置地址的 指针（pointer）。这个过程称作 在堆上分配内存（allocating on the heap），
有时简称为 “分配”（allocating）。将数据推入栈中并不被认为是分配。因为指针的大小是已知并且固定的，你可以将指针存储在栈上，不过当需要实际数据时，必须访问指针。

想象一下去餐馆就座吃饭。当进入时，你说明有几个人，餐馆员工会找到一个够大的空桌子并领你们过去。如果有人来迟了，他们也可以通过询问来找到你们坐在哪。

入栈比在堆上分配内存要快，因为（入栈时）分配器无需为存储新数据去搜索内存空间；其位置总是在栈顶。相比之下，在堆上分配内存则需要更多的工作，
这是因为分配器必须首先找到一块足够存放数据的内存空间，并接着做一些记录为下一次分配做准备。

访问堆上的数据比访问栈上的数据慢，因为必须通过指针来访问。现代处理器在内存中跳转越少就越快（缓存）。继续类比，假设有一个服务员在餐厅里处理多个桌子的点菜。
在一个桌子报完所有菜后再移动到下一个桌子是最有效率的。从桌子 A 听一个菜，接着桌子 B 听一个菜，然后再桌子 A，然后再桌子 B 这样的流程会更加缓慢。出于同样原因，处理器在处理的数据彼此较近的时候（比如在栈上）比较远的时候（比如可能在堆上）能更好的工作。在堆上分配大量的空间也可能消耗时间。

当你的代码调用一个函数时，传递给函数的值（包括可能指向堆上数据的指针）和函数的局部变量被压入栈中。当函数结束时，这些值被移出栈。

跟踪哪部分代码正在使用堆上的哪些数据，最大限度的减少堆上的重复数据的数量，以及清理堆上不再使用的数据确保不会耗尽空间，这些问题正是所有权系统要处理的。
一旦理解了所有权，你就不需要经常考虑栈和堆了，不过明白了所有权的存在就是为了管理堆数据，能够帮助解释为什么所有权要以这种方式工作。




所有权规则
首先，让我们看一下所有权的规则。当我们通过举例说明时，请谨记这些规则：

Rust 中的每一个值都有一个被称为其 所有者（owner）的变量。
值在任一时刻有且只有一个所有者。
当所有者（变量）离开作用域，这个值将被丢弃。


变量作用域

我们已经在第 2 章完成一个 Rust 程序示例。既然我们已经掌握了基本语法，将不会在之后的例子中包含 fn main() { 代码，所以如果你是一路跟过来的，必须手动将之后例子的代码放入一个 main 函数中。这样，例子将显得更加简明，使我们可以关注实际细节而不是样板代码。

在所有权的第一个例子中，我们看看一些变量的 作用域（scope）。作用域是一个项（item）在程序中有效的范围。假设有这样一个变量：





fn main() {
let s = "hello";
}
变量 s 绑定到了一个字符串字面量，这个字符串值是硬编码进程序代码中的。这个变量从声明的点开始直到当前 作用域 结束时都是有效的。示例 4-1 的注释标明了变量 s 在何处是有效的。

fn main() {
    {                      // s 在这里无效, 它尚未声明
        let s = "hello";   // 从此处起，s 开始有效

        // 使用 s
    }                      // 此作用域已结束，s 不再有效
}



 */
 */
// 下面是如何定义并使用一个（新的）calculate_length 函数，它以一个对象的引用作为参数而不是获取值的所有权：

fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}


// 宏
macro_rules! say_hello{
  ()=>{
    println!("Hello, world!");
  }
}

// 带参数的宏
macro_rules! greet{
  ($name:expr)=>{
    println!("hello {}", $name);
  }
}

macro_rules! repeat_println{
  ($test:expr, $count:expr) =>{
    $(
      println("{}",$test)
    )
  }
}

// HashMap



// 变量与数据交互的方式（一）：移动

fn main() {
  let s1 = String::from("hello");
  let s2 = s1;

  println!("{}, world!", s1);
}
// 你会得到一个类似如下的错误，因为 Rust 禁止你使用无效的引用。
// $ cargo run
//    Compiling ownership v0.1.0 (file:///projects/ownership)
// error[E0382]: borrow of moved value: `s1`
//  --> src/main.rs:5:28
//   |
// 2 |     let s1 = String::from("hello");
//   |         -- move occurs because `s1` has type `String`, which does not implement the `Copy` trait
// 3 |     let s2 = s1;
//   |              -- value moved here
// 4 | 
// 5 |     println!("{}, world!", s1);
//   |                            ^^ value borrowed here after move

// For more information about this error, try `rustc --explain E0382`.
// error: could not compile `ownership` due to previous error



// 变量与数据交互的方式（二）：克隆
fn main() {
  let s1 = String::from("hello");
  let s2 = s1.clone();

  println!("s1 = {}, s2 = {}", s1, s2);
}
// 这段代码能正常运行，并且明确产生图 4-3 中行为，这里堆上的数据 确实 被复制了。



// 只在栈上的数据：拷贝

fn main() {
  let x = 5;
  let y = x;

  println!("x = {}, y = {}", x, y);
}
// 但这段代码似乎与我们刚刚学到的内容相矛盾：没有调用 clone，不过 x 依然有效且没有被移动到 y 中。

// 原因是像整型这样的在编译时已知大小的类型被整个存储在栈上，所以拷贝其实际的值是快速的。这意味着没有理由在创建变量 y 后使 x 无效。换句话说，这里没有深浅拷贝的区别，
// 所以这里调用 clone 并不会与通常的浅拷贝有什么不同，我们可以不用管它。


// 所有权与函数
// 将值传递给函数在语义上与给变量赋值相似。向函数传递值可能会移动或者复制，就像赋值语句一样。
// 示例 4-3 使用注释展示变量何时进入和离开作用域：

fn main() {
  let s = String::from("hello");  // s 进入作用域

  takes_ownership(s);             // s 的值移动到函数里 ...
                                  // ... 所以到这里不再有效

  let x = 5;                      // x 进入作用域

  makes_copy(x);                  // x 应该移动函数里，
                                  // 但 i32 是 Copy 的，所以在后面可继续使用 x

} // 这里, x 先移出了作用域，然后是 s。但因为 s 的值已被移走，
  // 所以不会有特殊操作

fn takes_ownership(some_string: String) { // some_string 进入作用域
  println!("{}", some_string);
} // 这里，some_string 移出作用域并调用 `drop` 方法。占用的内存被释放

fn makes_copy(some_integer: i32) { // some_integer 进入作用域
  println!("{}", some_integer);
} // 这里，some_integer 移出作用域。不会有特殊操作

// 示例 4-3：带有所有权和作用域注释的函数
// 当尝试在调用 takes_ownership 后使用 s 时，Rust 会抛出一个编译时错误。
// 这些静态检查使我们免于犯错。试试在 main 函数中添加使用 s 和 x 的代码来看看哪里能使用他们，
// 以及所有权规则会在哪里阻止我们这么做。


// 返回值与作用域

// 返回值也可以转移所有权。示例 4-4 与示例 4-3 一样带有类似的注释。
fn main() {
  let s1 = gives_ownership();         // gives_ownership 将返回值
                                      // 移给 s1
  let s2 = String::from("hello");     // s2 进入作用域
  let s3 = takes_and_gives_back(s2);  // s2 被移动到
                                      // takes_and_gives_back 中,
                                      // 它也将返回值移给 s3
} // 这里, s3 移出作用域并被丢弃。s2 也移出作用域，但已被移走，
  // 所以什么也不会发生。s1 移出作用域并被丢弃

fn gives_ownership() -> String {           // gives_ownership 将返回值移动给
                                           // 调用它的函数
  let some_string = String::from("yours"); // some_string 进入作用域
  some_string                              // 返回 some_string 并移出给调用的函数
}

// takes_and_gives_back 将传入字符串并返回该值
fn takes_and_gives_back(a_string: String) -> String { // a_string 进入作用域
  a_string  // 返回 a_string 并移出给调用的函数
}

// 示例 4-4: 转移返回值的所有权

// 变量的所有权总是遵循相同的模式：将值赋给另一个变量时移动它。当持有堆中数据值的变量离开作用域时，
// 其值将通过 drop 被清理掉，除非数据被移动为另一个变量所有。
// 在每一个函数中都获取所有权并接着返回所有权有些啰嗦。如果我们想要函数使用一个值但不获取所有权该怎么办呢？
// 如果我们还要接着使用它的话，每次都传进去再返回来就有点烦人了，除此之外，我们也可能想返回函数体中产生的一些数据。
// 我们可以使用元组来返回多个值，如示例 4-5 所示。




// Rust 借用所有权 Borrowing / 引用
// 堆（ heap ） 上分配的变量都有所有权。

/**
 * 使用的过程中，我就一直在想，为什么不多支持一个 借用所有权 或者 租借所有权 的概念呢 ？

把具有所有权的变量传递给函数作为参数时，就是临时出租所有权，
当函数执行完后就会自动收回所有权。就像现实生活中，
我可以把某个工具临时借用给其它人，当他们使用完了之后还给我们就可以了。


Rust 支持对所有权的 出借 borrowing。当把一个具有所有权的变量传递给函数时，
就是把所有权借用给函数的参数，当函数返回后则自动收回所有权。


下面的代码，我们并没有使用上一章节的 所有权 转让规则收回所有权，所以程序会报错


fn main(){

    let v = vec![10,20,30]; // 声明一个向量，变量 v 具有数据的所有权
    print_vector(v);
    println!("{}",v[0]);    // 这行会报错
}

fn print_vector(x:Vec<i32>){
    println!("Inside print_vector function {:?}",x);
}

&s1 语法让我们创建一个 指向 值 s1 的引用，但是并不拥有它。
因为并不拥有这个值，所以当引用停止使用时，它所指向的值也不会被丢弃。

变量 s 有效的作用域与函数参数的作用域一样，不过当引用停止使用时并不丢弃它指向的数据，因为我们没有所有权。
当函数使用引用而不是实际值作为参数，无需返回值来交还所有权，因为就不曾拥有所有权。

我们将创建一个引用的行为称为 借用（borrowing）。正如现实生活中，
如果一个人拥有某样东西，你可以从他那里借来。当你使用完毕，必须还回去。

我们将创建一个引用的行为称为 借用（borrowing）。正如现实生活中，
如果一个人拥有某样东西，你可以从他那里借来。当你使用完毕，必须还回去。


如果我们尝试修改借用的变量呢？剧透：这行不通！
fn main() {
    let s = String::from("hello");

    change(&s);
}

fn change(some_string: &String) {
    some_string.push_str(", world"); //报错
}

正如变量在默认情况下是不可变的一样，引用也是不可变的。我们无法通过引用修改内容。

可变引用

fn main() {
    let mut s = String::from("hello");

    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

我们通过一个小调整就能修复示例

fn main() {
    let mut s = String::from("hello");

    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

先，我们必须将 s 改为 mut。然后必须在调用 change 函数的地方创建一个可变引用 &mut s，并更新函数签名以接受一个可变引用 some_string: &mut String。这就非常清楚地表明，change 函数将改变它所借用的值。

不过可变引用有一个很大的限制：在同一时间，只能有一个对某一特定数据的可变引用。尝试创建两个可变引用的代码将会失败：

fn main() {
    let mut s = String::from("hello");
    let r1 = &mut s;
    let r2 = &mut s; 报错
    println!("{}, {}", r1, r2);
}
这个报错说这段代码是无效的，因为我们不能在同一时间多次将 s 作为可变变量借用。
第一个可变的借入在 r1 中，并且必须持续到在 println! 中使用它，但是在那个可变引用的创建和它的使用之间，
我们又尝试在 r2 中创建另一个可变引用，它借用了与 r1 相同的数据。

防止同一时间对同一数据进行多个可变引用的限制允许可变性，不过是以一种受限制的方式允许。
新 Rustacean 们经常难以适应这一点，因为大部分语言中变量任何时候都是可变的。

这个限制的好处是 Rust 可以在编译时就避免数据竞争。数据竞争（data race）类似于竞态条件，它由这三个行为造成：
两个或更多指针同时访问同一数据。
至少有一个指针被用来写入数据。
没有同步数据访问的机制。

以上三个行为同时发生才会造成数据竞争，而不是单一行为。

一如既往，可以使用大括号来创建一个新的作用域，以允许拥有多个可变引用，只是不能 同时 拥有：
fn main() {
    let mut s = String::from("hello");

    {
        let r1 = &mut s;
    } // r1 在这里离开了作用域，所以我们完全可以创建一个新的引用

    let r2 = &mut s;
}



类似的规则也存在于同时使用可变与不可变引用中。这些代码会导致一个错误：

fn main() {
    let mut s = String::from("hello");

    let r1 = &s; // 没问题
    let r2 = &s; // 没问题
    let r3 = &mut s; // 大问题

    println!("{}, {}, and {}", r1, r2, r3);
}

哇哦！我们 也 不能在拥有不可变引用的同时拥有可变引用。
使用者可不希望不可变引用的值在他们的眼皮底下突然被改变了！
然而，多个不可变引用是可以的，因为没有哪个只能读取数据的人有能力影响其他人读取到的数据。


引用的作用范围:
注意一个引用的作用域从声明的地方开始一直持续到最后一次使用为止。
例如，因为最后一次使用不可变引用（println!)，发生在声明可变引用之前，所以如下代码是可以编译的：

fn main() {
    let mut s = String::from("hello");

    let r1 = &s; // 没问题
    let r2 = &s; // 没问题
    println!("{} and {}", r1, r2);
    // 此位置之后 r1 和 r2 不再使用

    let r3 = &mut s; // 没问题
    println!("{}", r3);
}



悬垂引用（Dangling References）

在具有指针的语言中，很容易通过释放内存时保留指向它的指针而错误地生成一个 悬垂指针（dangling pointer），所谓悬垂指针是其指向的内存可能已经被分配给其它持有者。相比之下，
在 Rust 中编译器确保引用永远也不会变成悬垂状态：当你拥有一些数据的引用，编译器确保数据不会在其引用之前离开作用域。

fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String { // dangle 返回一个字符串的引用
    let s = String::from("hello"); // s 是一个新字符串
    &s // 返回字符串 s 的引用  
} // 这里 s 离开作用域并被丢弃。其内存被释放。
  // 危险！ 会报错


  引用的规则
让我们概括一下之前对引用的讨论：

在任意给定时间，要么 只能有一个可变引用，要么 只能有多个不可变引用。
引用必须总是有效的。


切片 Slice 类型
另一个没有所有权的数据类型是 slice。
slice 允许你引用集合中一段连续的元素序列，而不用引用整个集合。



fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn main() {}


/**
 
Rust 的每个值都有确切的数据类型（data type），该类型告诉 Rust 数据是被指定成哪类数据，从而让 Rust 知道如何使用该数据。在本节中，我们将介绍两种数据类型：标量类型和复合类型。

请记住 Rust 是一种静态类型（statically typed）的语言，
这意味着它必须在编译期知道所有变量的类型。
编译器通常可以根据值和使用方式推导出我们想要使用的类型。
在类型可能是多种情况时，例如在第 2 章“比较猜测的数字和秘密数字”中当我们使用 parse 将String 转换成数值类型时，我们必须加上一个类型标注，如下所示：

let guess: u32 = "42".parse().expect("Not a number!");


如果我们在这里不添加类型标注的话，Rust 将显示以下错误，
意思是编译器需要我们提供更多信息来确定我们到底想用什么类型：

$ cargo build
   Compiling no_type_annotations v0.1.0 (file:///projects/no_type_annotations)
error[E0282]: type annotations needed
 --> src/main.rs:2:9
  |
2 |     let guess = "42".parse().expect("Not a number!");
  |         ^^^^^ consider giving `guess` a type

For more information about this error, try `rustc --explain E0282`.
error: could not compile `no_type_annotations` due to previous error


标量类型

标量（scalar）类型表示单个值。Rust 有 4 个基本的标量类型：整型、浮点型、
布尔型和字符。你可能从其他语言了解过这些类型。
下面我们深入了解它们在 Rust 中的用法


整数类型
整数（integer）是没有小数部分的数字。我们在第 2 章使用过一个整数类型（整型），
即 u32 类型。此类型声明表明它关联的值应该是占用 32 位空间的无符号整型
（有符号整型以 i 开始，i 是英文单词 integer 的首字母，与之相反的是 u，
代表无符号 unsigned 类型）。表 3-1 显示了 Rust 中的内置的整数类型。
我们可以使用这些定义形式中的任何一个来声明整数值的类型。



长度	有符号类型	无符号类型
8 位	i8	u8
16 位	i16	u16
32 位	i32	u32
64 位	i64	u64
128 位	i128	u128
arch	isize	usize


每个定义形式要么是有符号类型要么是无符号类型，且带有一个显式的大小。
有符号和无符号表示数字能否取负数——也就是说，这个数是否可能是负数（有符号类型），或一直为正而不需要带上符号（无符号类型）。就像在纸上写数字一样：当要强调符号时，数字前面可以带上正号或负号；
然而，当很明显确定数字为正数时，就不需要加上正号


复合类型


元组类型

元组是将多种类型的多个值组合到一个复合类型中的一种基本方式。元组的长度是固定的：声明后，它们就无法增长或缩小。

我们通过在小括号内写入以逗号分隔的值列表来创建一个元组。元组中的每个位置都有一个类型，
并且元组中不同值的类型不要求是相同的。我们在下面示例中添加了可选的类型标注：

数组类型
将多个值组合在一起的另一种方式就是使用数组（array）。与元组不同，数组的每个元素必须具有相同的类型。与某些其他语言中的数组不同，Rust 中的数组具有固定长度。

当你希望将数据分配到栈（stack）而不是堆（heap）时（我们将在第 4 章中进一步讨论栈和堆），或者当你希望确保始终具有固定数量的元素时，数组特别有用。但它们不像 vector（译注：中文字面翻译为“向量”，在 Rust 中意义为“动态数组，可变数组”）类型那么灵活。vector 类型类似于标准库中提供的集合类型，其大小允许增长或缩小。如果不确定是使用数组还是 vector，那就应该使用一个 vector。第 8 章将详细地讨论 vector。

不过当你明确元素数量不需要改变时，数组会更有用。例如，如果你在程序中使用月份的名称，你很可能希望使用的是数组而不是 vector，因为你知道它始终包含 12 个元素：



泛型数据类型
当使用泛型定义函数时，本来在函数签名中指定参数和返回值的类型的地方，
会改用泛型来表示。采用这种技术，使得代码适应性更强，从而为函数的调用者提供更多的功能
，同时也避免了代码的重复。

函数中定义泛型

fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);
   assert_eq!(result, 100);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);
    println!("The largest char is {}", result);
   assert_eq!(result, 'y');
}


使用泛型合并上面两个 函数
fn largest<T>(list: &[T])->T{
     let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

error[E0369]: binary operation `>` cannot be applied to type `T`
 --> src/main.rs:5:12
  |
5 |         if item > largest {
  |            ^^^^^^^^^^^^^^
  |
  = note: an implementation of `std::cmp::PartialOrd` might be
   missing for `T`
注释中提到了 std::cmp::PartialOrd，这是一个 trait。下一部分会讲到 trait。
不过简单来说，这个错误表明 largest 的函数体不能适用于 T 的所有可能的类型。因为在函数体需要比较 T 类型的值，不过它只能用于我们知道如何排序的类型。为了开启比较功能，标准库中定义的 std::cmp::PartialOrd trait 可以实现类型的比较功能（查看附录 C 获取该 trait 的更多信息）。

标准库中定义的 std::cmp::PartialOrd trait 可以实现类型的比较功能。在
 “trait 作为参数” 部分会讲解如何指定泛型实现特定的 trait，
 不过让我们先探索其他使用泛型参数的方法。

*/
 * 
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

#[test]

fn test_macro(){
  say_hello!();
  repeat_println!("Hello", 3);
}


#[test]
fn test_hash_map(){
  let mut word_count:HashMap<String,u32> = HashMap::new();

  for word in text.split_whitespace(){
    // 对当前单词使用entry()方法，该方法返回一个Entry枚举值，
    // 它表示HashMap中的一个条目。如果这个单词已经存在于HashMap中，
    // entry()方法将返回已存在的条目，否则将会插入一个新的条目。
    // 使用or_insert(0)方法，
    // 如果这个单词不存在，它会插入一个值为0的新条目，并返回对应的可变引用
    let entry = word_count.entry(word.to_string()).or_insert(0);
    *entry += 1;
  }
     // 打印单词和对应的出现次数
  for (word, count) in &word_count {
      println!("Word: {}, Count: {}", word, count);
  }

}
/**
 
 ← Rust 常量Rust 运算符 →
Rust 字符串
Rust 语言提供了两种字符串

字符串字面量 &str。它是 Rust 核心内置的数据类型。

字符串对象 String。它不是 Rust 核心的一部分，只是 Rust 标准库中的一个 公开 pub 结构体

← Rust 常量Rust 运算符 →
Rust 字符串
Rust 语言提供了两种字符串

字符串字面量 &str。它是 Rust 核心内置的数据类型。

字符串对象 String。它不是 Rust 核心的一部分，只是 Rust 标准库中的一个 公开 pub 结构体。

字符串字面量 &str
字符串字面量 &str 就是在 编译时 就知道其值的字符串类型，是 Rust 语言核心的一部分。

字符串字面量 &str 是字符的集合，被硬编码赋值给一个变量。

Rust 中的字符串字面量被称之为 字符串切片。因为它的底层实现是 切片。
字符串字面量模式是 静态 的。 这就意味着字符串字面量从创建时开始会一直保存到程序结束。


字符串对象

字符串对象
字符串对象是 Rust 标准库提供的内建类型。

与字符串字面量不同的是：字符串对象并不是 Rust 核心内置的数据类型，它只是标准库中的一个 公开 pub 的结构体。

字符串对象在标准库中的定义语法如下

pub struct String
字符串对象是是一个 长度可变的集合，它是 可变 的而且使用 UTF-8 作为底层数据编码格式。

字符串对象在 堆 heap 中分配，可以在运行时提供字符串值以及相应的操作方法。

创建字符串对象的语法
要创建一个字符串对象，有两种方法：

一种是创建一个新的空字符串，使用 String::new() 静态方法

String::new()
另一种是根据指定的字符串字面量来创建字符串对象，使用 String::from() 方法

String::from()
范例

 */

/**
Rust ownership
编程语言把内存分为两大类：

栈 stack
堆 heap
当然了，这两种分类并没有对实际的内存做什么，只是把系统分给应用程序的内存标识为上面的两大类而已


ust 语言中每一值都有一个对应的变量，这个变量就成为这个值的 所有者。从某些方面说，定义一个变量就是为这个变量和它存储的数据定义一种所有者管理，声明这个值由这个变量所有。

例如，对于 let age = 30 这条语句，相当于声明 30 这个值由变量 age 所有。
任何东西只有一个所有者，Rust 中是不允许有共同所有者这个概念的。

Rust 中，任何特定时刻，一个数据只能有一个所有者。

Rust 中，不允许两个变量同时指向同一块内存区域。变量必须指向不同的内存区域。


转让所有权
既然所有权就是一个东西属不属于你，你有没有权力随意处理它，比如送人，比如扔掉。
那么转让所有权就会时不时的发生。
Rust 语言中转让所有权的方式有以下几种：

  把一个变量赋值给另一个变量。重要
  把变量传递给函数作为参数。
  函数中返回一个变量作为返回值。

接下来我们分别对这三种方式做详细的介绍

把一个变量赋值给另一个变量
fn main(){

   // 向量 v 拥有堆上数据的所有权
   // 每次只能有一个变量对堆上的数据拥有所有权
   let v = vec![1,2,3]; 


   // 赋值会导致两个变量都对同一个数据拥有所有权
   // 因为两个变量指向了相同的内存块
   let v2 = v; 

   // Rust 会检查两个变量是否同时拥有堆上内存块的所有权。
   // 如果发生所有权竞争，它会自动将所有权判给给新的变量
   // 运行出错，因为 v 不再拥有数据的所有权
   println!("{:?}",v);
}
上面的代码中我们首先声明了一个向量 v。所有权的概念是只有一个变量绑定到资源，v 绑定到资源或 v2 绑定到资源。

上面的代码会发生编译错误 use of moved value: v。这是因为赋值操作会将资源的所有权转移到了



把变量传递给函数作为参数

fn main(){
   let v = vec![1,2,3];     // 向量 v 拥有堆上数据的所有权
   let v2 = v;              // 向量 v 将所有权转让给 v2
   display(v2);             // v2 将所有权转让给函数参数 v ，v2 将变得不可用
   println!("In main {:?}",v2);    // v2 变得不可用
}
fn display(v:Vec<i32>){
   println!("inside display {:?}",v);
}
inside display [1, 2, 3]


函数中返回一个变量作为返回值
fn main(){
   let v = vec![1,2,3];       // 向量 v 拥有堆上数据的所有权
   let v2 = v;                // 向量 v 将所有权转让给 v2
   let v2_return = display(v2);    
   println!("In main {:?}",v2_return);
}

fn display(v:Vec<i32>)-> Vec<i32> { 
   // 返回同一个向量
   println!("inside display {:?}",v);
   return v;
}

编译运行上面的 Rust 代码，输出结果如下
inside display [1, 2, 3]
In main [1, 2, 3]

 */

 /**
  * 
包（Packages）： Cargo 的一个功能，它允许你构建、测试和分享 crate。
Crates ：一个模块的树形结构，它形成了库或二进制项目。
模块（Modules）和 use： 允许你控制作用域和路径的私有性。
路径（path）：一个命名例如结构体、函数或模块等项的方式


crate 是一个二进制项或者库。crate root 是一个源文件，Rust 编译器以它为起始点，并构成你的 crate 的根模块（我们将在“定义模块来控制作用域与私有性”一节深入解读）。包（package） 是提供一系列功能的一个或者多个 crate。一个包会包含有一个 Cargo.toml 文件，阐述如何去构建这些 crate。

包中所包含的内容由几条规则来确立。一个包中至多 只能 包含一个库 crate(library crate)；包中可以包含任意多个二进制 crate(binary crate)；包中至少包含一个 crate，无论是库的还是二进制的。

crate 是一个二进制项或者库。crate root 是一个源文件，Rust 编译器以它为起始点，并构成你的 crate 的根模块（我们将在“定义模块来控制作用域与私有性”一节深入解读）。包（package） 是提供一系列功能的一个或者多个 crate。一个包会包含有一个 Cargo.toml 文件，阐述如何去构建这些 crate。

包中所包含的内容由几条规则来确立。一个包中至多 只能 包含一个库 crate(library crate)；包中可以包含任意多个二进制 crate(binary crate)；包中至少包含一个 crate，无论是库的还是二进制的。

。如果一个包同时含有 src/main.rs 和 src/lib.rs，则它有两个 crate：一个库和一个二进制项，且名字都与包相同。通过将文件放在 src/bin 目录下，一个包可以拥有多个二进制 crate：每个 src/bin 下的文件都会被编译成一个独立的二进制 crate。
  
  
  定义模块来控制作用域与私有性

  在本节，我们将讨论模块和其它一些关于模块系统的部分，如允许你命名项的 路径（paths）；用来将路径引入作用域的 use 关键字；以及使项变为公有的 pub 关键字。我们还将讨论 as 关键字、外部包和 glob 运算符。现在，让我们把注意力放在模块上！

模块 让我们可以将一个 crate 中的代码进行分组，以提高可读性与重用性。模块还可以控制项的 私有性，即项是可以被外部代码使用的（public），还是作为一个内部实现的内容，不能被外部代码使用（private）。

在餐饮业，餐馆中会有一些地方被称之为 前台（front of house），还有另外一些地方被称之为 后台（back of house）。前台是招待顾客的地方，在这里，店主可以为顾客安排座位，服务员接受顾客下单和付款，调酒师会制作饮品。后台则是由厨师工作的厨房，洗碗工的工作地点，以及经理做行政工作的地方组成。

我们可以将函数放置到嵌套的模块中，来使我们的 crate 结构与实际的餐厅结构相同。通过执行 cargo new --lib restaurant，来创建一个新的名为 restaurant 的库。然后将示例 7-1 中所罗列出来的代码放入 src/lib.rs 中，来定义一些模块和函数。



#![allow(unused)]
fn main() {
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn server_order() {}

        fn take_payment() {}
    }
}
}


我们用关键字 mod 定义一个模块，指定模块的名字（在示例中为 front_of_house），并用大括号包围模块的主体。我们可以在模块中包含其他模块，就像本示例中的 hosting 和 serving 模块。模块中也可以包含其他项，比如结构体、枚举、常量、trait，或者像示例 7-1 一样——包含函数。

通过使用模块，我们可以把相关的定义组织起来，并通过模块命名来解释为什么它们之间有相关性。使用这部分代码的开发者可以更方便的循着这种分组找到自己需要的定义，而不需要通览所有。编写这部分代码的开发者通过分组知道该把新功能放在哪里以便继续让程序保持组织性。

之前我们提到，src/main.rs 和 src/lib.rs 被称为 crate 根。如此称呼的原因是，这两个文件中任意一个的内容会构成名为 crate 的模块，且该模块位于 crate 的被称为 模块树 的模块结构的根部（"at the root of the crate’s module structure"）。


crate
 └── front_of_house
     ├── hosting
     │   ├── add_to_waitlist
     │   └── seat_at_table
     └── serving
         ├── take_order
         ├── serve_order
         └── take_payment


         这个树展示了模块间是如何相互嵌套的（比如，hosting 嵌套在 front_of_house 内部）。这个树还展示了一些模块互为 兄弟 ，即它们被定义在同一模块内（hosting 和 serving 都定义在 front_of_house 内）。继续使用家族比喻，如果模块A包含在模块B的内部，我们称模块A是模块B的 孩子 且模块B是模块A的 父辈 。注意整个模块树的根位于名为 crate 的隐式模块下。

模块树或许让你想起了电脑上文件系统的目录树。这是一个非常恰当的比喻！就像文件系统中的目录那样，你应使用模块来组织你的代码。而且就像一个目录中的文件那样，我们需要一个找到我们的模块的方式。
  
 
 
 路径有两种形式：

绝对路径（absolute path）从 crate 根部开始，以 crate 名或者字面量 crate 开头。
相对路径（relative path）从当前模块开始，以 self、super 或当前模块的标识符开头。
绝对路径和相对路径都后跟一个或多个由双冒号（::）分割的标识符。

让我们回到示例 7-1。我们如何调用 add_to_waitlist 函数？还是同样的问题，add_to_waitlist 函数的路径是什么？在示例 7-3 中，
我们通过删除一些模块和函数，稍微简化了一下我们的代码。我们在 crate 根部定义了一个新函数 eat_at_restaurant，并在其中展示调用 add_to_waitlist 函数的两种方法。
eat_at_restaurant 函数是我们 crate 库的一个公共 API，所以我们使用 pub 关键字来标记它。
在“使用 pub 关键字暴露路径”一节，我们将详细介绍 pub。注意，这个例子无法编译通过，我们稍后会解释原因

mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}


panic! 与不可恢复的错误


有的时候代码出问题了，而你对此束手无策。对于这种情况，Rust 有 panic!宏。当执行这个宏时，程序会打印出一个错误信息，展开并清理栈数据，
然后接着退出。出现这种情况的场景通常是检测到一些类型的 bug，而且开发者并不清楚该如何处理它。

对应 panic 时的栈展开或终止

对应 panic 时的栈展开或终止
当出现 panic 时，程序默认会开始 展开（unwinding），这意味着 Rust 会回溯栈并清理它遇到的每一个函数的数据，
不过这个回溯并清理的过程有很多工作。另一种选择是直接 终止（abort），这会不清理数据就退出程序。那么程序所使用的内存需要由操作系统来清理。
如果你需要项目的最终二进制文件越小越好，panic 时通过在 Cargo.toml 的 [profile] 部分增加 panic = 'abort'，可以由展开切换为终止。例如，如果你想要在release模式中 panic 时直接终止：


让我们在一个简单的程序中调用 panic!：

文件名: src/main.rs

fn main() {
    panic!("crash and burn");
}

$ cargo run
   Compiling panic v0.1.0 (file:///projects/panic)
    Finished dev [unoptimized + debuginfo] target(s) in 0.25s
     Running `target/debug/panic`
thread 'main' panicked at 'crash and burn', src/main.rs:2:5
note: Run with `RUST_BACKTRACE=1` for a backtrace.

最后两行包含 panic! 调用造成的错误信息。第一行显示了 panic 提供的信息并指明了源码中 panic 出现的位置：src/main.rs:2:5 表明这是 src/main.rs 文件的第二行第五个字符。

在这个例子中，被指明的那一行是我们代码的一部分，而且查看这一行的话就会发现 panic! 宏的调用。在其他情况下，panic! 可能会出现在我们的代码所调用的代码中。
错误信息报告的文件名和行号可能指向别人代码中的 panic! 宏调用，
而不是我们代码中最终导致 panic! 的那一行。我们可以使用 panic! 被调用的函数的 backtrace 来寻找代码中出问题的地方。下面我们会详细介绍 backtrace 是什么。

[profile.release]
panic = 'abort'

使用 panic! 的 backtrace

让我们来看看另一个因为我们代码中的 bug 引起的别的库中 panic! 的例子，而不是直接的宏调用。示例 9-1 有一些尝试通过索引访问 vector 中元素的例子：

fn main() {
    let v = vec![1, 2, 3];

    v[99];
}
这里尝试访问 vector 的第 100 个元素（这里的索引是 99，因为索引从 0 开始），不过它只有 3 个元素。这种情况下 Rust 会 panic。[] 应当返回一个元素，但是如果传递了一个无效的索引，那么 Rust 在这里返回任何元素都不会是正确的。

这种情况下其他像 C 这样语言会尝试直接提供所要求的值，即便这可能不是你期望的：你会得到任何对应 vector 中这个元素的内存位置的值，甚至是这些内存并不属于 vector 的情况。这被称为 缓冲区溢出（buffer overread），并可能会导致安全漏洞，比如攻击者可以像这样操作索引来读取储存在数组后面不被允许的数据。

为了使程序远离这类漏洞，如果尝试读取一个索引不存在的元素，Rust 会停止执行并拒绝继续。尝试运行上面的程序会出现如下：

$ cargo run
   Compiling panic v0.1.0 (file:///projects/panic)
    Finished dev [unoptimized + debuginfo] target(s) in 0.27s
     Running `target/debug/panic`
thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 99', libcore/slice/mod.rs:2448:10
note: Run with `RUST_BACKTRACE=1` for a backtrace.

这指向了一个不是我们编写的文件，libcore/slice/mod.rs。其为 Rust 源码中 slice 的实现。这是当对 vector v 使用 [] 时 libcore/slice/mod.rs 中会执行的代码，也是真正出现 panic! 的地方。

接下来的几行提醒我们可以设置 RUST_BACKTRACE 环境变量来得到一个 backtrace。backtrace 是一个执行到目前位置所有被调用的函数的列表。Rust 的 backtrace 跟其他语言中的一样：阅读 backtrace 的关键是从头开始读直到发现你编写的文件。这就是问题的发源地。这一行往上是你的代码所调用的代码；往下则是调用你的代码的代码。这些行可能包含核心 Rust 代码，标准库代码或用到的 crate 代码。让我们将 RUST_BACKTRACE 环境变量设置为任何不是 0 的值来获取 backtrace 看看。



Result 与可恢复的错误
大部分错误并没有严重到需要程序完全停止执行。有时，一个函数会因为一个容易理解并做出反应的原因失败。例如，如果因为打开一个并不存在的文件而失败，此时我们可能想要创建这个文件，而不是终止进程。

回忆一下第 2 章 “使用 Result 类型来处理潜在的错误” 部分中的那个 Result 枚举，它定义有如下两个成员，Ok 和 Err：


#![allow(unused)]
fn main() {
enum Result<T, E> {
    Ok(T),
    Err(E),
}
}

T 和 E 是泛型类型参数；第 10 章会详细介绍泛型。现在你需要知道的就是 T 代表成功时返回的 Ok 成员中的数据的类型，而 E 代表失败时返回的 Err 成员中的错误的类型。
因为 Result 有这些泛型类型参数，我们可以将 Result 类型和标准库中为其定义的函数用于很多不同的场景，这些情况中需要返回的成功值和失败值可能会各不相同。


use std::fs::File;

fn main() {
    let f = File::open("hello.txt");
}
如何知道 File::open 返回一个 Result 呢？我们可以查看 标准库 API 文档，
或者可以直接问编译器！如果给 f 某个我们知道 不是 函数返回值类型的类型标注，接着尝试编译代码，编译器会告诉我们类型不匹配。
然后错误信息会告诉我们 f 的类型 应该 是什么。让我们试试！我们知道 File::open 的返回值不是 u32 类型的，所以将 let f 语句改为如下：

let f: u32 = File::open("hello.txt");

error[E0308]: mismatched types
 --> src/main.rs:4:18
  |
4 |     let f: u32 = File::open("hello.txt");
  |                  ^^^^^^^^^^^^^^^^^^^^^^^ expected u32, found enum
`std::result::Result`
  |
  = note: expected type `u32`
             found type `std::result::Result<std::fs::File, std::io::Error>`



匹配不同的错误
示例 9-4 中的代码不管 File::open 是因为什么原因失败都会 panic!。
我们真正希望的是对不同的错误原因采取不同的行为：如果 File::open 因为文件不存在而失败，
我们希望创建这个文件并返回新文件的句柄。如果 File::open 因为任何其他原因失败，例如没有打开文件的权限，我们仍然希望像示例 9-4 那样 panic!。
让我们看看示例 9-5，其中 match 增加了另一个分支：
*/

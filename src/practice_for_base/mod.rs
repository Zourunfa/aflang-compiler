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

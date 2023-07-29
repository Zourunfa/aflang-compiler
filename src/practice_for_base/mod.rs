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
let origin = Point::origin();


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

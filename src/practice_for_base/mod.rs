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
    num = 10;
    // return y;
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

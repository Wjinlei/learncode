use std::thread;
use std::time::Duration;

// 安全且高效的处理并发编程是 Rust 的另一个主要目标。
// 并发编程（Concurrent programming），代表程序的不同部分相互独立的执行
// 并行编程（parallel programming）代表程序不同部分于同时执行

// 在大部分现代操作系统中，已执行程序的代码在一个 进程（process）中运行，操作系统则负责管理多个进程。
// 在程序内部，也可以拥有多个同时运行的独立部分。运行这些独立部分的功能被称为 线程（threads）。
// 将程序中的计算拆分进多个线程可以改善性能，因为程序可以同时进行多个任务，不过这也会增加复杂性。
// 因为线程是同时运行的，所以无法预先保证不同线程中的代码的执行顺序。这会导致诸如此类的问题：
//     竞争状态（Race conditions），多个线程以不一致的顺序访问数据或资源
//     死锁（Deadlocks），两个线程相互等待对方停止使用其所拥有的资源，这会阻止它们继续运行
//     只会发生在特定情况且难以稳定重现和修复的 bug
// Rust 尝试减轻使用线程的负面影响。
// 不过在多线程上下文中编程仍需格外小心，同时其所要求的代码结构也不同于运行于单线程的程序。

// 编程语言有一些不同的方法来实现线程。很多操作系统提供了创建新线程的 API。
// 这种由编程语言调用操作系统 API 创建线程的模型有时被称为 1:1，一个 OS 线程对应一个语言线程。
// 很多编程语言提供了自己特殊的线程实现。
// 编程语言提供的线程被称为 绿色（green）线程，使用绿色线程的语言会在不同数量的 OS 线程的上下文中执行它们。
// 为此，绿色线程模式被称为 M:N 模型：M 个绿色线程对应 N 个 OS 线程，这里 M 和 N 不必相同。
// 每一个模型都有其优势和取舍。对于 Rust 来说最重要的取舍是运行时支持。
// 运行时（Runtime）是一个令人迷惑的概念，其在不同上下文中可能有不同的含义。

// 在当前上下文中，运行时 代表二进制文件中包含的由语言自身提供的代码。
// 这些代码根据语言的不同可大可小，不过任何非汇编语言都会有一定数量的运行时代码。
// 为此，通常人们说一个语言 “没有运行时”，一般意味着 “小运行时”。
// 更小的运行时拥有更少的功能不过其优势在于更小的二进制输出，这使其易于在更多上下文中与其他语言相结合。
// 虽然很多语言觉得增加运行时来换取更多功能没有什么问题，
// 但是 Rust 需要做到几乎没有运行时，同时为了保持高性能必须能够调用 C 语言，这点也是不能妥协的。
// 绿色线程的 M:N 模型需要更大的语言运行时来管理这些线程。因此Rust 标准库只提供了 1:1 线程模型实现。

fn main() {
    let v = vec![1, 2, 3];

    // 为了创建一个新线程，需要调用 thread::spawn 函数并传递一个闭包，并在其中包含希望在新线程运行的代码。
    let handle = thread::spawn(move || {
        // 为闭包函数增加move 关键字强制闭包获取其使用的环境值的所有权。
        println!("Here's a vector: {:?}", v); // 增加move关键字后就可以获得v的所有权了
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            //thread::sleep 调用强制线程停止执行一小段时间，这会允许其他不同的线程运行。
            thread::sleep(Duration::from_millis(1));
        }
    });

    handle.join().unwrap();

    // 主线程会等待直到新建线程执行完毕之后才开始执行 for 循环，所以输出将不会交替出现
    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    // 由于主线程结束，上面代码大部分时候不光会提早结束新建线程
    // 甚至不能实际保证新建线程会被执行。其原因在于无法保证线程运行的顺序！
    // 可以通过将 thread::spawn 的返回值储存在变量中来修复新建线程部分没有执行或者完全没有执行的问题。
    // thread::spawn 的返回值类型是 JoinHandle
    // JoinHandle 是一个拥有所有权的值，当对其调用 join 方法时，它会等待其线程结束。

    //handle.join().unwrap(); // thread::spawn 保存一个 JoinHandle 以确保该线程能够运行至结束

    // 这两个线程仍然会交替执行，不过主线程会由于 handle.join() 调用会等待直到新建线程执行完毕。
    // 让我们不过让我们看看将 handle.join() 移动到 main 中 for 循环之前会发生什么
}

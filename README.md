# Just for learning



## plan list

### 基础知识
1. **Rust语言基础**：
   - 熟悉Rust的基本语法和数据结构
   - 理解所有权、借用和生命周期

2. **标准库**：
   - 学习Rust标准库中的并发模块，例如`std::thread`和`std::sync`
   - 掌握异步编程基础，例如`std::future`和`async/await`

### 高并发开发
1. **线程和同步**：
   - 创建和管理线程 (`std::thread`)✅day16
   - 线程间同步机制（互斥锁 `Mutex`，读写锁 `RwLock`）

2. **消息传递**：
   - 使用通道进行线程间通信 (`std::sync::mpsc`)

3. **异步编程**：
   - 异步函数和`async/await`语法
   - 异步任务和执行器（如`tokio`或`async-std`）

4. **并发数据结构**：
   - 学习常用并发数据结构，如`Arc`，`Atomic`类型
   - 掌握无锁数据结构的使用和设计

### 高性能开发
1. **内存管理**：
   - 深入理解Rust的所有权模型和生命周期
   - 学习如何避免内存泄漏和悬挂指针

2. **性能优化**：
   - 掌握性能分析工具（如`cargo flamegraph`、`perf`等）
   - 了解缓存友好性和内存对齐
   - 使用内联和宏优化性能

3. **高效IO操作**：
   - 异步IO库（如`tokio`、`async-std`）
   - 零拷贝技术（如`mmap`）

4. **并行计算**：
   - 使用Rayon进行数据并行计算
   - 了解SIMD和GPU加速技术

### 实践与进阶
1. **开源项目**：
   - 阅读和贡献Rust高并发项目的源码（如`tokio`、`actix`）
   - 参与Rust社区活动和讨论

2. **设计模式**：
   - 学习并发设计模式，如生产者-消费者、工作窃取
   - 了解常见的分布式系统设计

3. **实战项目**：
   - 构建高并发的Web服务器或微服务框架
   - 开发高性能的网络应用或数据库系统

### 资源与工具
1. **书籍与教程**：
   - 《The Rust Programming Language》（Rust官方书籍）
   - 《Programming Rust》（O'Reilly出版）

2. **在线资源**：
   - Rust官方文档和Rustonomicon
   - Rust社区和论坛（如users.rust-lang.org）

3. **开发工具**：
   - 使用IDE（如VSCode或IntelliJ Rust插件）
   - 利用调试器和性能分析工具（如GDB、Valgrind）

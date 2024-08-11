# rustLang_learning
Everyday keep learning rustLang   |    每天保持学习rust编程语言



# List day07

### 线性数据结构

1. **数组（Array）**：✅
   - **描述**：固定大小的连续内存块，每个元素可以通过索引访问。
   - **特点**：快速随机访问，插入和删除操作慢。
   - **应用**：常用于需要快速访问数据的场景，如静态表、缓冲区等。

2. **链表（Linked List）**：day09✅
   - **描述**：由节点组成的线性结构，每个节点包含数据和指向下一个节点的指针。
   - **类型**：单向链表、双向链表、循环链表。
   - **特点**：插入和删除操作快，随机访问慢。
   - **应用**：常用于动态数据结构，如队列、堆栈等。

3. **栈（Stack）**：✅
   - **描述**：后进先出（LIFO）的线性结构，只允许在一端进行插入和删除操作。
   - **特点**：插入和删除操作快。
   - **应用**：常用于递归算法、表达式求值等场景。

4. **队列（Queue）**：✅
   - **描述**：先进先出（FIFO）的线性结构，只允许在一端进行插入操作，在另一端进行删除操作。
   - **类型**：普通队列、双端队列、优先队列。
   - **特点**：插入和删除操作快。
   - **应用**：常用于任务调度、缓冲区管理等场景。

### 非线性数据结构

1. **树（Tree）**：
   - **描述**：由节点组成的层次结构，每个节点包含数据和指向子节点的指针。
   - **类型**：二叉树day10✅、二叉搜索树、平衡树（如AVL树、红黑树）、B树、B+树等。
   - **特点**：快速查找、插入和删除操作。
   - **应用**：常用于数据库索引、文件系统、语法解析等场景。

2. **图（Graph）**：
   - **描述**：由节点（顶点）和边组成的结构，边可以有方向（有向图）或无方向（无向图）day11✅。
   - **特点**：可以表示复杂的关系。
   - **应用**：常用于网络路由、社交网络分析、图像处理等场景。

### 哈希结构

1. **哈希表（Hash Table）**：day12✅
   - **描述**：通过哈希函数将键映射到数组中的位置，以实现快速查找。
   - **特点**：快速查找、插入和删除操作。
   - **应用**：常用于数据库索引、缓存等场景。

### 特殊数据结构

1. **堆（Heap）**：
   - **描述**：一种特殊的树形数据结构，满足堆属性（最大堆或最小堆）。
   - **特点**：可以高效地进行最大值或最小值的查找、插入和删除操作。
   - **应用**：常用于优先队列、排序算法（如堆排序✅day13）等场景。

2. **Trie（字典树）**：✅day14
   - **描述**：一种用于存储字符串的树形结构，每个节点表示一个字符串前缀。
   - **特点**：高效的字符串查找和前缀匹配。
   - **应用**：常用于自动补全、拼写检查、前缀查询等场景。

3. **并查集（Union-Find）**：
   - **描述**：用于处理不相交集合的数据结构，支持合并和查找操作。
   - **特点**：快速的合并和查找操作。
   - **应用**：常用于网络连接性问题、最小生成树算法✅day15等场景。

### 选择数据结构时的考虑因素

1. **访问和操作性能**：不同数据结构在访问、插入、删除等操作上的性能差异。
2. **内存使用**：一些数据结构可能需要更多的内存来存储额外的信息。
3. **实现复杂度**：有些数据结构实现较为复杂，需要更多的编程工作。
4. **应用场景**：选择最适合特定应用场景的数据结构，以达到最佳性能。
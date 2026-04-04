# Data structure

## Standard library
### Containers
| Container | C++ | Java |
| --------- | --- | ---- |
| 可选值 | [`std::optional` (C++17)](https://en.cppreference.com/w/cpp/utility/optional.html) | |
| 元组 | [`std::tuple` (C++11)](https://en.cppreference.com/w/cpp/utility/tuple.html) | |
| 静态数组 | [`std::array`](https://en.cppreference.com/w/cpp/container/array.html) | |
| 动态数组 | [`std::vector`](https://en.cppreference.com/w/cpp/container/vector.html), [`std::vector<T,Allocator>::vector`](https://en.cppreference.com/w/cpp/container/vector/vector.html), [`std::vector<T,Allocator>::operator[]`](https://en.cppreference.com/w/cpp/container/vector/operator_at.html) | `java.util.ArrayList` |
| 栈 | [`std::stack`](https://en.cppreference.com/w/cpp/container/stack.html), [`std::stack<T,Container>::stack`](https://en.cppreference.com/w/cpp/container/stack/stack.html) | |
| 队列 | [`std::queue`](https://en.cppreference.com/w/cpp/container/queue.html), [`std::queue<T,Container>::queue`](https://en.cppreference.com/w/cpp/container/queue/queue.html) | |
| 优先队列 | [`std::priority_queue`](https://en.cppreference.com/w/cpp/container/priority_queue.html), [`std::priority_queue<T,Container,Compare>::priority_queue`](https://en.cppreference.com/w/cpp/container/priority_queue/priority_queue.html) | |
| 有序集合 | [`std::set`](https://en.cppreference.com/w/cpp/container/set.html) | |
| 无序集合 | [`std::unordered_set`](https://en.cppreference.com/w/cpp/container/unordered_set.html) | |
| 无序映射/（可能是）哈希表 | [`std::unordered_map` (C++11)](https://en.cppreference.com/w/cpp/container/unordered_map.html) | |
| 定长位集 | [`std::bitset`](https://en.cppreference.com/w/cpp/utility/bitset.html) | `java.util.BitSet` (unexamined) |

### Interfaces
| Interface | Java |
| --------- | ---- |
| 队列 | `java.util.Queue` |

## Self-defined
### Containers
| Containers | C++ |
| ---------- | --- |
| 并查集 | `disjoint_set` |

# Data structure

## Standard library
### Containers
| Container | C++ | Java |
| --------- | --- | ---- |
| | [`std::optional` (C++17)](https://en.cppreference.com/w/cpp/utility/optional.html) | |
| 元组 | [`std::tuple` (C++11)](https://en.cppreference.com/w/cpp/utility/tuple.html) | |
| 静态数组 | [`std::array`](https://en.cppreference.com/w/cpp/container/array.html) | |
| 动态数组 | [`std::vector`](https://en.cppreference.com/w/cpp/container/vector.html) | `java.util.ArrayList` |
| 优先队列 | [`std::priority_queue`](https://en.cppreference.com/w/cpp/container/priority_queue.html) | |
| 有序集合 | [`std::set`](https://en.cppreference.com/w/cpp/container/set.html) | |
| 无序集合 | [`std::unordered_set`](https://en.cppreference.com/w/cpp/container/unordered_set.html) | |
| 位集 | [`std::bitset`](https://en.cppreference.com/w/cpp/utility/bitset.html) | `java.util.BitSet` |

### Views
| Container | C++ |
| --------- | --- |
| 空视图 | [`std::ranges::views::single` (C++20), `std::ranges::single_view` (C++20)](https://en.cppreference.com/w/cpp/ranges/single_view.html) |

### Interfaces
| Interface | Java |
| --------- | ---- |
| 队列 | `java.util.Queue` |

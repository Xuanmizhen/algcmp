# Sorting

## Standard library
### Functions
| Functionality | C++ | Python |
| ------------- | --- | ------ |
| 排序 | [`std::sort`](https://en.cppreference.com/w/cpp/algorithm/sort.html) | |
| 基于范围排序 | [`std::ranges::sort` (C++20)](https://en.cppreference.com/w/cpp/algorithm/ranges/sort.html) | |
| 稳定排序 | [`std::stable_sort`](https://en.cppreference.com/w/cpp/algorithm/stable_sort.html) | |
| 基于范围稳定排序 | [`std::ranges::stable_sort` (C++20)](https://en.cppreference.com/w/cpp/algorithm/ranges/stable_sort.html) | |
| 对前 n 大的元素排序 | | `heapq.nlargest` |
| 对前 n 小的元素排序 | | `heapq.nsmallest` |

### Concepts
| Concept | C++ |
| ------- | --- |
| 随机访问范围 | [`std::ranges::random_access_range` (C++20)](https://en.cppreference.com/w/cpp/ranges/random_access_range.html) |
| 随机访问迭代器 | [`std::random_access_iterator` (C++20)](https://en.cppreference.com/w/cpp/iterator/random_access_iterator.html) |

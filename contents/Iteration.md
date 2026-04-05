# Iteration

## Standard library
### Functions
| Functionality | C++ | Python |
| ------------- | --- | ------ |
| 懒重复 | | `itertools.repeat` |
| 计数 | [`std::ranges::count`, `std::ranges::count_if`](https://en.cppreference.com/w/cpp/algorithm/ranges/count.html) | `len` |
| 累积 | [`std::accumulate`](https://en.cppreference.com/w/cpp/algorithm/accumulate.html) | |
| 计算最小最大值 | [`std::ranges::minmax` (C++20), `std::ranges::minmax_result` (C++20)](https://en.cppreference.com/w/cpp/algorithm/ranges/minmax.html) | |
| 计算最大值 | [`std::ranges::max_element` (C++20)](https://en.cppreference.com/w/cpp/algorithm/ranges/max_element.html) | `max` |
| 去重 | [`std::ranges::unique` (C++20)](https://en.cppreference.com/w/cpp/algorithm/ranges/unique.html) | |
| 顺序查找 | [`std::ranges::find` (C++20), `std::ranges::find_if` (C++20), `std::ranges::find_if_not` (C++20)](https://en.cppreference.com/w/cpp/algorithm/ranges/find.html) | |
| 倒序查找 | [`std::ranges::find_last` (C++23), `std::ranges::find_last_if` (C++23), `std::ranges::find_last_if_not` (C++23)](https://en.cppreference.com/w/cpp/algorithm/ranges/find_last.html) | |
| 倒序区间查找 | [`std::ranges::find_end` (C++20)](https://en.cppreference.com/w/cpp/algorithm/ranges/find_end.html) | |
| 顺序查找另一给定集合中的任一元素 | [`std::ranges::find_first_of` (C++20)](https://en.cppreference.com/w/cpp/algorithm/ranges/find_first_of.html) | |
| 顺序查找连续相等元素 | [`std::ranges::adjacent_find` (C++20)](https://en.cppreference.com/w/cpp/algorithm/ranges/adjacent_find.html) | |
| 顺序查找两个范围中不匹配的值 | [`std::ranges::mismatch` (C++20)](https://en.cppreference.com/w/cpp/algorithm/ranges/mismatch.html) | |
| 求前缀和 | [`std::partial_sum`](https://en.cppreference.com/w/cpp/algorithm/partial_sum.html) | |
| 判断谓词可满足性 | [`std::ranges::all_of` (C++20), `std::ranges::any_of` (C++20), `std::ranges::none_of` (C++20)](https://en.cppreference.com/w/cpp/algorithm/ranges/all_any_none_of.html) | |
| 判断存在性 | [`std::ranges::contains` (C++23), `std::ranges::contains_subrange` (C++23)](https://en.cppreference.com/w/cpp/algorithm/ranges/contains.html) | |

### 链式调用
| Operation | C++ | Python |
| --------- | --- | ------ |
| 截断 | [`std::ranges::take_view` (C++20), `std::views::take` (C++20)](https://en.cppreference.com/w/cpp/ranges/take_view.html) | |
| 反转 | [`std::ranges::reverse_view` (C++20), `std::views::reverse` (C++20)](https://en.cppreference.com/w/cpp/ranges/reverse_view.html) | `reversed` |
| 过滤 | [`std::ranges::filter_view` (C++20), `std::views::filter` (C++20)](https://en.cppreference.com/w/cpp/ranges/filter_view.html) | `filter` |
| 映射 | [`std::ranges::transform_view` (C++20), `std::views::transform` (C++20)](https://en.cppreference.com/w/cpp/ranges/transform_view.html) | `map` |
| 取键 | [`std::ranges::keys_view` (C++20), `std::views::keys` (C++20)](https://en.cppreference.com/w/cpp/ranges/keys_view.html) | |
| 取值 | [`std::ranges::values_view` (C++20), `std::views::values` (C++20)](https://en.cppreference.com/w/cpp/ranges/values_view.html) | |

### Concepts
| Concept | C++ |
| ------- | --- |
| 随机访问迭代器 | [`std::random_access_iterator` (C++20)](https://en.cppreference.com/w/cpp/iterator/random_access_iterator.html) |
| 随机访问范围 | [`std::random_access_range` (C++20)](https://en.cppreference.com/w/cpp/ranges/random_access_range.html) |

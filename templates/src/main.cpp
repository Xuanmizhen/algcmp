// C++20
// 牛客竞赛: C++ (clang++18) or C++ (g++ 13)
// PTA: C++ (g++)
// QOJ.ac: C++20 or C++23 or C++26

#include <cassert> // assert
#include <compare> // std::strong_ordering
#include <cstdint>
#include <fstream>
#include <iostream>
#include <numbers> // std::numbers::pi_v, std::numbers::pi, std::numbers::inv_pi_v, std::numbers::inv_pi
#include <optional> // std::optional
#include <queue> // std::priority_queue
#include <source_location>
#include <string_view>
#include <type_traits> // std::make_signed_t
#include <vector>

// #include <bits/stdc++.h> // g++


// 缩写和语法糖

using i16 = int16_t;
using i32 = int32_t;
using i64 = int64_t;
using isize = std::make_signed_t<size_t>;
using i128 = __int128;

using u16 = uint16_t;
using u32 = uint32_t;
using u64 = uint64_t;
using usize = size_t;
using u128 = __uint128_t;

using std::cin;
using std::cout;
using std::source_location;

#define let const auto
#define var auto
#define fn auto
#define loop for (;;)


// 向 C++26 看齐

#if __cplusplus <= 202302L
// Computes the addition `x + y` and stores the result into `*result`. The addition is performed as if both operands were represented in a signed integer type with infinite range, and the result was then converted from this integer type to `type1`. If the value assigned to `*result` correctly represents the mathematical result of the operation, it returns `false`. Otherwise, it returns `true`. In this case, the value assigned to *result is the mathematical result of the operation wrapped around to the width of `*result`.
template<std::unsigned_integral type1>
fn ckd_add(type1* result, type1 a, type1 b) -> bool {
    return __builtin_add_overflow(a, b, result);
}
// Computes the multiplication `x × y` and stores the result into `*result`. The multiplication is performed as if both operands were represented in a signed integer type with infinite range, and the result was then converted from this integer type to `type1`. If the value assigned to `*result` correctly represents the mathematical result of the operation, it returns `false`. Otherwise, it returns `true`. In this case, the value assigned to `*result` is the mathematical result of the operation wrapped around to the width of `*result`.
template<std::unsigned_integral type1>
fn ckd_mul(type1* result, type1 a, type1 b) -> bool {
    return __builtin_mul_overflow(a, b, result);
}
#else
#include <stdckdint.h> // C++26
#endif


// 调试工具

fn log_loc(std::ostream &os, const source_location loc = source_location::current()) -> std::ostream & {
    return os << '[' << loc.file_name() << ':' << loc.line() << ':' << loc.column() << "] `" << loc.function_name() << "`: ";
}

#ifdef LOCAL
#define dbg(val) \
    ([](const decltype((val)) v, const source_location loc) -> decltype((val)) { \
        log_loc(std::clog, loc) << " (" << #val << ") = " << v << std::endl; \
        return (v); \
    })((val), source_location::current())
#define debug_assert(e) assert(e)
#define todo() (throw std::runtime_error("not yet implemented"))
#else
#define dbg(val) val
#define debug_assert(e) ((void) 0)
#define todo() static_assert(false)
#endif


// 圆周率

namespace std {
namespace numbers {
template <class T> inline constexpr T tau_v = 2 * pi_v<T>;
template <class T> inline constexpr T inv_tau_v = inv_pi_v<T> / static_cast<T>(2.0);
inline constexpr double tau = tau_v<double>;
inline constexpr double inv_tau = inv_tau_v<double>;
}
}


// 线性代数

template<class T, class Container = std::vector<std::vector<T>>>
class matrix {
public:
    using row_type = Container::value_type;
    using value_type = row_type::value_type;
    static_assert(std::is_same_v<value_type, T>);
    Container inner;

    matrix() : matrix(Container()) { }
    explicit matrix(const Container &cont) : inner(cont) { }
    explicit matrix(Container&& cont) : inner(cont) { }
    matrix(const usize n, const usize m) : inner(n, row_type(m)) { }
    // reference operator[](usize pos) {
    //     return inner[pos];
    // }
    // const_reference operator[](usize pos) const {
    //     return inner[pos];
    // }
};


// 并查集

class disjoint_set {
    std::vector<isize> parent_or_neg_rank;
    fn link(usize x, usize y) -> uint8_t {
        let cmp = parent_or_neg_rank[x] <=> parent_or_neg_rank[y];
        if (cmp < 0) {
            parent_or_neg_rank[y] = x;
            return 0;
        }
        if (cmp == 0) {
            --parent_or_neg_rank[y];
        }
        parent_or_neg_rank[x] = y;
        return 1;
    }

public:
    explicit disjoint_set(const usize n) : parent_or_neg_rank(n, -1) { }
    fn find_set(usize x) -> usize {
        var root = x;
        while (parent_or_neg_rank[root] >= 0) {
            root = parent_or_neg_rank[root];
        }
        while (x != root) {
            let next = parent_or_neg_rank[x];
            parent_or_neg_rank[x] = root;
            x = next;
        }
        return root;
    }
    struct unite_result {
        usize union_repr;
        std::optional<usize> deleted_repr;
    };
    unite_result unite(usize x, usize y) {
        if ((x = find_set(x)) == (y = find_set(y))) {
            return {x, {}};
        }
        if (link(x, y) == 0) {
            return {x, y};
        } else {
            return {y, x};
        }
    }
};


// 图论

template <std::three_way_comparable W>
class undirected_edge {
public:
    usize u, v;
    W weight;

    fn operator<=>(const undirected_edge& rhs) const -> std::weak_ordering {
        return rhs.weight <=> weight;
    }
};

template <std::three_way_comparable W>
fn kruskal_safe_edge(disjoint_set &components, std::priority_queue<undirected_edge<W>> &q) -> std::optional<undirected_edge<W>> {
    while (!q.empty()) {
        let e = q.top();
        q.pop();
        if (components.unite(e.u, e.v).deleted_repr.has_value()) {
            return e;
        }
    }
    return {};
}

template<typename C>
struct action
{
    usize dest;
    C cost;
};

template<typename W>
class directed_edge
{
public:
    usize source;
    action<W> act;

    directed_edge(const usize src, const usize dst, const W weight) : source(src), act(dst, weight) { }
};


// 快速幂
template<std::integral T, std::unsigned_integral E>
fn powi(T base, E exp) -> T {
    var res = T(1);
    while (exp > 0) {
        if (exp % 2 == 1) {
            res *= base;
        }
        base *= base;
        exp /= 2;
    }
    return res;
}


// 溢出标记

template<std::unsigned_integral I>
class overflowable {
    std::optional<I> inner;
    overflowable() { }

public:
    overflowable(const I val) : inner(val) { }
    fn overflowed() const -> bool {
        return !inner.has_value();
    }
    fn value() -> std::optional<I> {
        return inner;
    }
    fn operator+(const I val) -> overflowable {
        if (inner.has_value()) {
            I res;
            return ckd_add(&res, inner.value(), val) ? overflowable() : overflowable(res);
        }
        return overflowable();
    }
    fn operator+=(const I val) -> overflowable & {
        return *this = *this + val;
    }
    fn operator*(const I val) -> overflowable {
        if (inner.has_value()) {
            I res;
            return ckd_mul(&res, inner.value(), val) ? overflowable() : overflowable(res);
        }
        return overflowable();
    }
    fn operator*(const overflowable& rhs) -> overflowable {
        if (inner.has_value() && rhs.inner.has_value()) {
            I res;
            return ckd_mul(&res, inner.value(), rhs.inner.value()) ? overflowable() : overflowable(res);
        }
        return overflowable();
    }
    fn operator*=(const I val) -> overflowable & {
        return *this = *this * val;
    }
    fn operator*=(const overflowable& rhs) -> overflowable & {
        return *this = *this * rhs;
    }
    fn operator<=>(const overflowable& rhs) const -> std::partial_ordering {
        if (!inner.has_value() && !rhs.inner.has_value()) {
            // overflowable() cannot compare with itself
            return std::partial_ordering::unordered;
        }
        if (!inner.has_value()) {
            // this is overflowable(), greater than any I
            return std::partial_ordering::greater;
        }
        if (!rhs.inner.has_value()) {
            // rhs is overflowable(), less than overflowable()
            return std::partial_ordering::less;
        }
        return std::partial_ordering(inner.value() <=> rhs.inner.value());
    }
    fn operator<(const overflowable& rhs) const -> bool {
        return (*this <=> rhs) < 0;
    }
};


// 模意义下的计算

template<std::unsigned_integral Inner, Inner M>
class mod_unsigned_unchecked {
public:
    Inner inner;

    mod_unsigned_unchecked(const Inner val) : inner(val) {
        debug_assert(val < M);
    }

    fn operator+=(const mod_unsigned_unchecked<Inner, M> &&rhs) -> mod_unsigned_unchecked & {
        inner = (inner + rhs.inner) % M;
        return *this;
    }
    fn operator-=(const mod_unsigned_unchecked<Inner, M> &&rhs) -> mod_unsigned_unchecked & {
        inner = (inner - rhs.inner) % M;
        return *this;
    }
    fn operator*=(const mod_unsigned_unchecked<Inner, M> &&rhs) -> mod_unsigned_unchecked & {
        inner = (inner * rhs.inner) % M;
        return *this;
    }
};


// 最长公共子序列

// longest-common-subsequence problem
template<std::ranges::input_range R>
fn lcs(R&& a, R&& b) -> usize {
    usize cnt{0};
    todo();
    return cnt;
}


// 测试

fn test() {
    var a = mod_unsigned_unchecked<u32, 2017>{5};
    a *= 20;
    assert(dbg(a.inner == 100));
    a *= 21;
    assert(dbg(dbg(a.inner) += 3) == 2103 % 2017);
    var b = overflowable<uint8_t>{255};
    assert((b += uint8_t{1}).overflowed());
    assert(powi(-3, u16{3}) == -27);
}


// 任务

fn run() {
    u32 t;
    cin >> t;
    while (t-- > 0) {
        // Process each test case
    }
}

fn main() -> int {
    using namespace std;

#ifdef LOCAL
    // 测试
    test();
    clog << "Test passed" << endl;
#endif

    ios_base::sync_with_stdio(false);

#if defined(LOCAL) && defined(REDIRECT)
    ifstream fin("in.txt");
    cin.rdbuf(fin.rdbuf());
    if (!fin.is_open()) {
        cerr << "in.txt not opened" << endl;
        return 1;
    }
    ofstream fout("out.txt");
    if (!fout.is_open()) {
        cerr << "out.txt not opened" << endl;
        return 1;
    }
    cout.rdbuf(fout.rdbuf());
#endif

#if !defined(LOCAL) || defined(REDIRECT)
    cin.tie(nullptr);
#endif

    run();
}

// C++20
// 牛客竞赛: C++ (clang++18) or C++ (g++ 13)
// PTA: C++ (g++)
// QOJ.ac: C++20 or C++23 or C++26

static_assert(__cplusplus >= 202002L);


#include <cassert> // assert
#include <compare> // std::strong_ordering
#include <cstdint>
#include <fstream>
#include <iostream>
#include <limits>   // std::numeric_limits
#include <numbers>  // std::numbers::pi_v, std::numbers::pi, std::numbers::inv_pi_v, std::numbers::inv_pi
#include <numeric>  // std::accumulate
#include <optional> // std::optional
#include <queue>    // std::priority_queue
#include <ranges>   // std::views::iota
#include <source_location>
#include <string_view>
#include <type_traits> // std::make_signed_t
#include <vector>

// #include <bits/stdc++.h> // g++


// 进口

#define let const auto
#define var auto
#define fn auto
#define loop for (;;)
#define in :
#define self (*this)

using u16 = uint16_t;
using i16 = int16_t;
using u32 = uint32_t;
using i32 = int32_t;
using u64 = uint64_t;
using i64 = int64_t;
using u128 = __uint128_t;
using i128 = __int128;
using usize = size_t;
using isize = std::make_signed_t<usize>;
template <typename T>
using Option = std::optional<T>;
template <class T>
constexpr std::optional<std::decay_t<T>> Some(T &&value) {
    return std::make_optional(value);
}
inline constexpr std::nullopt_t None{std::nullopt};

template <typename I>
class Cycle;

template <typename Derived, typename T>
class Iterator {
public:
    using Item = T;

    fn next() -> Option<Item> {
        return static_cast<Derived *>(this)->next();
    }

    template <typename B, typename F>
    fn fold(const B init, F f) -> B {
        var accum = init;
        while (let item = self.next()) {
            accum = f(accum, item);
        }
        return accum;
    }

    template <typename F>
    fn for_each(F f) {
        while (let item = self.next()) {
            f(item.value());
        }
    }

    fn cycle() -> Cycle<Derived> {
        return Cycle<Derived>(*static_cast<Derived *>(this), *static_cast<Derived *>(this));
    }
};

template <typename I>
class Cycle : public Iterator<Cycle<I>, typename I::Item> {
    I orig;
    I iter;
    Cycle(I orig, I iter) : orig(orig), iter(iter) { }

public:
    template <typename Derived, typename T>
    friend class Iterator;

    fn next() -> Option<typename I::Item> {
        if (let y = self.iter.next()) {
            return y;
        } else {
            self.iter = self.orig;
            return self.iter.next();
        }
    }
};


// 向 C++26 看齐

#if __cplusplus <= 202302L
// Computes the addition `x + y` and stores the result into `*result`. The addition is performed as if both operands were represented in a signed integer type with infinite range, and the result was then converted from this integer type to `type1`. If the value assigned to `*result` correctly represents the mathematical result of the operation, it returns `false`. Otherwise, it returns `true`. In this case, the value assigned to *result is the mathematical result of the operation wrapped around to the width of `*result`.
template <std::unsigned_integral type1>
fn ckd_add(type1 *result, type1 a, type1 b) -> bool {
    return __builtin_add_overflow(a, b, result);
}
// Computes the multiplication `x × y` and stores the result into `*result`. The multiplication is performed as if both operands were represented in a signed integer type with infinite range, and the result was then converted from this integer type to `type1`. If the value assigned to `*result` correctly represents the mathematical result of the operation, it returns `false`. Otherwise, it returns `true`. In this case, the value assigned to `*result` is the mathematical result of the operation wrapped around to the width of `*result`.
template <std::unsigned_integral type1>
fn ckd_mul(type1 *result, type1 a, type1 b) -> bool {
    return __builtin_mul_overflow(a, b, result);
}
#else // C++26
#include <stdckdint.h>
#endif


// 调试工具

fn log_loc(std::ostream &os, const std::source_location loc = std::source_location::current()) -> std::ostream & {
    return os << '[' << loc.file_name() << ':' << loc.line() << ':' << loc.column() << "] `" << loc.function_name() << "`: ";
}

#ifdef LOCAL
#define dbg(val) \
    ([](const decltype((val)) v, const std::source_location loc) -> decltype((val)) { \
        log_loc(std::clog, loc) << " (" << #val << ") = " << v << std::endl; \
        return (v); \
    })((val), std::source_location::current())
#define debug_assert(e) assert(e)
#define todo() (throw std::runtime_error("not yet implemented"))
#else
#define dbg(val) val
#define debug_assert(e) ((void)0)
#define todo() static_assert(false)
#endif


// 圆周率

namespace std {
    namespace numbers {
        template <typename T>
        inline constexpr T tau_v = 2 * pi_v<T>;
        template <typename T>
        inline constexpr T inv_tau_v = inv_pi_v<T> / static_cast<T>(2.0);
        inline constexpr double tau = tau_v<double>;
        inline constexpr double inv_tau = inv_tau_v<double>;
    }
}


// 并查集

class disjoint_set {
    std::vector<isize> parent_or_neg_rank;
    fn link(usize x, usize y) -> uint8_t {
        let cmp = self.parent_or_neg_rank[x] <=> self.parent_or_neg_rank[y];
        if (cmp < 0) {
            self.parent_or_neg_rank[y] = x;
            return 0;
        }
        if (cmp == 0) {
            --self.parent_or_neg_rank[y];
        }
        self.parent_or_neg_rank[x] = y;
        return 1;
    }

public:
    explicit disjoint_set(const usize n) : parent_or_neg_rank(n, -1) { }
    fn find_set(usize x) -> usize {
        var root = x;
        while (self.parent_or_neg_rank[root] >= 0) {
            root = self.parent_or_neg_rank[root];
        }
        while (x != root) {
            let next = self.parent_or_neg_rank[x];
            self.parent_or_neg_rank[x] = root;
            x = next;
        }
        return root;
    }
    struct unite_result {
        usize union_repr;
        Option<usize> deleted_repr;
    };
    fn unite(usize x, usize y) -> unite_result {
        if ((x = self.find_set(x)) == (y = self.find_set(y))) {
            return {x, None};
        }
        if (self.link(x, y) == 0) {
            return {x, Some(y)};
        } else {
            return {y, Some(x)};
        }
    }
};


// 图论

template <std::three_way_comparable W>
class undirected_edge {
public:
    usize u, v;
    W weight;

    fn operator<=>(const undirected_edge &rhs) const -> std::weak_ordering {
        return rhs.weight <=> self.weight;
    }
};

template <std::three_way_comparable W>
fn kruskal_safe_edge(disjoint_set &components, std::priority_queue<undirected_edge<W>> &q) -> Option<undirected_edge<W>> {
    while (!q.empty()) {
        let e = q.top();
        q.pop();
        if (components.unite(e.u, e.v).deleted_repr.has_value()) {
            return Some(e);
        }
    }
    return None;
}

template <typename C>
struct action {
    usize dest;
    C cost;
};

template <typename W>
class directed_edge {
public:
    usize source;
    action<W> act;

    directed_edge(const usize src, const usize dst, const W weight) : source(src), act(dst, weight) { }
};


// 快速幂
template <std::integral T, std::unsigned_integral E>
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

template <std::unsigned_integral I>
class overflowable {
    Option<I> inner;
    overflowable() { }

public:
    overflowable(const I val) : inner(val) { }
    fn overflowed() const -> bool {
        return !inner.has_value();
    }
    fn value() -> Option<I> {
        return self.inner;
    }
    fn operator++() -> overflowable & {
        if (!overflowed()) {
            if (inner.value() == std::numeric_limits<I>::max()) {
                inner.reset();
            } else {
                ++inner.value();
            }
        }
        return self;
    }
    fn operator++(int) -> overflowable {
        let original = self;
        ++self;
        return original;
    }
    fn operator+(const I val) const -> overflowable {
        if (!overflowed()) {
            I res;
            return ckd_add(&res, self.inner.value(), val) ? overflowable() : overflowable(res);
        }
        return overflowable();
    }
    fn operator+=(const I val) -> overflowable & {
        return self = self + val;
    }
    fn operator*(const I val) const -> overflowable {
        if (!overflowed()) {
            I res;
            return ckd_mul(&res, self.inner.value(), val) ? overflowable() : overflowable(res);
        }
        return overflowable();
    }
    fn operator*(const overflowable &rhs) const -> overflowable {
        if (inner.has_value() && rhs.inner.has_value()) {
            I res;
            return ckd_mul(&res, inner.value(), rhs.inner.value()) ? overflowable() : overflowable(res);
        }
        return overflowable();
    }
    fn operator*=(const I val) -> overflowable & {
        return self = self * val;
    }
    fn operator*=(const overflowable &rhs) -> overflowable & {
        return self = self * rhs;
    }
    fn operator<=>(const overflowable &rhs) const -> std::partial_ordering {
        if (self.overflowed() && rhs.overflowed()) {
            // overflowable() cannot compare with itself
            return std::partial_ordering::unordered;
        }
        if (self.overflowed()) {
            // this is overflowable(), greater than any I
            return std::partial_ordering::greater;
        }
        if (rhs.overflowed()) {
            // rhs is overflowable(), less than overflowable()
            return std::partial_ordering::less;
        }
        return std::partial_ordering(self.inner.value() <=> rhs.inner.value());
    }
};


// 模意义下的计算

template <std::unsigned_integral I, I M>
class mod_unsigned {
    static_assert(M > 0, "M must be positive");

public:
    I inner;

    mod_unsigned() : inner({}) { }
    mod_unsigned(const I val) : inner(val) {
        debug_assert(val < M);
    }

    fn operator==(const mod_unsigned &rhs) const -> bool {
        return self.inner == rhs.inner;
    }

    fn operator++() -> mod_unsigned & {
        if (++self.inner == M) {
            self.inner = 0;
        }
        return self;
    }
    fn operator++(int) -> mod_unsigned {
        let original = self;
        ++self;
        return original;
    }
    fn operator+(const mod_unsigned &rhs) const -> mod_unsigned {
        static_assert(std::bit_width(M) + 1 <= std::numeric_limits<I>::digits, "M may be too large for +");
        return mod_unsigned{(self.inner + rhs.inner) % M};
    }
    fn operator+=(const mod_unsigned &rhs) -> mod_unsigned & {
        return self = self + rhs;
    }

    fn operator--() -> mod_unsigned & {
        if (self.inner == 0) {
            self.inner = M - 1;
        } else {
            --self.inner;
        }
        return self;
    }
    fn operator--(int) -> mod_unsigned {
        let original = self;
        --self;
        return original;
    }
    fn operator-() const -> mod_unsigned {
        return self.inner == 0 ? self : mod_unsigned{M - self.inner};
    }
    fn operator-(const mod_unsigned &rhs) const -> mod_unsigned {
        return self + -rhs;
    }
    fn operator-=(const mod_unsigned &rhs) -> mod_unsigned & {
        return self += -rhs;
    }

    fn operator*(const mod_unsigned &rhs) const -> mod_unsigned {
        static_assert(std::bit_width(M) * 2 <= std::numeric_limits<I>::digits, "M may be too large for *");
        return mod_unsigned{(inner * rhs.inner) % M};
    }
    fn operator*=(const mod_unsigned &rhs) -> mod_unsigned & {
        return self = self * rhs;
    }

    fn multiplicative_inverse() const -> mod_unsigned {
        // FIXME: Assert M to be prime.
        return powi(self, M - I{2});
    }
    fn operator/=(const mod_unsigned &rhs) -> mod_unsigned & {
        return self *= rhs.multiplicative_inverse();
    }
    fn operator/(const mod_unsigned &rhs) const -> mod_unsigned {
        return self * rhs.multiplicative_inverse();
    }
};


// 组合数学

template <std::unsigned_integral I, typename R>
fn factorial(const I n) -> R {
    let view = std::views::iota(I{1}, n + I{1});
    return std::accumulate(view.begin(), view.end(), R{1}, [](let acc, let x) {
        return acc * R{x};
    });
}

template <std::unsigned_integral I, typename R>
fn perm(const I n, const I k) -> R {
    if (k > n) {
        return R{0};
    }
    let view = std::views::iota(n - k + I{1}, n + I{1});
    return std::accumulate(view.begin(), view.end(), R{1}, [](let acc, let x) {
        return acc * R{x};
    });
}

template <std::unsigned_integral I, typename R>
fn comb(const I n, const I k) -> R {
    return perm<I, R>(n, k) / factorial<I, R>(k);
}


// 最长公共子序列

// longest-common-subsequence problem
template <std::ranges::input_range R>
fn lcs(R &&a, R &&b) -> usize {
    usize cnt{0};
    todo();
    return cnt;
}


// 测试

fn test() {
    var a = mod_unsigned<u32, 2017>{5};
    a *= 20;
    assert(dbg(a.inner == 100));
    a *= 21;
    assert(dbg(dbg(a.inner) += 3) == 2103 % 2017);
    var b = overflowable<uint8_t>{255};
    assert((b += uint8_t{1}).overflowed());
    var c = overflowable<uint8_t>{255};
    assert((++c).overflowed());
    var d = overflowable<uint8_t>{255};
    assert(!(d++).overflowed());
    assert(powi(-3, u16{3}) == -27);
    var e = mod_unsigned<u32, 10>{0};
    assert((--e).inner == 9);
    assert((factorial<u32, mod_unsigned<u32, 105>>(5).inner) == 15);
}


// 任务

fn run() {
    using std::cin;
    using std::cout;

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

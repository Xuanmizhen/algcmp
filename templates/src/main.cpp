// C++20
// PTA: C++ (g++)

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


#define loop for (;;)

std::ostream &log_loc(std::ostream &os, const source_location loc = source_location::current()) {
    return os << '[' << loc.file_name() << ':' << loc.line() << ':' << loc.column() << "] `" << loc.function_name() << "`: ";
}

#define dbg(val) \
    ([](const auto v, const source_location loc) { \
        log_loc(std::clog, loc) << ' ' << #val << " = " << v << '\n'; \
        return v; \
    })(val, source_location::current())

#ifdef LOCAL
#define debug_assert(e) assert(e)
#else
#define debug_assert(e) void
#endif

namespace std {
namespace numbers {
template <class _Tp> inline constexpr _Tp tau_v = 2 * pi_v<_Tp>;
template <class _Tp> inline constexpr _Tp inv_tau_v = inv_pi_v<_Tp> / static_cast<_Tp>(2.0);
inline constexpr double tau = tau_v<double>;
inline constexpr double inv_tau = inv_tau_v<double>;
}
}

class disjoint_set {
    std::vector<isize> parent_or_neg_rank;
    uint8_t link(usize x, usize y) {
        const auto cmp{parent_or_neg_rank[x] <=> parent_or_neg_rank[y]};
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
    usize find_set(usize x) {
        auto root = x;
        while (parent_or_neg_rank[root] >= 0) {
            root = parent_or_neg_rank[root];
        }
        while (x != root) {
            const auto next = parent_or_neg_rank[x];
            parent_or_neg_rank[x] = root;
            x = next;
        }
        return root;
    }
    struct link_res {
        usize union_repr;
        std::optional<usize> deleted_repr;
    };
    link_res unite(usize x, usize y) {
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

template <std::three_way_comparable weight>
class edge {
public:
    usize u, v;
    weight cost;

    std::weak_ordering operator<=>(const edge& rhs) const {
        return rhs.cost <=> cost;
    }
};

template <std::three_way_comparable weight>
std::optional<edge<weight>> kruskal_safe_edge(disjoint_set &components, std::priority_queue<edge<weight>> &q) {
    while (!q.empty()) {
        const auto e = q.top();
        q.pop();
        if (components.unite(e.u, e.v).deleted_repr.has_value()) {
            return e;
        }
    }
    return {};
}


void run() {
    u32 t;
    cin >> t;
    while (t-- > 0) {
        // Process each test case
    }
}

int main() {
    using namespace std;
    ios_base::sync_with_stdio(false);
    cin.tie(nullptr);

#ifdef LOCAL
    ifstream fin("in.txt");
    ofstream fout("out.txt");
    if (!fin.is_open() || !fout.is_open()) {
        std::cerr << "files not opened\n";
        return 1;
    }
    cin.rdbuf(fin.rdbuf());
    cout.rdbuf(fout.rdbuf());
#endif

    run();
}

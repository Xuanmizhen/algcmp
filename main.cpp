// C++20
// PTA: C++ (g++)

#include <compare> // std::strong_ordering
#include <cstdint>
#include <fstream>
#include <iostream>
#include <type_traits> // std::make_signed_t
#include <vector>

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
using std::cerr;


#define dbg(val) \
    ([](const auto v) { \
        cerr << "[" __FILE__ ":" << __LINE__ << "] " #val " = " << v << '\n'; \
        return v; \
    })(val)

class disjoint_set {
    std::vector<isize> parent_or_neg_rank;

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
    bool unite(usize x, usize y) {
        if ((x = find_set(x)) == (y = find_set(y))) {
            return true;
        }
        const auto cmp{parent_or_neg_rank[x] <=> parent_or_neg_rank[y]};
        if (cmp < 0) {
            parent_or_neg_rank[y] = x;
        } else {
            if (cmp == 0) {
                --parent_or_neg_rank[y];
            }
            parent_or_neg_rank[x] = y;
        }
        return false;
    }
};


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
        cerr << "files not opened\n";
        return 1;
    }
    cin.rdbuf(fin.rdbuf());
    cout.rdbuf(fout.rdbuf());
#endif

    run();
}

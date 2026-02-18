#include <cstdint>
#include <fstream>
#include <iostream>
#include <type_traits> // std::make_signed_t

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

void run() {
    u32 t;
    cin >> t;
    while (t--) {
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

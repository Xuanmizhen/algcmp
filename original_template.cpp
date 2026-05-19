// clang++ -std=c++20 -DLOCAL ./template.cpp

#include <iostream>
#include <vector>

#ifdef LOCAL

#include <chrono>
#include <iomanip>

class _timer
{
    std::chrono::time_point<std::chrono::steady_clock> start;

public:
    _timer();
    void report() const;
    ~_timer();
};

#endif

template<typename T>
void cin_vector(std::vector<T> &vec, const size_t n);

int main()
{
    using namespace std;

    #ifndef LOCAL
    ios_base::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    #else
    cout << "Running locally" << endl
         << "I/O acceleration not enabled" << endl
         << "Timer enabled" << endl
         << endl;
    _timer _t;
    #endif

    cout << "Hello, world!" << endl << flush;
}

template<typename T>
void cin_vector(std::vector<T> &vec, const size_t n) {
    vec.clear();
    for (size_t i {0}; i < n; ++i) {
        T val;
        std::cin >> val;
        vec.push_back(val);
    }
}

#ifdef LOCAL

_timer::_timer()
    : start(std::chrono::steady_clock::now())
{}

void _timer::report() const
{
    auto end = std::chrono::steady_clock::now();
    auto elapsed = std::chrono::duration_cast<std::chrono::nanoseconds>(end - start).count();

    auto old_fill = std::cout.fill('0');
    auto old_width = std::cout.width(0);
    std::cout << std::endl
              << "Running for " << (elapsed / 1'000'000) << '.' << std::setw(6) << (elapsed % 1'000'000) << " milliseconds." << std::endl
              << std::flush << std::setw(old_width) << std::setfill(old_fill);
}

_timer::~_timer()
{
    report();
}

#endif

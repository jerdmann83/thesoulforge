#include <atomic>
#include <chrono>
#include <functional>
#include <iostream>
#include <thread>
#include <vector>
#include <cassert>
#include <mutex>
#include <stdio.h>
#include <unistd.h>
using namespace std;
using namespace std::chrono;

class spinlock {
    atomic_bool lock_ { false };

public:
    void lock()   { while (lock_.exchange(true)); }
    void unlock() { lock_ = false; }
};
using lock_t = lock_guard<spinlock>;

template <typename T>
class freelist {
    struct node {
        T data;
        node* next;
    };

    atomic<node*> head_;
    spinlock lock_;

public:
    freelist() : head_(nullptr) {}
    void push_v1(T d) {
        lock_t lock(lock_);

        node* n = new node{d, nullptr};
        node* prev = head_.load();
        n->next = prev;
        head_ = n;
    }

    void push_v2(T d) {
        node* n = new node{d, nullptr};
        n->next = head_.load();
        while (!head_.compare_exchange_weak(n->next, n));
    }

    size_t size() {
        lock_t lock(lock_);
        node* n = head_.load();
        size_t num = 0;
        while (n) {
            ++num;
            n = n->next;
        }
        return num;
    }
};

void timeit(std::function<void()> fn, const char* tag) {
    using cl = high_resolution_clock;
    auto start = cl::now();
    fn();
    auto d = duration_cast<microseconds>(cl::now() - start);
    cout << tag << ": " << d.count() << endl;
}

int main()
{
    size_t num_threads = thread::hardware_concurrency();
    size_t pushes = 10000;

    timeit([=]() {
        freelist<int> f;
        vector<thread> threads;
        for (int i=0; i<num_threads; ++i) {
            threads.push_back(std::thread([&f, i, pushes]() {
                for (int j=0; j<pushes; ++j) f.push_v1(i);
            }));
        }
        for (auto& t : threads) t.join();
        assert(f.size() == num_threads * pushes);
    }, "v1");

    timeit([=]() {
        freelist<int> f;
        vector<thread> threads;
        for (int i=0; i<num_threads; ++i) {
            threads.push_back(std::thread([&f, i, pushes]() {
                for (int j=0; j<pushes; ++j) f.push_v2(i);
            }));
        }
        for (auto& t : threads) t.join();
        assert(f.size() == num_threads * pushes);
    }, "v2");
}

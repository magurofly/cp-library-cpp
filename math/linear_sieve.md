# 線形篩

エラトステネスの篩の線形版。

```c++
struct LinearSieve {
public:
  vector<int> table, primes;
  // n までの篩を構築する
  // 計算量: O(n)
  LinearSieve(int n) : table(n) {
    table[0] = table[1] = 1;
    for (int d = 2; d <= n; d++) {
      if (!table[d]) table[d] = d, primes.push_back(d);
      for (int p : primes) {
        if (p * d > n || p > table[d]) break;
        table[p * d] = p;
      }
    }
  }
  
  // n の最小素因数を返す
  // n が素数なら lpf(n) = n
  // 計算量: O(1)
  int lpf(int n) {
    return table[n];
  }
  
  // n を素因数分解する
  // 計算量: O(log n)
  map<int, int> prime_division(int n) {
    map<int, int> m;
    while (n != 1) {
      int p = lpf(n);
      n /= p;
      m[p]++;
    }
    return m;
  }
}
```

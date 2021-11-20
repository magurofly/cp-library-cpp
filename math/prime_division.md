# 素因数分解

$a = \Pi_{p} p^{e_p}$ であるような $a$ に対して、 $m_p = e_p$ となる `map` を返す。

計算量: $O(\sqrt a)$

```c++
using ll = long long;
map<ll, int> prime_division(ll a) {
  map<ll, int> m;
  for (ll p = 2; p * p <= a; p++) while (a % p == 0) a /= p, m[p]++;
  if (a > 1) m[a]++;
  return m;
}
```

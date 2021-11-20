# $a^{-1} \bmod m$

逆元を求める。 $m$ は素数でなくてもよいが、 $a \perp m$ である必要がある。

C++11を想定。

計算量: $O(\log m)$

```c++
using ll = long long;
ll mod_inv(ll a, ll m) {
  a = (a % m + m) % m;
  assert(a != 0);
  ll s = m, t = a;
  ll x = 0, y = 1;
  while (t) {
    ll u = s / t;
    s -= t * u;
    x -= y * u;
    swap(s, t);
    swap(x, y);
  }
  return (x < 0) ? x + m / s : x;
}
```

## 参考

- https://github.com/atcoder/ac-library/blob/master/atcoder/internal_math.hpp

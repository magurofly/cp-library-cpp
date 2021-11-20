# 累乗 $\bmod m$

```c++
using ll = long long;
ll mod_pow(ll a, ll e, ll m) {
  ll r = 1;
  while (e != 0) {
    if (e & 1) r = r * a % m;
    a = a * a % m;
    e >>= 1;
  }
  return (r < 0) ? r + m : r;
}
```

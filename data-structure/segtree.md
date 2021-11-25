# セグメント木

```c++
template<class S, S (*op)(S, S), S (*e)()>
struct segtree {

private:

  int n, m;
  vector<S> d;
  
  void update(int i) { d[i] = op(d[i * 2], d[i * 2 + 1]); }
  
public:

  // 構築 O(n)
  segtree(int n) : segtree(vector<S>(n, e())) {}
  segtree(vector<S>& v) : n(int(v.size())), m(1) {
    while (m < n) m *= 2;
    d = vector<S>(m + n, e());
    for (int i = n; i--; ) d[m + i] = v[i];
    for (int i = m; --i; ) update(i);
  }
  
  // 値の代入 O(log n)
  void set(int p, S x) {
    assert(0 <= p && p < n);
    d[p += m] = x;
    while (p /= 2) update(p);
  }
  
  // 値の取得 O(1)
  S get(int p) {
    assert(0 <= p && p < n);
    return d[p + m];
  }
  
  // 区間の取得 O(log n)
  S prod(int l, int r) {
    assert(0 <= l && l <= r && r <= n);
    l += m; r += m;
    S x = e(), y = e();
    while (l < r) {
      if (l & 1) x = op(x, d[l++]);
      if (r & 1) y = op(d[--r], y);
      l /= 2;
      r /= 2;
    }
    return op(x, y);
  }
  
  // 全区間の取得 O(1)
  S all_prod() {
    return d[1];
  }
};
```

## 参考
- https://github.com/atcoder/ac-library/blob/master/atcoder/segtree.hpp

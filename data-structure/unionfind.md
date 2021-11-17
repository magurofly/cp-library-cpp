# UnionFind

無向森を扱うデータ構造。

`unite` と `find` は必須、 `size` と `groups` は省略可能。

[Verify](https://atcoder.jp/contests/practice2/submissions/27309118)

```c++
struct UnionFind {
private:
  vector<int> p;
  int n;

public:
  // n 頂点で初期化する
  UnionFind(int n) : n(n), p(n, -1) {}
  
  // v の属する木の根を求める
  // 返り値: 根の頂点番号
  // 計算量: O(α(n))
  int find(int v) {
    assert(0 <= v && v <= n);
    return (p[v] < 0) ? v : (p[v] = find(p[v]));
  }
  
  // u と v を併合する
  // 返り値: u と v が異なる木に属していたか
  // 計算量: O(α(n)) amortized
  bool unite(int u, int v) {
    assert(0 <= u && u <= n);
    assert(0 <= v && v <= n);
    u = find(u);
    v = find(v);
    if (u == v) return false;
    if (p[u] > p[v]) swap(u, v);
    p[u] += p[v];
    p[v] = u;
    return true;
  }
  
  // v の属する木の頂点数を求める
  // 返り値: 頂点数
  int size(int v) {
    assert(0 <= v && v <= n);
    return -p[find(v)];
  }
  
  // すべての木を求める
  // 返り値: 木の頂点リストのリスト
  // 計算量: O(n)
  vector<vector<int>> groups() {
    vector<vector<int>> g(p.size());
    for (int v = p.size(); v--; ) g[find(v)].push_back(v);
    g.erase(remove_if(g.begin(), g.end(), [](vector<int>& v) { return v.empty(); }), g.end());
    return g;
  }
};
```

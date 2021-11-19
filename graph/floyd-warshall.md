# Floyd-Warshall

隣接行列で表されたグラフの全点対間最短経路を $O(N^3)$ で求める。

負閉路が存在する場合、負閉路上の頂点 $v$ は負の自己ループを持つ。

```c++
template<typename T>
vector<vector<T>> floyd_warshall(vector<vector<T>> g) {
  const T INF = numeric_limits<T>::max();
  for (int k = g.size(); k--; ) g[k][k] = 0;
  for (int k = g.size(); k--; ) for (int i = g.size(); i--; ) if (g[i][k] < INF) for (int j = g.size(); j--; ) if (g[k][j] < INF) {
    int d = g[i][k] + g[k][j];
    if (g[i][j] > d) g[i][j] = d;
  }
  return g;
}
```

## 参考

- [Floyd-Warshall のアルゴリズム – 37zigenのHP](https://37zigen.com/floyd-warshall-algorithm/)

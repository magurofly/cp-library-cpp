# SPFA (Shortest Path Faster Algorithm)

負辺を持ちうるグラフ上の最短経路を計算する。C++17を想定。

負閉路が含まれる場合、始点から到達できる負閉路上の距離は-INFとなる。

```c++
template<typename T>
vector<T> spfa(vector<vector<pair<int, T>>>& g, int s) {
  const int N = g.size();
  const T INF = numeric_limits<T>::max();
  assert(0 <= s && s < N);
  vector<T> dist(N, INF);
  queue<int> que;
  vector<int> inque(N), count(N);
  dist[s] = 0;
  que.emplace(s); inque[s] = true; count[s]++;
  while (!que.empty()) {
    int u = que.front(); que.pop(); inque[u] = false;
    for (auto [v, c] : g[u]) {
      T d = (dist[u] <= -INF || count[v] >= N) ? -INF : dist[u] + c;
      if (dist[v] <= d) continue;
      dist[v] = d;
      if (!inque[v]) {
        que.emplace(v); inque[v] = true;
        if (++count[v] >= N * 2) return dist;
      }
    }
  }
  return dist;
}
```

## Verify

- 負辺なし
  - [https://judge.u-aizu.ac.jp/onlinejudge/review.jsp?rid=6061027#1](https://judge.u-aizu.ac.jp/onlinejudge/review.jsp?rid=6061027#1)
- 負辺・負閉路あり（始点から到達できる負閉路）
  - [https://judge.u-aizu.ac.jp/onlinejudge/review.jsp?rid=6061043#1](https://judge.u-aizu.ac.jp/onlinejudge/review.jsp?rid=6061043#1)

## 参考
- [単一始点最短路(SPFA) \| Luzhiled’s memo](https://ei1333.github.io/luzhiled/snippets/graph/shortest-path-faster-algorithm.html)
- [単一始点最短経路 (SPFA) - yaketake08's 実装メモ](https://tjkendev.github.io/procon-library/python/graph/spfa.html)
- [ABC137-E:Coins Respawn \~負閉路検出について\~ - 思考の墓場](https://sigma1113.hatenablog.com/entry/2019/08/12/130042)

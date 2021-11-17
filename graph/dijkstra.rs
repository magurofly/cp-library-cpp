# ダイクストラ法（隣接リスト）

隣接リスト形式のグラフと始点を受け取り、最短経路長のリストを返す。

```c++
template<typename T>
vector<T> dijkstra(vector<vector<pair<int, T>>>& g, int s) {
  vector<T> dist(g.size(), numeric_limits<T>::max());
  dist[s] = 0;
  using P = pair<T, int>;
  priority_queue<P, vector<P>, greater<P>> pq;
  pq.emplace(0, s);
  while (!pq.empty()) {
    T d;
    int u;
    tie(d, u) = pq.top(); pq.pop();
    if (d > dist[u]) continue;
    for (auto& e : g[u]) {
      T d2 = d + e.second;
      if (dist[e.first] <= d2) continue;
      pq.emplace(dist[e.first] = d2, e.first);
    }
  }
  return dist;
}
```

## 参考
- https://ei1333.github.io/luzhiled/snippets/graph/dijkstra.html

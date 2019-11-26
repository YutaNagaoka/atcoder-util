#include <algorithm>
#include <cmath>
#include <iostream>
#include <string>
#include <vector>
using namespace std;


struct UnionFind {
    vector<int> _parent;
    vector<int> _size;

    explicit UnionFind(int n) : _parent(n), _size(n, 1LL) {
        for (int i = 0; i < n; i++)
            _parent[i] = i;
    }

    int root(int x) {
        if (_parent[x] == x)
            return x;
        return _parent[x] = root(_parent[x]);
    }

    bool same(int x, int y) {
        return root(x) == root(y);
    }

    void unite(int x, int y) {
        int rx = root(x);
        int ry = root(y);
        if (rx == ry)
            return;
        if (size(rx) < size(ry))
            swap(rx, ry);
        _parent[ry] = rx;
        _size[rx] += _size[ry];
    }

    int size(int x) {
        return _size[root(x)];
    }
};

signed main() {
    int n, m;
    cin >> n >> m;
    vector<pair<int, int>> v;
    for (int i = 0; i < m; i++) {
        int a, b;
        cin >> a >> b;
        v.emplace_back(a - 1, b - 1);
    }
    reverse(v.begin(), v.end());
    UnionFind tree(n);
    vector<int> ans(m);
    ans[0] = n * (n - 1) / 2;
    for (int i = 1; i < m; i++) {
        int x = v[i - 1].first;
        int y = v[i - 1].second;
        if (tree.same(x, y)) {
            ans[i] = ans[i - 1];
        }
        else {
            ans[i] = ans[i - 1] - tree.size(x) * tree.size(y);
        }
        tree.unite(x, y);
    }
    reverse(ans.begin(), ans.end());
    for_each(ans.begin(), ans.end(), [](int x) {
        cout << x << "\n";
    });
}
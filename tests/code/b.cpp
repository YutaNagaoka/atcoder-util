#include <algorithm>
#include <cmath>
#include <iostream>
#include <string>
#include <vector>
using namespace std;

int main(){
    int a, b, k;
    cin >> a >> b >> k;
    vector<int> v;
    int limit = max(a, b);

    for(int i = 1; i <= limit; i++){
        if (a % i == 0 && b % i == 0) {
            v.push_back(i);
        }
    }

    cout << v[v.size() - k] << "\n";
}
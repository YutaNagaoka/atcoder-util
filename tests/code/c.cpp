#include <algorithm>
#include <cmath>
#include <iostream>
#include <stack>
#include <string>
#include <vector>
using namespace std;

signed main() {
    string s;
    cin >> s;
    stack<char> st;
    int sum = 0;
    st.push(s[0]);
    for(int i = 1; i < s.size(); ++i) {
        if (st.empty() || st.top() == s[i]) {
            st.push(s[i]);
        }
        else {
            st.pop();
            sum += 2;
        }
    }
    cout << sum << "\n";
}
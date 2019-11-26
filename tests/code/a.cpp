#include <algorithm>
#include <iostream>
#include <string>
#include <vector>
using namespace std;

int main(){
    int a, b, c;
    cin >> a >> b >> c;
    int d = b / a;
    if (c > d) {
        cout << d;
    }
    else{
        cout << c;
    }
    cout << "\n";
}
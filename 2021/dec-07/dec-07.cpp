#include <iostream>
#include <fstream>
#include <vector>
#include <string>
#include <cmath>

using std::cout;
using std::endl;
using std::string;
using std::vector;

struct MinMax {
    long min = __LONG_LONG_MAX__;
    long max = -1;
};

void loadInput(string filename, vector<long> &input) {
    std::ifstream in(filename);
    for(string str; std::getline(in, str, ',');) {
        input.push_back(std::stol(str));
    }
}

MinMax findMinMax(vector<long> &input) {
    struct MinMax minMax;
    for(int i = 0; i < input.size(); i++) {
        long t = input[i];
        if(t < minMax.min) { minMax.min = t; }
        if(t > minMax.max) { minMax.max = t; }
    }

    return minMax;
}

vector<long> p1Lowest(vector<long> &input, MinMax &minMax, bool shouldPrint) {
    vector<long> costs;
    for(int i = minMax.min; i < minMax.max; i++) {
        int total = 0;
        for(int k = 0; k < input.size(); k++) {
            int t = std::abs(input[k] - i);
            total += t;
            if(shouldPrint) cout<<input[k]<<" -> "<<k<<" -> "<<i<<" == "<<t<<endl;
        }

        costs.push_back(total);
    }

    return costs;
}

long getSum(long &t) {
    long output = 0;
    for(int i = 1; i <= t; i++) {
        output += i;
    }

    return output;
}

vector<long> p2Lowest(vector<long> &input, MinMax &minMax, bool shouldPrint) {
    vector<long> costs;
    for(int i = minMax.min; i < minMax.max; i++) {
        long total = 0;
        for(int k = 0; k < input.size(); k++) {
            long t = std::abs(input[k] - i);
            long sum = getSum(t);
            total += sum;
            if(shouldPrint) cout<<input[k]<<" -> "<<k<<" -> "<<i<<" == "<<t<<" sum "<<sum<<endl;
        }

        costs.push_back(total);
    }

    return costs;
}

long findLowest(vector<long> &numbers) {
    long lowest = __LONG_LONG_MAX__;
    for(int i = 0; i < numbers.size(); i++) {
        if(lowest > numbers[i]) { lowest = numbers[i]; }
    }

    return lowest;
}

int main() {
    vector<long> input;
    loadInput("input.txt", input);
    
    struct MinMax minMax = findMinMax(input);
    cout<<"min "<<minMax.min<<" max "<<minMax.max<<endl;
    vector<long> costs = p1Lowest(input, minMax, false);
    cout<<"p1 lowest "<<findLowest(costs)<<endl;
    costs = p2Lowest(input, minMax, false);
    cout<<"p2 lowest "<<findLowest(costs)<<endl;
}
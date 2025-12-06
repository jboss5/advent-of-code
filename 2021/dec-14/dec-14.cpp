#include <iostream>
#include <string>
#include <vector>
#include <fstream>
#include <map>
#include <queue>
#include <climits>
#include <thread>
#include <future>
#include <unordered_map>

using std::cout;
using std::endl;
using std::string;
using std::vector;
using std::map;
using std::queue;
using std::thread;
using std::unordered_map;
using std::future;
using std::uint64_t;

struct Polymer {
    string t;
    //vector<string> pairInsertions;
    unordered_map<string, char> pairInsertions;
};

Polymer readInput(const string &filename) {
    std::ifstream in(filename);
    int count = 0;
    Polymer polymer;
    for(string line; getline(in, line);) {
        if(count == 0) polymer.t = line;
        else if(count > 1) {
            string input = line.substr(0, line.find(" -> "));
            polymer.pairInsertions[input] = line.at(line.length()-1);
        }

        count++;
    }

    return polymer;
}

void part1Take2(const string& filename, const int &STEPS) {
    struct Polymer polymer = readInput(filename);

    cout<<"Template: "<<polymer.t<<endl;
    string output = polymer.t;
    unordered_map<string,uint64_t> prevMap, outputMap;
    for(int k = 1; k < output.size(); k++) {
        prevMap[string() + output[k-1] + output[k]] = 1;
    }

    for(int i = 0; i < STEPS; i++) {
        unordered_map<string,uint64_t> currMap;
        for(std::unordered_map<string,uint64_t>::iterator it = prevMap.begin(); it != prevMap.end(); it++) {
            char insertChar = polymer.pairInsertions[it->first];
            string firstPair = string() + it->first.at(0) + insertChar;
            string secondPair = string() + insertChar + it->first.at(1);
            currMap[firstPair] += it->second;
            currMap[secondPair] += it->second;
        }

        prevMap.clear();
        prevMap = currMap;
    }

    map<char, uint64_t> charTotals;
    for(std::unordered_map<string,uint64_t>::iterator it = prevMap.begin(); it != prevMap.end(); it++) {
        cout<<"("<<it->first<<","<<it->second<<")"<<endl;

        charTotals[it->first.at(0)] += it->second;
    }

    uint64_t maxNum = 0, minNum = ULLONG_MAX;
    for(std::map<char,uint64_t>::iterator it = charTotals.begin(); it != charTotals.end(); it++) {
        if(it->second > maxNum) maxNum = it->second;
        if(it->second < minNum) minNum = it->second;   
    }

    cout<<"Max num: "<<maxNum<<" , minNum: "<<minNum<<endl;
    cout<<"Total: "<<(maxNum - minNum)+1<<endl;
}

int main() {
    cout<<"Part 1"<<endl;
    part1Take2("input.txt", 40);

    return 0;
}
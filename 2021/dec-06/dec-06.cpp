#include <iostream>
#include <string>
#include <sstream>
#include <fstream>
#include <vector>
#include <cmath>
#include <map>

using std::cout;
using std::endl;
using std::string;
using std::vector;
using std::map;

vector<char> getInput(string filename) {
    std::ifstream in(filename);
    vector<char> initialStates;
    string state;
    for(string state; std::getline(in, state, ',');) {
        initialStates.push_back(state.at(0));
    }

    return initialStates;
}

void print(vector<char> input) {
    for(int i = 0; i < input.size(); i++) {
        cout<<input[i]<<',';
    }

    cout<<endl;
}

unsigned long long p1(char a, const int TRIALS, bool shouldPrint) {
    vector<char> test;
    test.push_back(a);
    for(int i = 0; i < TRIALS; i++) {
        unsigned long long currentSize = test.size();
        cout<<i<<" ";
        if(shouldPrint) print(test);
        for(unsigned long long k = 0; k < currentSize; k++) {
            if(test[k] == '0') {
                test[k] = '6';
                test.push_back('8');
            } else { test[k]--; }
        }
    }

    return test.size();
}

map<char,int> uniqueInput(vector<char> &input) {
    map<char, int> outputCount;
    vector<char> outputVector;
    for(int i = 0; i < input.size(); i++) {
        cout<<input[i]<<" ";
        map<char,int>::iterator it = outputCount.find(input[i]);
        if(it != outputCount.end()) {
            it->second += 1;
        } else {
            outputCount[input[i]] = 1;
            outputVector.push_back(input[i]);
        }
    }

    input.swap(outputVector);
    return outputCount;
}

int main() {
    const int TRIALS = 256;
    vector<char> input = getInput("input.txt");
    map<char,int> uniqueCount = uniqueInput(input);

    for(map<char,int>::iterator it = uniqueCount.begin(); it != uniqueCount.end(); it++) {
        cout<<it->first<<" => "<<it->second<<endl;
    }

    cout<<endl<<"input:"<<endl;
    for(int i = 0; i < input.size(); i++) {
        cout<<i<<" => "<<input[i]<<endl;
    }

    unsigned long long total = 0;
    for(map<char,int>::iterator it = uniqueCount.begin(); it != uniqueCount.end(); it++) {
        cout<<"("<<it->first<<","<<it->second<<")"<<endl<<"RUN #";
        unsigned long long t = p1(it->first, TRIALS, false);
        if(it->second > 1) { t *= it->second; }
        total += t;
    }

    cout<<"Part 1:\nTotal fish after: "<<TRIALS<<" trials => "<<total<<endl;


    return 0;
}
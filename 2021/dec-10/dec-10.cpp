#include <iostream>
#include <stack>
#include <string>
#include <vector>
#include <fstream>
#include <algorithm>
#include <functional>

using std::cout;
using std::string;
using std::endl;
using std::vector;
using std::stack;

void readInput(vector<string> &input, const string &filename) {
    std::ifstream in(filename);
    for(string line; getline(in, line);) {
        input.push_back(line);
    }
}

void print(vector<string> &input) {
    for(int i = 0; i < input.size(); i++) {
        cout<<input[i]<<endl;
    }
}

bool isClosingChar(char &ch) {
    return (ch == ')' || ch == '>' || ch == '}' || ch == ']');
}

bool isClosingMatch(char &close, char &open) {
    if(open == '(') return close == ')';
    if(open == '{') return close == '}';
    if(open == '[') return close == ']';
    if(open == '<') return close == '>';

    return false;
}

int getSize(char &ch, bool isP1) {
    if(ch == ')') return (isP1) ? 3 : 1;
    if(ch == ']') return (isP1) ? 57 : 2;
    if(ch == '}') return (isP1) ? 1197 : 3;
    if(ch == '>') return (isP1) ? 25137 : 4;

    return -1;
}

void part1(vector<string> &input) {
    stack<char> s;
    vector<char> invalidChars;
    for(int i = 0; i < input.size(); i++) {
        for(int k = 0; k < input[i].size(); k++) {
            char ch = input[i].at(k);
            if(isClosingChar(ch)) {
                if(isClosingMatch(ch, s.top())) s.pop();
                else {
                    invalidChars.push_back(ch);
                    break;
                }
            } else s.push(ch);
        }
    }

    long total = 0;
    for(int i = 0; i < invalidChars.size(); i++) {
        total += getSize(invalidChars[i], true);
    }

    cout<<"Total error score: "<<total<<endl;
}

char getCloseChar(char &ch) {
    if(ch == '(') return ')';
    if(ch == '{') return '}';
    if(ch == '[') return ']';
    if(ch == '<') return '>';

    cout<<"Found unmatched open: "<<ch<<endl;
    return ' ';
}

void part2(vector<string> &input) {
    stack<char> s;
    vector<string> fixedLines;
    for(int i = 0; i < input.size(); i++) {
        bool isCorrupted = false;
        for(int k = 0; k < input[i].size(); k++) {
            char ch = input[i].at(k);
            if(isClosingChar(ch)) {
                if(isClosingMatch(ch, s.top())) s.pop();
                else isCorrupted = true;
            }
            else s.push(ch);
        }

        if(!s.empty() && !isCorrupted) {
            string fixedLine;
            while(!s.empty()) {
                if(isClosingChar(s.top())) s.pop();
                else {
                    fixedLine.push_back(getCloseChar(s.top()));
                    s.pop();
                }
            }

            fixedLines.push_back(fixedLine);
        } else {
            // empty remaining stack
            while(!s.empty()) { s.pop(); }
        }
    }

    //print(fixedLines);
    vector<long> scores;
    for(int i = 0; i < fixedLines.size(); i++) {
        long total = 0;
        for(int k = 0; k < fixedLines[i].size(); k++) {
            total *= 5;
            total += getSize(fixedLines[i].at(k), false);
            //cout<<total<<endl;
        }
        
        //cout<<"line: "<<fixedLines[i]<<" total: "<<total<<endl;
        scores.push_back(total);
    }

    std::sort(scores.begin(), scores.end());
    cout<<"Middle score: "<<scores.at(scores.size() / 2)<<endl;
}

int main() {
    vector<string> input;
    readInput(input, "input.txt");
    //print(input);

    cout<<"Part1:"<<endl;
    part1(input);

    cout<<endl<<"Part2:"<<endl;
    part2(input);
    return 0;
}
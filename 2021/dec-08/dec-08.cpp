#include <iostream>
#include <fstream>
#include <vector>
#include <sstream>
#include <algorithm>

using std::string;
using std::cout;
using std::endl;
using std::vector;

vector<string> getInput(string filename) {
    std::ifstream in(filename);
    vector<string> lines;
    for(string line; std::getline(in, line, '\n');) {
        lines.push_back(line);
    }

    return lines;
}

int getCount(string &line) {
    int output = 0;
    std::stringstream stream(line);
    string input; 
    string signalLine;
    getline(stream, input, '|');
    getline(stream, signalLine, '|');
    std::stringstream sigStream(signalLine);
    //cout<<"input "<<input<<endl;
    //cout<<"signalLine "<<signalLine<<endl;

    for(string signal; getline(sigStream, signal, ' ');) {
        //cout<<"signal "<< signal<<endl;
        switch(signal.size()) {
            case 2:
            case 4:
            case 3:
            case 7:
                //cout<<signal<<endl;
                output++;
                break;
        }
    }

    return output;
}

void storeChars(vector<vector<char>> &digitCharacters, int index, string &str) {
    vector<char> temp;
    for(int i = 0; i < str.size(); i++) {
        temp.push_back(str.at(i));
    }

    digitCharacters[index] = temp;
}

bool containsAll(vector<char> &charList, string &str) {
    for(int i = 0; i < charList.size(); i++) {
        if(str.find(charList[i]) == std::string::npos) return false;
    }

    return true;
}

void print(vector<char> &list) {
    for(int i = 0; i < list.size(); i++) {
        cout<<"["<<list[i]<<",";
    }

    cout<<"]"<<endl;
}

int getSimilar(vector<char> &charList, string &str) {
    int outputCount = 0;
    for(int i = 0; i < str.size(); i++) {
        if(std::find(charList.begin(), charList.end(), str.at(i)) != charList.end())
            outputCount++;
    }

    return outputCount;
}

void getPart2(vector<string> &lines) {
    vector<vector<char>> digitCharacters(10);
    vector<int> outputValues;

    for(int i = 0; i < lines.size(); i++) {
        string outputString;
        string outputLine;
        string input;
        std::stringstream stream(lines[i]);
        getline(stream, input, '|');
        getline(stream, outputLine, '|');
        std::stringstream inputStream(input);
        std::stringstream sigStream(outputLine);

        for(string block; getline(inputStream, block, ' ');) {
            //cout<<"signal "<< signal<<endl;
            switch(block.size()) {
                case 2:
                    storeChars(digitCharacters, 1, block);
                    break;
                case 4:
                    storeChars(digitCharacters, 4, block);
                    break;
                case 3:
                    storeChars(digitCharacters, 7, block);
                    break;
                case 7:
                    storeChars(digitCharacters, 8, block);
                    break;
            }
        }

        //print(digitCharacters[1]);
        //print(digitCharacters[4]);
        //print(digitCharacters[7]);

        for(string signal; getline(sigStream, signal, ' ');) {
            //cout<<"signal "<< signal<<endl;
            switch(signal.size()) {
                case 2:
                    //cout<<"Adding 1"<<endl;
                    outputString.push_back('1');
                    break;
                case 4:
                    //cout<<"Adding 4"<<endl;
                    outputString.push_back('4');
                    break;
                case 3:
                    //cout<<"Adding 7"<<endl;
                    outputString.push_back('7');
                    break;
                case 5:
                    if(containsAll(digitCharacters[7], signal)) { outputString.push_back('3'); }
                    else if(getSimilar(digitCharacters[4], signal) == 3) { outputString.push_back('5'); }
                    else { outputString.push_back('2'); }
                    break;
                case 6:
                    if(containsAll(digitCharacters[4], signal)) { outputString.push_back('9'); }
                    else if(containsAll(digitCharacters[7], signal)) { outputString.push_back('0'); }
                    else { outputString.push_back('6'); }
                    break;
                case 7:
                    outputString.push_back('8');
                    break;
            }
        }

        outputValues.push_back(std::stoi(outputString));
    }

    long total = 0;
    for(int i = 0; i < outputValues.size(); i++) {
        total += outputValues[i];
    }

    cout<<"Part2 sum: "<<total<<endl;
}



int main() {
    vector<string> lines = getInput("input.txt");
    int count = 0;
    for(int i = 0; i < lines.size(); i++) {
        //cout<<lines[i]<<endl;
        count += getCount(lines[i]);
    }

    cout<<"Part1 count: "<<count<<endl;
    getPart2(lines);

    return 0;
}
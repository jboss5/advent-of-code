#include <iostream>
#include <vector>
#include <fstream>
#include <sstream>

using std::cout;
using std::vector;
using std::endl;
using std::string;

vector<string> readInput(string filename) {
    std::ifstream file(filename);
    string line;
    vector<string> output;

    while(getline(file, line)) {
        output.push_back(line);
    }   

    file.close();
    return output;
}

int resizeLineData(int squaredSize, int x1, int x2, int y1, int y2, vector<vector<int>> &lineData) {
    int start = squaredSize;
    if(x1 >= squaredSize) { squaredSize = x1+1; }
    if(x2 >= squaredSize) { squaredSize = x2+1; }
    if(y1 >= squaredSize) { squaredSize = y1+1; }
    if(y2 >= squaredSize) { squaredSize = y2+1; }

    if(start == squaredSize) {
        //cout<<"No resize needed"<<endl;
        return squaredSize;
    }

    lineData.resize(squaredSize);
    for(int i = 0; i < lineData.size(); i++) {
        lineData[i].resize(squaredSize);
    }

    //cout<<"Resized to "<<squaredSize<<endl;
    return squaredSize;
}

void print(vector<vector<int>> lineData) {
    for(int i = 0; i < lineData.size(); i++) {
        for(int k = 0; k < lineData[i].size(); k++) {
            cout<<lineData[i][k]<<" ";
        }

        cout<<endl;
    }

    cout<<endl;
}

vector<vector<int>> loadLines(vector<string> inputData, bool ignoreEquals) {
    vector<vector<int>> lineData;
    int squaredSize = 0;

    for(int i = 0; i < inputData.size(); i++) {
        std::stringstream stream(inputData[i]);
        std::size_t delim = inputData[i].find(" -> ");
        string startCoords = inputData[i].substr(0, delim);
        string endCoords = inputData[i].substr(delim+4);

        std::size_t commaDelim = startCoords.find(",");
        int x1 = std::stoi(startCoords.substr(0, commaDelim));
        int y1 = std::stoi(startCoords.substr(commaDelim+1));

        commaDelim = endCoords.find(',');
        int x2 = std::stoi(endCoords.substr(0, commaDelim));
        int y2 = std::stoi(endCoords.substr(commaDelim+1));

        // only process if x1 == x2 or y1 == y2 if ignoreEquals==true
        if(ignoreEquals && x1 != x2 && y1 != y2) {
            continue;
        }

        int startX, endX;
        squaredSize = resizeLineData(squaredSize, x1, x2, y1, y2, lineData);
        if(x1 <= x2) { startX = x1; endX = x2; }
        if(x2 < x1) { startX = x2; endX = x1; }

        int startY, endY;
        if(y1 <= y2) { startY = y1; endY = y2; }
        if(y2 < y1) { startY = y2; endY = y1; }

        //cout<<"x1 "<<x1<<" x2 "<<x2<<endl;
        //cout<<"y1 "<<y1<<" y2 "<<y2<<endl;
        //cout<<"startX "<<startX<<" endX "<<endX<<endl;
        //cout<<"startY "<<startY<<" endY "<<endY<<endl;
        
        if(startX == endX) {
            for(int j = startY; j <= endY; j++) {
                lineData[j][startX] = lineData[j][startX]+1;
            }
        } else if(startY == endY) {
            for(int j = startX; j <= endX; j++) {
                lineData[startY][j] = lineData[startY][j]+1;
            }
        } else {
            vector<int> xCoords, yCoords;

            // populate x & y diagonal values
            // note: xCoords and yCoords must match and both start at same point
            if(x1 > x2) { for(int j = x1; j >= x2; j--) { xCoords.push_back(j); } }
            else { for(int j = x1; j <= x2; j++) { xCoords.push_back(j); } }

            if(y1 > y2) { for(int j = y1; j >= y2; j--) { yCoords.push_back(j); } }
            else { for(int j = y1; j <= y2; j++) { yCoords.push_back(j); } }

            for(int j = 0; j < xCoords.size(); j++) {
                lineData[yCoords[j]][xCoords[j]] = lineData[yCoords[j]][xCoords[j]]+1;
            }
        }
    }

    return lineData;
}

int countPoints(vector<vector<int>> lineData) {
    int output = 0;

    for(int i = 0; i < lineData.size(); i++) {
        for(int k = 0; k < lineData[i].size(); k++) {
            if(lineData[i][k] > 1) { output++; }
        }
    }

    return output;
}

void part1(vector<string> input) {
    vector<vector<int>> lineData = loadLines(input, true);
    //print(lineData);

    cout<<"Output points: "<<countPoints(lineData)<<endl;
}

void part2(vector<string> input) {
    vector<vector<int>> lineData = loadLines(input, false);
    //print(lineData);

    cout<<"Output points: "<<countPoints(lineData)<<endl;
}

int main() {
    vector<string> input = readInput("input.txt");
    vector<string>::iterator it = input.begin();
    while(it != input.end()) {
        cout<<*it<<endl;
        it++;
    }
    cout<<endl;

    cout<<"Part 1:"<<endl;
    part1(input);
    cout<<endl<<"Part 2:"<<endl;
    part2(input);

    return 0;
}
#include <iostream>
#include <string>
#include <fstream>
#include <vector>

using std::string;
using std::vector;
using std::cout;
using std::endl;

void readInput(vector<vector<char>> &matrix, vector<string> &foldInstructions, const string &filename) {
    std::ifstream in(filename);
    vector<string> inputStrings;
    int maxY = 0, maxX = 0;
    for(string line; getline(in, line);) {
        if(!line.empty()) {
            if(line.find("fold") != std::string::npos) {
                foldInstructions.push_back(line);
            } else {
                inputStrings.push_back(line);
                int x = std::stoi(line.substr(0, line.find(',')));
                int y = std::stoi(line.substr(line.find(',')+1));

                if(maxY < y) { maxY = y; }
                if(maxX < x) { maxX = x; }
            }
        }
    }

    cout<<"y: "<<maxY<<" x: "<<maxX<<endl;
    matrix.resize(maxY+1);
    for(int i = 0; i < matrix.size(); i++) {
        matrix[i].resize(maxX+1, '.');
    }

    for(string input : inputStrings) {
        cout<<input<<endl;
        int x = std::stoi(input.substr(0, input.find(',')));
        int y = std::stoi(input.substr(input.find(',')+1));
        matrix[y][x] = '#';
    }
}

void print(vector<vector<char>> &matrix) {
    for(int i = 0; i < matrix.size(); i++) {
        for(int k = 0; k < matrix[i].size(); k++) {
            cout<<matrix[i][k];
        }

        cout<<endl;
    }
}

void foldY(int &value, vector<vector<char>> &matrix) {
    for(int i = 0; i < value; i++) {
        for(int k = 0; k < matrix[i].size(); k++) {
            //cout<<"i: "<<i<<" k: "<<k<<" offset: "<<(matrix.size()-1 - i)<<endl;
            if(matrix[i][k] == '.') { matrix[i][k] = matrix[matrix.size()-1 - i][k]; }
        }
    }

    matrix.resize(value);
}

void foldX(int &value, vector<vector<char>> &matrix) {
    for(int i = 0; i < matrix.size(); i++) {
        for(int k = 0; k < value; k++) {
            //cout<<"i: "<<i<<" k: "<<k<<" offset: "<<(matrix[i].size()-1 - k)<<endl;
            if(matrix[i][k] == '.') { matrix[i][k] = matrix[i][matrix[i].size()-1 - k]; }
        }
    }

    for(int i = 0; i < matrix.size(); i++) {
        matrix[i].resize(value);
    }
}

void printDots(vector<vector<char>> &matrix) {
    print(matrix);
    int numDots = 0;
    for(int i = 0; i < matrix.size(); i++) {
        for(int k = 0; k < matrix[i].size(); k++) {
            if(matrix[i][k] == '#') numDots++;
        }
    }
    
    cout<<"Number of dots: "<<numDots<<endl;
}

void part1(const string &filename) {
    vector<vector<char>> matrix;
    vector<string> foldInstructions;
    readInput(matrix, foldInstructions, filename);
    //print(matrix);

    cout<<endl<<"Folding instructions: "<<endl;
    for(string ins : foldInstructions) {
        cout<<ins<<endl;
        int value = std::stoi(ins.substr(ins.find('=')+1));

        if(ins.find('y') != std::string::npos) {
            foldY(value, matrix);
        } else {
            foldX(value, matrix);
        }

        printDots(matrix);
        cout<<endl;
    }
}

int main() {

    part1("input.txt");

    return 0;
}

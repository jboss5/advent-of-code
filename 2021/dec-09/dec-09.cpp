#include <iostream>
#include <string>
#include <vector>
#include <fstream>
#include <algorithm>
#include <functional>
#include <queue>
#include <set>

using std::cout;
using std::endl;
using std::string;
using std::vector;
using std::queue;
using std::set;

struct Coord {
    int v, h;

    Coord(int vVal, int hVal) : v(vVal), h(hVal) {}
};

bool operator==(const Coord &rhs, const Coord &lhs) {
    return (rhs.v == lhs.v && rhs.h == lhs.h);
}

void getInput(vector<string> &input, string filename) {
    std::ifstream in(filename);
    char ch;
    int vIndex = 0, hIndex = 0;
    for(string line; getline(in, line);) {
        input.push_back(line);
    }
}

void print(vector<string> &input) {
    for(int i = 0; i < input.size(); i++) {
        cout<<input[i]<<endl;
    }    
}

void print(Coord &c) {
    cout<<'('<<c.v<<','<<c.h<<')'<<endl;
}

int getCurrentPoint(char &ch) {
    return ch-'0';
}

bool isLowPoint(int &current, char &next) {
    return (current < getCurrentPoint(next));
}

bool isLeftCol(int &hIndex) {
    return (hIndex == 0);
}

bool isRightCol(int &hIndex, int rowSize) {
    return (hIndex == rowSize-1);
}

bool isTopRow(int &vIndex) {
    return (vIndex == 0); 
}

bool isBottomRow(int &vIndex, int colSize) {
    return (vIndex == colSize-1);
}

void getNeighbors(Coord &current, vector<string> &input, vector<Coord> &neighbors) {
    if(current.h > 0) neighbors.push_back(Coord{current.v, current.h - 1});
    if(current.v > 0) neighbors.push_back(Coord{current.v - 1, current.h});
    if(current.h < input[current.v].size()-1) neighbors.push_back(Coord{current.v, current.h+1});
    if(current.v < input.size()-1) neighbors.push_back(Coord{current.v+1, current.h});
}

bool alreadyVisited(vector<Coord> &reached, Coord &coord) {
    for(int i = 0; i < reached.size(); i++) {
        if(reached[i] == coord) return true;
    }

    return false;
}


void search(vector<string> &input, int &vIndex, int &hIndex, vector<int> &basinNumbers) {
    vector<Coord> reached;
    queue<Coord> q;
    q.push(Coord{vIndex, hIndex});
    int basins = 0;
    while(!q.empty()) {
        Coord c = q.front();
        print(c);
        q.pop();
        int current = getCurrentPoint(input[c.v].at(c.h));
        if(current == 9) continue;

        vector<Coord> neighbors;
        getNeighbors(c, input, neighbors);

        for(int i = 0; i < neighbors.size(); i++) {
            cout<<"Neighbor: ";
            print(neighbors[i]);
            int next = getCurrentPoint(input[neighbors[i].v].at(neighbors[i].h));
            if(next == 9) continue;

            if(!alreadyVisited(reached, neighbors[i])) {
                reached.push_back(neighbors[i]);
                q.push(neighbors[i]);
                basins++;
            }
        }
    }

    basinNumbers.push_back(basins);
}

void runit(vector<string> &input) {
    vector<int> lowPoints;
    vector<int> basinNumbers;
    for(int i = 0; i < input.size(); i++) {
        for(int k = 0; k < input[i].size(); k++) {
            int currentPoint = getCurrentPoint(input[i].at(k));

            if(isLeftCol(k)) {
                if(isTopRow(i)) {
                    if(isLowPoint(currentPoint, input[i].at(k+1)) &&
                        isLowPoint(currentPoint, input[i+1].at(k))) {
                            lowPoints.push_back(currentPoint);
                            search(input, i, k, basinNumbers);
                        }
                } else if(isBottomRow(i, input.size())) {
                    if(isLowPoint(currentPoint, input[i].at(k+1)) &&
                        isLowPoint(currentPoint, input[i-1].at(k))) {
                            lowPoints.push_back(currentPoint);
                            search(input, i, k, basinNumbers);
                        }
                } else {
                    if(isLowPoint(currentPoint, input[i].at(k+1)) &&
                        isLowPoint(currentPoint, input[i+1].at(k)) &&
                        isLowPoint(currentPoint, input[i-1].at(k))) {
                            lowPoints.push_back(currentPoint);
                            search(input, i, k, basinNumbers);
                        }
                }
            } else if(isRightCol(k, input[i].size())) {
                if(isTopRow(i)) {
                    if(isLowPoint(currentPoint, input[i].at(k-1)) &&
                        isLowPoint(currentPoint, input[i+1].at(k))) {
                            lowPoints.push_back(currentPoint);
                            search(input, i, k, basinNumbers);
                        }
                } else if(isBottomRow(i, input.size())) {
                    if(isLowPoint(currentPoint, input[i].at(k-1)) &&
                        isLowPoint(currentPoint, input[i-1].at(k))) {
                            lowPoints.push_back(currentPoint);
                            search(input, i, k, basinNumbers);
                        }
                } else {
                    if(isLowPoint(currentPoint, input[i].at(k-1)) &&
                        isLowPoint(currentPoint, input[i+1].at(k)) &&
                        isLowPoint(currentPoint, input[i-1].at(k))) {
                            lowPoints.push_back(currentPoint);
                            search(input, i, k, basinNumbers);
                        }
                }
            } else {
                if(isTopRow(i)) {
                    if(isLowPoint(currentPoint, input[i].at(k-1)) &&
                        isLowPoint(currentPoint, input[i].at(k+1)) &&
                        isLowPoint(currentPoint, input[i+1].at(k))) {
                            lowPoints.push_back(currentPoint);
                            search(input, i, k, basinNumbers);
                        }
                } else if(isBottomRow(i, input.size())) {
                    if(isLowPoint(currentPoint, input[i].at(k-1)) &&
                        isLowPoint(currentPoint, input[i].at(k+1)) &&
                        isLowPoint(currentPoint, input[i-1].at(k))) {
                            lowPoints.push_back(currentPoint);
                            search(input, i, k, basinNumbers);
                        }
                } else {
                    if(isLowPoint(currentPoint, input[i].at(k-1)) &&
                        isLowPoint(currentPoint, input[i].at(k+1)) &&
                        isLowPoint(currentPoint, input[i-1].at(k)) &&
                        isLowPoint(currentPoint, input[i+1].at(k))) {
                            lowPoints.push_back(currentPoint);
                            search(input, i, k, basinNumbers);
                        }
                }
            }
        }
    }

    long count = 0;
    for(int i = 0; i < lowPoints.size(); i++) {
        cout<<lowPoints[i]<<" || ";
        count += lowPoints[i]+1;
    }

    cout<<"Total Size: "<<count<<endl;

    for(int i = 0; i < basinNumbers.size(); i++) {
        cout<<basinNumbers[i]<<" ";
    }

    std::sort(basinNumbers.begin(), basinNumbers.end(), std::greater<int>());
    int output = basinNumbers[0] * basinNumbers[1] * basinNumbers[2];
    cout<<"basin calc: "<<output<<endl;
    
}

int main() {
    vector<string> input;
    getInput(input, "input.txt");
    print(input);
    
    runit(input);
    return 0;
}
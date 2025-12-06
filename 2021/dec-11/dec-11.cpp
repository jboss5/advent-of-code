#include <iostream>
#include <vector>
#include <fstream>
#include <queue>

using std::cout;
using std::endl;
using std::string;
using std::vector;
using std::queue;

struct Coord {
    int x,y;
    Coord(int xVal, int yVal) : x(xVal), y(yVal) {}
};

bool operator==(const Coord &rhs, const Coord &lhs) {
    return (rhs.x == lhs.x && rhs.y == lhs.y);
}

void readInput(vector<vector<int>> &input, const string &filename) {
    std::ifstream in(filename);
    for(string line; getline(in, line);) {
        vector<int> internal;
        for(int i = 0; i < line.size(); i++) {
            internal.push_back(line.at(i)-'0');
        }

        input.push_back(internal);
    }
}

void print(vector<vector<int>> &input) {
    for(int i = 0; i < input.size(); i++) {
        for(int k = 0; k < input[i].size(); k++) {
            cout<<input[i][k];
        }

        cout<<endl;
    }

    cout<<endl;
}

void getNeighbors(Coord &starting, vector<vector<int>> &input, vector<Coord> &neighbors) {
    if(starting.x > 0)
        neighbors.push_back(Coord(starting.x-1, starting.y));
    
    if(starting.x < input.size()-1)
        neighbors.push_back(Coord(starting.x+1, starting.y));

    if(starting.y > 0)
        neighbors.push_back(Coord(starting.x, starting.y-1));

    if(starting.y < input[starting.x].size()-1)
        neighbors.push_back(Coord(starting.x, starting.y+1));

    if(starting.x > 0 && starting.y > 0)
        neighbors.push_back(Coord(starting.x-1, starting.y-1));

    if(starting.x < input.size()-1 && starting.y < input[starting.x+1].size()-1) 
        neighbors.push_back(Coord(starting.x+1, starting.y+1));

    if(starting.x > 0 && starting.y < input[starting.x-1].size()-1)
        neighbors.push_back(Coord(starting.x-1, starting.y+1));

    if(starting.x < input.size()-1 && starting.y > 0) 
        neighbors.push_back(Coord(starting.x+1, starting.y-1));
}

bool hasVisited(vector<Coord> &visited, Coord &c) {
    for(int i = 0; i < visited.size(); i++) {
        if(visited[i] == c) return true;
    }

    return false;
}

bool allSynched(vector<vector<int>> &input) {
    for(int i = 0; i < input.size(); i++) {
        for(int k = 0; k < input[i].size(); k++) {
            if(input[i][k] != 0)  return false;
        }
    }

    return true;
}

void simulate(vector<vector<int>> &input, const int &STEPS) {
    print(input);
    long totalFlashes = 0;
    queue<Coord> flashes;
    vector<Coord> visited;
    int synchStep = -1;

    for(int i = 0; i < STEPS; i++) {
        for(int k = 0; k < input.size(); k++) {
            for(int j = 0; j < input[k].size(); j++) {
                input[k][j]++;

                if(input[k][j] > 9) {
                    Coord c(k,j);
                    flashes.push(c);
                    visited.push_back(c);
                }
            }
        }

        cout<<"Flashes size: "<<flashes.size()<<endl;

        while(!flashes.empty()) {
            Coord coord = flashes.front();
            flashes.pop();
            if(input[coord.x][coord.y] == 0) { continue; }

            if(input[coord.x][coord.y] < 9) {
                input[coord.x][coord.y]++;
                continue;
            }
            input[coord.x][coord.y] = 0;
            totalFlashes++;
            visited.push_back(coord);

            vector<Coord> neighbors;
            getNeighbors(coord, input, neighbors);

            for(int i = 0; i < neighbors.size(); i++) {
                if(!hasVisited(visited, neighbors[i])) {
                    flashes.push(neighbors[i]);
                    //cout<<"Current: ("<<coord.x<<","<<coord.y<<") Added: ("<<neighbors[i].x<<","<<neighbors[i].y<<")"<<" || ";
                }
                //cout<<endl;
            }
        }

        if(synchStep < 0 && allSynched(input)) synchStep = i;
        cout<<"STEP: "<<i+1<<endl;
        print(input);

        visited.clear();
    }

    cout<<"Total flashes: "<<totalFlashes<<endl;
    cout<<"Synchronized step: "<<synchStep+1<<endl;
}

int main() {
    const int STEPS = 1000;
    vector<vector<int>> input;
    readInput(input, "input.txt");
    simulate(input, STEPS);
}
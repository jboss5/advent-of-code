#include <iostream>
#include <string>
#include <vector>
#include <fstream>
#include <queue>
#include <map>
#include <cmath>

using std::cout;
using std::endl;
using std::string;
using std::vector;
using std::priority_queue;
using std::map;

struct Node {
    int x{}, y{};
    double costSoFar = 0;

    Node() = default;
    Node(int yVal, int xVal) : x(xVal), y(yVal) {}
    string to_string() { return "(" + std::to_string(x) + "," + std::to_string(y) + ")"; }
};

bool operator==(const Node &lhs, const Node &rhs) {
    return (lhs.x == rhs.x && lhs.y == rhs.y);
}

bool operator!=(const Node &lhs, const Node &rhs) {
    return (lhs.x != rhs.x || lhs.y != rhs.y);
}

bool operator<(const Node &lhs, const Node &rhs) {
    return (lhs.x < rhs.x) || (lhs.x == rhs.x && lhs.y < rhs.y);
}

struct CompareNodePriority {
    bool operator()(const Node &lhs, const Node &rhs) {
        return lhs.costSoFar > rhs.costSoFar;
    }
};

void readInput(vector<vector<int>> &input, const string &filename) {
    std::ifstream in(filename);
    for(string line; getline(in, line);) {
        vector<int> temp;
        for(int i = 0; i < line.size(); i++) {
            temp.push_back(line.at(i) - '0');
        }

        input.push_back(temp);
    }
}

void print(vector<vector<int>> &input) {
    for(auto & i : input) {
        for(int k : i) {
            cout<<k;
        }
        cout<<endl;
    }

    cout<<endl;
}

vector<Node> getNeighbors(Node &current, Node &end) {
    vector<Node> output;
    if(current.y > 0) output.emplace_back(current.y - 1, current.x);
    if(current.x > 0) output.emplace_back(current.y, current.x - 1);
    if(current.y < end.y) output.emplace_back(current.y + 1, current.x);
    if(current.x < end.x) output.emplace_back(current.y, current.x + 1);

    return output;
}

int getCost(Node &current, Node &edge, vector<vector<int>> &input) {
    return input[edge.y][edge.x];
}

double calcDistance(Node &end, Node &n) {
    return std::abs(end.x - n.x) + std::abs(end.y - n.y);
}

void part1(vector<vector<int>> &input) {
//    print(input);

    map<Node, double> costMap;
    map<Node, Node> pathMap;
    priority_queue<Node, vector<Node>, CompareNodePriority> q;
    Node start(0,0);
    Node end(input.size()-1, input[input.size()-1].size()-1);
    q.push(start);
    pathMap[start] = Node(-1,-1);

    while(!q.empty()) {
        Node current = q.top();
//        cout<<current.to_string()<<endl;
        q.pop();

        if(current == end) break;
        for(Node n : getNeighbors(current, end)) {
            double cost = costMap[current] + getCost(current, n, input);
//            cout<<"cost: "<<cost<<endl;
            auto iff = costMap.find(n);
            auto iffe = costMap.end();
            if(iff == costMap.end() || cost < costMap[n]) {
//                cout<<"added"<<endl;
                costMap[n] = cost;
                n.costSoFar = cost + calcDistance(end, n);
                q.push(n);
                pathMap[n] = current;
            }
        }
    }

    auto it = pathMap.find(end);
    int lowestCost = 0;
    while(it != pathMap.end()) {
        if(it->second.x == -1 && it->second.y == -1) break;
        lowestCost += input[it->first.y][it->first.x];
        input[it->first.y][it->first.x] = 0;
        it = pathMap.find(it->second);
    }

    cout<<endl;
//    print(input);

    cout<<"Lowest cost: "<<lowestCost<<endl;
}

void part2(vector<vector<int>> &input, const int &maxSize) {
    vector<vector<int>> bigMatrix(input);
    int origSize = input.size();

    bigMatrix.resize(maxSize);
    for(auto & i : bigMatrix) {
        i.resize(maxSize);
    }

    for(int y = 0; y < maxSize; y++) {
        for(int x = 0; x < maxSize; x++) {
            int oldY = y / origSize;
            int oldX = x / origSize;
            int originalVal = input[y % origSize][x % origSize];
            originalVal += (oldY + oldX);
            if(originalVal > 9) originalVal -= 9;
            bigMatrix[y][x] = originalVal;
        }
    }

//    print(bigMatrix);
    part1(bigMatrix);
}

int main() {
    vector<vector<int>> input;
    readInput(input, "input.txt");
    int maxSize = input.size()*5;
    part2(input, maxSize);

    return 0;
}
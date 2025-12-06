#include <iostream>
#include <string>
#include <list>
#include <vector>
#include <fstream>
#include <sstream>
#include <map>
#include <set>
#include <queue>
#include <algorithm>

using std::cout;
using std::endl;
using std::string;
using std::vector;
using std::list;
using std::map;
using std::queue;
using std::set;

long smallCaveCount = 0;
long totalCaveCount = 0;
long smallCaveCountP2 = 0;
long totalCaveCountP2 = 0;
set<string> smallCaves;
vector<string> outputLinesP2;

class Graph {
    public:
        Graph() {}
        map<string, vector<string>> graphMap;
        void addNode(string &start, string &end) {
            if(graphMap.find(start) != graphMap.end()) {
                graphMap.at(start).push_back(end);
            } else {
                cout<<"adding "<<start<<endl;
                vector<string> temp;
                temp.push_back(end);
                graphMap[start] = temp;
            }
        }
};

bool isBigCave(string str) {
    for(int i = 0; i < str.size(); i++) {
        if(!std::isupper(str.at(i))) return false;
    }

    return true;
}

bool contains(string search, vector<string> &input) {
    //cout<<"comapring "<<search<<endl;

    for(int i = 0; i < input.size(); i++) {
        //cout<<"input: "<<input[i]<<" ";
        if(input[i] == search) return true;
    }

    //cout<<"not contains"<<endl;
    return false;
}

bool containsPt2(string search, vector<string> &input) {
    map<string,int> visitCount;
    for(int i = 0; i < input.size(); i++) {
        if(!isBigCave(input[i])) {
            if(visitCount.find(input[i]) != visitCount.end()) {
                visitCount[input[i]]++;
            } else {
                visitCount[input[i]] = 1;
            }
        }
    }

    if(visitCount.find(search) == visitCount.end()) return true;
    else {
        if(visitCount[search] >= 2) return false;
        else {
            for(map<string,int>::iterator it = visitCount.begin(); it != visitCount.end(); it++) {
                if(it->first != search && it->second >= 2) return false;
            }
        }
    }

    return true;
}

void printPath2(Graph &g, string start, string end, vector<string> visited, list<string> path) {
    //cout<<"starting with "<<start<<" and "<<end<<endl;
    if(start == end) {
        cout<<"Ending string :: ";
        bool hasIncremented = false;
        for(string str : path) {
            cout<<str<<" ";

            if(!hasIncremented && !isBigCave(str) && str != "start" && str != "end") {
                smallCaveCount++;
                hasIncremented = true;
            }
        }

        cout<<endl;
        totalCaveCount++;
        return;
    }

    vector<string>::iterator it = visited.insert(visited.end(), start);
    vector<string> verts = g.graphMap[start];
    for(string str : verts) {
        //cout<<"Checking: "<<str<<endl;
        if(!contains(str, visited) || isBigCave(str)) {
            //cout<<"Going to not visited "<<str<<endl;
            list<string>::iterator it = path.insert(path.end(), str);
            printPath2(g, str, end, visited, path);
            path.erase(it);
        }
    }

    //cout<<"erasing..."<<endl;
    visited.erase(it);
}

void printPath3(Graph &g, string start, string end, vector<string> visited, list<string> path, map<string, int> &visitCountMap) {
    cout<<"starting with "<<start<<" and "<<end<<endl;
    if(start == end) {
        cout<<"Ending string :: ";
        bool hasIncremented = false;
        string outputStr;
        for(string str : path) {
            //cout<<str<<" ";
            //if(str != "start" && str != "end") 
            outputStr += str + " ";

            if(!hasIncremented && !isBigCave(str) && str != "start" && str != "end") {
                smallCaveCountP2++;
                hasIncremented = true;
            }
        }

        cout<<outputStr<<endl;
        totalCaveCountP2++;
        //outputLinesP2.push_back(outputStr);
        return;
    }

    //cout<<"adding "<<start<<endl;
    //if(!isBigCave(start) && smallCaves.find(start) == smallCaves.end() && start != "start" && start != "end") smallCaves.insert(start);
    vector<string>::iterator it = visited.insert(visited.end(), start);
    vector<string> verts = g.graphMap[start];
    for(string str : verts) {
        cout<<"Checking: "<<str<<endl;
        if(containsPt2(str, visited) || isBigCave(str)) {
            cout<<"Going to not visited "<<str<<endl;
            list<string>::iterator it = path.insert(path.end(), str);
            //addToMap(str, visitCountMap);
            printPath3(g, str, end, visited, path, visitCountMap);
            path.erase(it);
            //decrement(str, visitCountMap);
        }
    }

    //cout<<"erasing..."<<endl;
    visited.erase(it);
}

void readInput(const string &filename) {
    std::ifstream in(filename);
    vector<string> lines;
    Graph g;

    for(string line; getline(in, line);) { 
        string lhs, rhs;
        std::stringstream ss(line);
        getline(ss, lhs, '-');
        getline(ss, rhs, '-');

        if(rhs == "start") {
            g.addNode(rhs, lhs);
        } else if(lhs == "start") {
            g.addNode(lhs,rhs);
        } else {
            g.addNode(lhs,rhs);
            g.addNode(rhs,lhs);
        }
    }

    for(map<string, vector<string>>::iterator it = g.graphMap.begin(); it != g.graphMap.end(); it++) {
        cout<<"["<<it->first<<":";
        for(int i = 0; i < it->second.size(); i++) {
            cout<<it->second[i]<<",";
        }
        cout<<"]"<<endl;
    }

    vector<string> visited;
    list<string> path;
    path.push_back("start");
    printPath2(g, "start", "end", visited, path);
    cout<<endl<<endl<<endl<<"========== Part 2 =========="<<endl;

    visited.clear();
    path.clear();
    path.push_back("start");
    map<string, int> visitCountMap;
    printPath3(g, "start", "end", visited, path, visitCountMap);
}


int main() {
    readInput("sample-input1.txt");
    cout<<"smallCaveCount: "<<smallCaveCount<<endl;
    cout<<"totalCaveCount: "<<totalCaveCount<<endl;
    cout<<"smallCaveCountP2: "<<smallCaveCountP2<<endl;
    cout<<"totalCaveCountP2: "<<totalCaveCountP2<<endl;

    return 0;
}

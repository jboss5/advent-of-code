#include <iostream>
#include <string>
#include <vector>
#include <fstream>
#include <stack>
#include <queue>
#include <deque>
#include <climits>

using std::cout;
using std::string;
using std::vector;
using std::endl;
using std::stack;
using std::queue;
using std::deque;

int start = 0;
int versionTotal = 0;
void getPacket(const string &packet, stack<string> &valueStack);

void readInput(vector<char> &input, const string &filename) {
    std::ifstream in(filename);
    char c;
    while(in.get(c)) {
        input.push_back(c);
    }
}

string convertHexToBin(const char &hex) {
    switch(hex) {
        case '0':
            return "0000";
        case '1':
            return "0001";
        case '2':
            return "0010";
        case '3':
            return "0011";
        case '4':
            return "0100";
        case '5':
            return "0101";
        case '6':
            return "0110";
        case '7':
            return "0111";
        case '8':
            return "1000";
        case '9':
            return "1001";
        case 'A':
            return "1010";
        case 'B':
            return "1011";
        case 'C':
            return "1100";
        case 'D':
            return "1101";
        case 'E':
            return "1110";
        case 'F':
            return "1111";
        default:
            return "";
    }
}

string getOperator(const int &type) {
    
    switch(type) {
        case 0:
            return "+";
        case 1:
            return "*";
        case 2:
            return "min";
        case 3:
            return "max";
        case 5:
            return ">";
        case 6:
            return "<";
        case 7:
            return "==";
        default:
            return nullptr;
    }
}

int getPacketSize(char &ch) {
    return (ch == '0') ? 15 : 11;
}

int getNumPackets(string str) {
    return std::stoi(str, nullptr, 2);
}

unsigned long long bin2int(string str) {
    return std::stoull(str, nullptr, 2);
}

bool allZeroes(const string &packet) {
    for(int i = 0; i < packet.length(); i++) {
        if(packet[i] != '0') {
            return false;
        }
    }

    return true;
}

void getLiteral(const string &packet, stack<string> &valueStack) {
    string val;
    string outputBin;
    do {
        val = packet.substr(start,5);
        // literal has first bit cut-off
        outputBin += val.substr(1);
        start+=5;
    } while(val[0] != '0');

    valueStack.push(std::to_string(bin2int(outputBin)));
}

void getOperatorPacket(int packetType, const string &packet, stack<string> &valueStack) {
    string op = getOperator(packetType);
    valueStack.push(op);
    valueStack.push("(");
    char lbit = packet[start];
    int size = getPacketSize(lbit);
    start++;
    int numPackets = bin2int(packet.substr(start, size));
    start+=size;

    if(size == 15) {
        int end = start+numPackets;
        while(start < end) {
            getPacket(packet, valueStack);
        }
    } else {
        for(int i = 0; i < numPackets; i++) {
            getPacket(packet, valueStack);
        }
    }

    valueStack.push(")");
}

void getPacket(const string &packet, stack<string> &valueStack) {
    if(allZeroes(packet.substr(start))) { return; }

    int packetVersion = bin2int(packet.substr(start,3));
    versionTotal += packetVersion;
    int packetType = bin2int(packet.substr(start+3,3));
    start+=6;

    if(packetType == 4) {
        getLiteral(packet, valueStack);
    } else {
        getOperatorPacket(packetType, packet, valueStack);
    }
}

unsigned long long evaluateExpression(const string &type, const vector<unsigned long long> &values) {
    unsigned long long total = 0ULL;

    if(values.size() == 1) { return values[0]; }

    if(type=="+") {
        for(int i = 0; i < values.size(); i++) {
            total += values[i];
        }
    } else if(type=="*") {
        unsigned long long temp = values[0];
        for(int i = 1; i < values.size(); i++) {
            temp *= values[i];
        }

        total = temp;
    } else if(type=="min") {
        unsigned long long temp = ULLONG_MAX;
        for(int i = 0; i < values.size(); i++) {
            if(values[i] < temp) {
                temp = values[i];
            }
        }

        total = temp;
    } else if(type=="max") {
        unsigned long long temp = 0ULL;
        for(int i = 0; i < values.size(); i++) {
            if(values[i] > temp) {
                temp = values[i];
            }
        }

        total = temp;
    } else if(type==">") {
        total = (values[0] > values[1]) ? 1 : 0;
    } else if(type=="<") {
        total = (values[0] < values[1]) ? 1 : 0;    
    } else if(type=="==") {
        total = (values[0] == values[1]) ? 1 : 0;
    }

    return total;
}

bool isOperator(const string &str) {
    return (str == "+" || str == "*" || str == "min" || str == "max" || str == ">" || str == "<" || str == "==");
}

unsigned long long evaluateStack(stack<string> &valueStack) {
    stack<string> answerStack;

    while(!valueStack.empty()) {
        string item = valueStack.top();
        valueStack.pop();

        if(isOperator(item)) {
            // operator
            vector<unsigned long long> values;
            string t;
            do {
                t = answerStack.top();
                answerStack.pop();
                if(t != "(" && t != ")") { values.push_back(std::stoull(t)); }
            } while(t != ")");

            answerStack.push(std::to_string(evaluateExpression(item, values)));
        } else {
            // value or parenthesis
            answerStack.push(item);
        }
    }

    return std::stoull(answerStack.top());
}

void part1(const string &filename) {
    vector<char> input;
    readInput(input, filename);
    string packetStr;
    for(char &ch : input) {
        packetStr.append(convertHexToBin(ch));
    }

    cout << "packet str: " << packetStr << endl;

    stack<string> valueStack;
    getPacket(packetStr, valueStack);

    cout << "version total: " << versionTotal << endl;
    cout << "total value: " << evaluateStack(valueStack) << endl;
}

int main() {

    cout<<"Part1 & Part2:\n";
    part1("sample-input.txt");

    return 0;
}
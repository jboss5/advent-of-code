#include <iostream>
#include <fstream>
#include <string>

using namespace std;

int main()
{
    const char SPACE = ' ';
    const string FORWARD = "forward";
    const string DOWN = "down";
    const string UP = "up";

    int horizontalPosition = 0;
    int depth = 0;
    int aim = 0;

    fstream in("input.txt");
    string operation;
    int num;
    while(in >> operation >> num)
    {
        if(operation == FORWARD)
        {
            horizontalPosition += num;
            depth += (aim * num);
        }
        else if(operation == DOWN)  aim += num;
        else if(operation == UP)    aim -= num;
    }

    in.close();
    
    cout << "Depth: " << depth << "\n";
    cout << "Horizontal Position: " << horizontalPosition << "\n";
    cout << "Aim: " << aim << "\n";
    cout << depth * horizontalPosition <<"\n";
    return 0;
}
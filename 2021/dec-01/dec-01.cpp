#include <iostream>
#include <fstream>
#include <vector>
#include <string>

using namespace std;

int getNumIncreases(vector<int> inputData)
{
    int previous=-1;
    int numIncrease=0;

    for(int val : inputData)
    {
        if(previous >= 0 && val > previous)
        {
            numIncrease++;
        }

        previous = val;
    }

    return numIncrease;
}

int main()
{
    ifstream in("input.txt");
    vector<int> input;

    string line;
    while(in >> line)
    {
        input.push_back(stoi(line));
    }
    in.close();

    vector<int> threesSums;
    for(int i = 2; i < input.size(); i++)
    {
        int sum = input[i-2] + input[i-1] + input[i];
        //cout << sum << "\n";
        threesSums.push_back(sum);
    }

    cout << getNumIncreases(threesSums);
    return 0;
}
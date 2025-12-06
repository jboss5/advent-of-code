#include <iostream>
#include <fstream>
#include <vector>
#include <string>
#include <math.h>

using namespace std;

vector<vector<char>> buildInputVector()
{
    vector<vector<char>> data;
    fstream file("input.txt");

    int index = 0;
    int y = 0;
    char ch;
    vector<char> temp;

    while(file.get(ch))
    {
        if(ch == '\n')
        {
            data.insert(data.begin() + index, temp);
            temp.clear();
            index++;
            continue; 
        }
        
        temp.push_back(ch);
    }
    file.close();

    return data;
}

int getSumOfIndex(int index, vector<vector<char>> data)
{
    int value = 0;
    for(int lineNum = 0; lineNum < data.size(); lineNum++)
    {
        value += data[lineNum][index] - '0';
    }

    return value;
}

string getGammaRate(vector<vector<char>> data)
{
    string output = "";
    int charNum = 0;
    int totalChars = data[0].size(); // input data is squared
    for(int i = 0; i < totalChars; i++)
    {
        int value = getSumOfIndex(i, data);

        // if 1 is most common, value is > half of the size
        // otherwise 0 is most common
        if(value > (data.size() / 2)) { output += '1'; }
        else { output += '0'; }
    }

    return output;
}

string getEpsilonRate(string gammaRate)
{
    string output = "";
    for(int i = 0; i < gammaRate.size(); i++)
    {
        char ch = gammaRate.at(i);
        if(ch == '1') { output += '0'; }
        else { output += '1'; }
    }

    return output;
}

string createString(vector<char> vector)
{
    string output = "";
    for(int i = 0; i < vector.size(); i++)
    {
        output += vector[i];
    }

    return output;
}

void print(vector<vector<char>> data)
{
    for(int i = 0; i < data.size(); i++)
    {
        cout << createString(data[i]) << '\n';
    }

    cout << "\n\n";
}

void removeEntries(vector<vector<char>> &data, char criteria, int pos)
{
    vector<vector<char>>::iterator iterator = data.begin();
    int i = 0;
    while(iterator != data.end())
    {
        if((*iterator)[pos] == criteria) { data.erase(iterator); }
        else { iterator++; }
    }
}

string getOxygenGenRating(vector<vector<char>> originalData)
{
    vector<vector<char>> inputData(originalData);

    int totalChars = inputData[0].size(); // input data is squared
    for(int i = 0; i < totalChars; i++)
    {
        int value = getSumOfIndex(i, inputData);
        int half = round(inputData.size() / 2.0);

        if(value >= half) { removeEntries(inputData, '0', i); }
        else { removeEntries(inputData, '1', i); }

        if(inputData.size() == 1) { return createString(inputData[0]); }
    }

    return "";
}

string getScrubberRating(vector<vector<char>> originalData)
{
    vector<vector<char>> inputData(originalData);
    int totalChars = inputData[0].size(); // input data is squared
    for(int i = 0; i < totalChars; i++)
    {
        int value = getSumOfIndex(i, inputData);
        int half = round(inputData.size() / 2.0);

        if(value >= half) { removeEntries(inputData, '1', i); }
        else { removeEntries(inputData, '0', i);  }

        if(inputData.size() == 1) { return createString(inputData[0]); }
    }

    return "";
}

void part2(vector<vector<char>> &inputData)
{
    long oxygenGenRating = stoull(getOxygenGenRating(inputData),0,2);
    long scrubberRating = stoull(getScrubberRating(inputData),0,2);

    cout << "oxygen rating " << oxygenGenRating << '\n';
    cout << "co2 scrubber rating " << scrubberRating << '\n';
    cout << "life support " << (oxygenGenRating * scrubberRating) << '\n';
}

void part1(vector<vector<char>> &inputData)
{
    string gammaRateStr = getGammaRate(inputData);
    long gammaRate = stoull(gammaRateStr,0,2);
    long epsilonRate = stoull(getEpsilonRate(gammaRateStr),0,2);

    cout << "Decimal Gamma: " << gammaRate << '\n';
    cout << "Decimal Epsilon: " << epsilonRate << '\n';
    cout << "Multiplied: " << (gammaRate * epsilonRate) << '\n';
}

int main()
{
    vector<vector<char>> inputData = buildInputVector();
    cout << "Part1:\n";
    part1(inputData);
    cout << "\nPart2:\n";
    part2(inputData);

    return 0;
}
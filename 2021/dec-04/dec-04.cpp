#include "bingo-header.h"
#include <iostream>
#include <vector>
#include <fstream>
#include <sstream>

std::string loadInputData(std::string filename, std::vector<BingoCard> *bingoCardList) {
    std::ifstream in(filename);
    std::string line, drawnNumberLine;

    int lineNum = 0;
    BingoCard *bingoCard = new BingoCard;
    while(getline(in, line)) {
        if(lineNum == 0) { drawnNumberLine = line; }
        if(lineNum <= 1) { lineNum++; continue; } // skip second line of file

        if(line == "") {
            bingoCardList->push_back(*bingoCard);
            bingoCard = new BingoCard;
        } else {
            bingoCard->addLineToCard(line);
        }
    }

    bingoCardList->push_back(*bingoCard);
    in.close();
    return drawnNumberLine;
}

int getWinningBingoCard(std::string drawnNumberLine, std::vector<BingoCard> bingoCardList, BingoCard &winner, int &winnerIndex) {
    std::stringstream stream(drawnNumberLine);
    std::string draw;
    while(getline(stream, draw, ',')) {
        int drawNum = std::stoi(draw);
        for(int i = 0; i < bingoCardList.size(); i++) { 
            bingoCardList[i].hitValue(drawNum);
            if(bingoCardList[i].isWinner()) {
                winner = bingoCardList[i];
                winnerIndex = i;
                return drawNum;
            }
        }
    }

    return 0;
}

void part1(std::vector<BingoCard> bingoCardList, std::string drawnNumberLine) {
    std::cout << "drawnNumberLine " << drawnNumberLine << '\n';
    std::cout << "number of cards: " << bingoCardList.size() << '\n';

    BingoCard winner;
    int winnerIndex;
    int winningNumber = getWinningBingoCard(drawnNumberLine, bingoCardList, winner, winnerIndex);

    std::cout << "Winning number: " << winningNumber << '\n';
    std::cout << "Winning sum: " << winner.cardSum() << '\n';
    std::cout << "Multiplied: " << winningNumber * winner.cardSum() << '\n';
}

void part2(std::vector<BingoCard> bingoCardList, std::string drawnNumberLine) {
    std::cout << "drawnNumberLine " << drawnNumberLine << '\n';
    std::cout << "number of cards: " << bingoCardList.size() << '\n';

    BingoCard winner;
    int winningNumber;
    
    std::vector<BingoCard>::iterator iterator = bingoCardList.begin();
    int winnerIndex;
    while(iterator != bingoCardList.end()) {
        winningNumber = getWinningBingoCard(drawnNumberLine, bingoCardList, winner, winnerIndex);
        bingoCardList.erase(bingoCardList.begin() + winnerIndex);
    }

    std::cout << "Winning number: " << winningNumber << '\n';
    std::cout << "Winning sum: " << winner.cardSum() << '\n';
    std::cout << "Multiplied: " << winningNumber * winner.cardSum() << '\n';
}

int main() {
    std::vector<BingoCard> bingoCardList;
    std::string drawnNumberLine = loadInputData("input.txt", &bingoCardList);
    
    std::cout<<"Part1:\n";
    part1(bingoCardList, drawnNumberLine);
    std::cout<<"\nPart2:\n";
    part2(bingoCardList, drawnNumberLine);

    return 0;
}
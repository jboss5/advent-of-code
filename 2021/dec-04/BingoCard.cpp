#include "bingo-header.h"
#include <iostream>
#include <vector>
#include <sstream>

BingoCard::BingoCard() {
    // do nothing
}

BingoCard::~BingoCard() {
    cardData.clear();
}

bool BingoCard::isWinner() {
    // check row winner
    for(int i = 0; i < cardData.size(); i++) {
        int rowHits = 0;
        for(int k = 0; k < cardData[i].size(); k++) {
            if(cardData[i][k].isHit()) { rowHits++; }
        }

        if(rowHits == 5) { return true; } // row winner, only 5 numbers per row
    }

    // check column winner, only 5 rows in each card
    // cards are squared
    for(int k = 0; k < cardData.size(); k++) {
        int colHits = 0;
        for(int i = 0; i < cardData.size(); i++) {
            if(cardData[i][k].isHit()) { colHits++; }
        }

        if(colHits == 5) { return true; } // column winner, only 5 numbers per column
    }

    return false;
}

void BingoCard::hitValue(int value) {
    for(int i = 0; i < cardData.size(); i++) {
        for(int k = 0; k < cardData[i].size(); k++) {
            if(cardData[i][k].getValue() == value) {
                cardData[i][k].setIsHit(true);
                break; // number cannot be found again on this card
            }
        }
    }
}

void BingoCard::addLineToCard(const std::string &line) {
    std::vector<BingoValue> valueList;
    std::stringstream stream(line);
    std::string token = "";
    std::string buffer;
    while(stream >> buffer) {
       valueList.push_back(BingoValue(std::stoi(buffer), false));
    }

    cardData.push_back(valueList);
}

void BingoCard::print() {
    std::cout << "card data size: " << cardData.size() << '\n';
    for(int i = 0; i < cardData.size(); i++) {
        std::vector<BingoValue> temp = cardData[i];
        for(int k = 0; k < temp.size(); k++) {
            std::cout << temp[k].getValue() << '(' << (temp[k].isHit() ? 't' : 'f') << ") ";
        }
        std::cout << '\n';
    }
}

int BingoCard::cardSum() {
    int sum = 0;
    for(int i = 0; i < cardData.size(); i++) {
        for(int k = 0; k < cardData[i].size(); k++) {
            if(!cardData[i][k].isHit()) { sum += cardData[i][k].getValue(); }
        }
    }

    return sum;
}
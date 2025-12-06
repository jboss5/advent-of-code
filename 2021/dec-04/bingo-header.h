#ifndef BINGO_HEADER_H
#define BINGO_HEADER_H

#include <iostream>
#include <vector>

class BingoValue {
    public:
        BingoValue(int,bool);
        void setIsHit(bool);
        bool isHit();
        void setValue(int);
        int getValue();

    private:
        int value;
        bool hitFlag;
};

class BingoCard {
    public:
        BingoCard();
        ~BingoCard();
        bool isWinner();
        void hitValue(int);
        void addLineToCard(const std::string &);
        void print();
        int cardSum();

    private:
        std::vector<std::vector<BingoValue>> cardData;
        
};

#endif
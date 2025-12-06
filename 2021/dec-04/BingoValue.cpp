#include "bingo-header.h"
#include <iostream>

BingoValue::BingoValue(int value, bool hitFlag) {
    this->value = value;
    this->hitFlag = hitFlag;
}

void BingoValue::setIsHit(bool hitFlag) {
    this->hitFlag = hitFlag;
}

bool BingoValue::isHit() {
    return this->hitFlag;
}

void BingoValue::setValue(int value) {
    this->value = value;
}

int BingoValue::getValue() {
    return this->value;
}
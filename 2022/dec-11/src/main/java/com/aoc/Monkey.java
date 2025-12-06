package com.aoc;

import java.util.ArrayList;
import java.util.List;

public class Monkey {

    public enum OperationType {
        ADD,
        MULTIPLY,
        SUBTRACT,
        DIVIDE;

        public static OperationType fromString(String str) {
            switch(str) {
                case "*": return MULTIPLY;
                case "+": return ADD;
                case "-": return SUBTRACT;
                case "/": return DIVIDE;
                default: throw new RuntimeException("Invalid type: " + str);
            }
        }
    }

    private List<Long> heldItems = new ArrayList<>();
    private String operationString = "";
    private long testNumber = 0;
    private OperationType operationType;
    private int monkeyToTrue = -1;
    private int monkeyToFalse = -1;

    public Monkey() {}

    public Monkey cloneIt() {
        return new Monkey()
                .setHeldItems(new ArrayList<>(heldItems))
                .setOperationString(operationString)
                .setTestNumber(testNumber)
                .setOperationType(operationType)
                .setMonkeyToFalse(monkeyToFalse)
                .setMonkeyToTrue(monkeyToTrue);
    }

    public long computeWorryLevel(long worryLevel, int divisor) {
        long operationNumber;
        if (operationString.equals("old")) {
            operationNumber = worryLevel;
        } else {
            operationNumber = Long.parseLong(operationString);
        }

        switch (operationType) {
            case ADD:
                worryLevel += operationNumber;
                break;
            case MULTIPLY:
                worryLevel *= operationNumber;
                break;
            case DIVIDE:
                worryLevel /= operationNumber;
                break;
            case SUBTRACT:
                worryLevel -= operationNumber;
                break;
        }

        return worryLevel / divisor;
    }

    public void removeItem(int index) {
        heldItems.remove(index);
    }

    public void addItem(Long item) {
        heldItems.add(item);
    }

    public OperationType getOperationType() {
        return operationType;
    }

    public Monkey setOperationType(OperationType operationType) {
        this.operationType = operationType;
        return this;
    }

    public int getMonkeyToTrue() {
        return monkeyToTrue;
    }

    public Monkey setMonkeyToTrue(int monkeyToTrue) {
        this.monkeyToTrue = monkeyToTrue;
        return this;
    }

    public int getMonkeyToFalse() {
        return monkeyToFalse;
    }

    public Monkey setMonkeyToFalse(int monkeyToFalse) {
        this.monkeyToFalse = monkeyToFalse;
        return this;
    }

    public List<Long> getHeldItems() {
        return heldItems;
    }

    public Monkey setHeldItems(List<Long> heldItems) {
        this.heldItems = heldItems;
        return this;
    }

    public String getOperationString() {
        return operationString;
    }

    public Monkey setOperationString(String operationString) {
        this.operationString = operationString;
        return this;
    }

    public long getTestNumber() {
        return testNumber;
    }

    public Monkey setTestNumber(long testNumber) {
        this.testNumber = testNumber;
        return this;
    }
}

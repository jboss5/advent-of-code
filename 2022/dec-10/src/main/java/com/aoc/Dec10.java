package com.aoc;

import java.io.File;
import java.util.ArrayDeque;
import java.util.Collections;
import java.util.HashSet;
import java.util.Queue;
import java.util.Set;
import java.util.function.Consumer;

public class Dec10 {

    public static void main(String[] args) {
        File file = new File("input.txt");
        long duration = AOCUtils.timeTask(() -> {
            SignalConsumer signalConsumer = new SignalConsumer();
            try { AOCUtils.readFileByLine(file, signalConsumer); }
            catch(Exception e) { throw new RuntimeException(e); }

            System.out.printf("\nPart1: %d\n", signalConsumer.compute());
        });

        AOCUtils.printTimeBlock(duration);
    }
}

class SignalConsumer implements Consumer<String> {

    private static final int SCREEN_WIDTH = 40;
    private Queue<Operation> operationQueue = new ArrayDeque<>();

    @Override
    public void accept(String s) {
        String[] contents = s.split(" ");
        Operation operation;
        switch(contents[0]) {
            case "noop":
                operation = new Operation(Operation.Type.NOOP);
                break;
            case "addx":
                operation = new Operation(Operation.Type.ADDX, Integer.parseInt(contents[1]));
                break;
            default:
                throw new RuntimeException("Invalid operation " + contents[0]);
        }

        operationQueue.add(operation);
    }

    public int compute() {
        Set<Integer> signalChecks = new HashSet<>();
        Collections.addAll(signalChecks, 20, 60, 100, 140, 180, 220);

        int totalTicks = 1;
        int register = 1;
        int signalStrength = 0;
        int pixel = 0;
        boolean inOperation = false;
        while(!operationQueue.isEmpty()) {
            paintScreen(pixel++, register);

            if (signalChecks.contains(totalTicks)) {
                signalStrength += (register * totalTicks);
            }

            if(totalTicks % SCREEN_WIDTH == 0) {
                moveLine();
                pixel = 0;
            }

            Operation temp = operationQueue.peek();
            if(temp.getType() == Operation.Type.NOOP) {
                operationQueue.remove();
            } else if(temp.getType() == Operation.Type.ADDX) {
                if(!inOperation) {
                    inOperation = true;
                } else {
                    inOperation = false;
                    register += operationQueue.remove().getValue();
                }
            }

            totalTicks++;
        }

        return signalStrength;
    }

    private void paintScreen(int pixel, int register) {
        if(pixel == register-1 || pixel == register || pixel == register+1) {
            System.out.print('#');
        } else {
            System.out.print('.');
        }
    }

    private void moveLine() {
        System.out.print('\n');
    }
}

class Operation {

    public enum Type {
        NOOP,
        ADDX
    }

    private Type type;
    private Integer value;

    public Operation(Type type) {
        this.type = type;
    }

    public Operation(Type type, Integer value) {
        this.type = type;
        this.value = value;
    }

    public Type getType() {
        return type;
    }

    public Integer getValue() {
        return value;
    }
}

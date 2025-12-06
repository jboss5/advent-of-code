package com.aoc;

import java.io.File;
import java.util.ArrayDeque;
import java.util.ArrayList;
import java.util.List;
import java.util.Queue;
import java.util.Stack;
import java.util.function.Consumer;

public class Dec05 {

    public static void main(String[] args) {
        File file = new File("input.txt");

        long duration = AOCUtils.timeTask(() -> {
            CraneConsumer craneConsumer = new CraneConsumer();
            try { AOCUtils.readFileByLine(file, craneConsumer); }
            catch(Exception e) { throw new RuntimeException(e); }

            System.out.printf("Top crates: %s\n", craneConsumer.part1());
            System.out.printf("Top crates p2: %s", craneConsumer.part2());
        });

        AOCUtils.printTimeBlock(duration);
    }
}

class CraneConsumer implements Consumer<String> {

    private List<ArrayDeque<Character>> crates = new ArrayList<>();
    private List<Operation> operations = new ArrayList<>();
    boolean inOperations = false;

    public CraneConsumer() {}

    @Override
    public void accept(String line) {
        if(!inOperations) {
            if (line.matches("^[ 1-9]+$")) {
                inOperations = true;
                return;
            }

            int ind = 0;
            while(ind < line.length()) {
                switch(line.charAt(ind)) {
                    case ' ':
                        ind++;
                        break;
                    case '[':
                        addToStack(ind+3, line.charAt(ind+1));
                        ind+=3;
                        break;
                }
            }
        } else {
            String temp = line.trim();
            if(temp.isEmpty()) { return; }

            String[] actions = temp.split(" ");
            int quantity = Integer.parseInt(actions[1]);
            int from = Integer.parseInt(actions[3]) - 1;
            int to = Integer.parseInt(actions[5]) - 1;
            operations.add(new Operation(quantity, from, to));
        }
    }

    private void addToStack(int incr, char crate) {
        int index = incr / 4;
        if(index >= crates.size()) {
            fillEmptyCrates(index);
        }

        ArrayDeque<Character> queue = crates.get(index);
        queue.add(crate);
    }

    private void fillEmptyCrates(int newMax) {
        for(int i = crates.size(); i <= newMax; i++) {
            crates.add(i, new ArrayDeque<>());
        }
    }

    public String part1() {
        List<ArrayDeque<Character>> crates = cloneCrates();
        for(Operation operation : operations) {
            Queue<Character> queue = crates.get(operation.getFromIndex());
            for(int i = 0; i < operation.getQuantity(); i++) {
                crates.get(operation.getToIndex())
                        .addFirst(queue.remove());
            }
        }

        return buildFirstCrates(crates);
    }

    public String part2() {
        List<ArrayDeque<Character>> crates = cloneCrates();
        for(Operation operation : operations) {
            Queue<Character> queue = crates.get(operation.getFromIndex());
            Stack<Character> moveItems = new Stack<>();
            for(int i = 0; i < operation.getQuantity(); i++) {
                moveItems.push(queue.remove());
            }

            while(!moveItems.isEmpty()) {
                crates.get(operation.getToIndex())
                        .addFirst(moveItems.pop());
            }
        }

        return buildFirstCrates(crates);
    }

    private String buildFirstCrates(List<ArrayDeque<Character>> crates) {
        StringBuilder outputString = new StringBuilder();
        for(ArrayDeque<Character> queue : crates) {
            outputString.append(queue.getFirst());
        }
        return outputString.toString();
    }

    private List<ArrayDeque<Character>> cloneCrates() {
        List<ArrayDeque<Character>> newCrates = new ArrayList<>(crates.size());
        crates.forEach(item -> newCrates.add(item.clone()));
        return newCrates;
    }
}

class Operation {
    private final int quantity;
    private final int fromIndex;
    private final int toIndex;

    public Operation(int quantity, int fromIndex, int toIndex) {
        this.quantity = quantity;
        this.fromIndex = fromIndex;
        this.toIndex = toIndex;
    }

    public int getQuantity() {
        return quantity;
    }

    public int getFromIndex() {
        return fromIndex;
    }

    public int getToIndex() {
        return toIndex;
    }
}

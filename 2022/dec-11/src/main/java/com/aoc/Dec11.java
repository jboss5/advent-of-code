package com.aoc;

import java.io.BufferedReader;
import java.io.File;
import java.io.FileReader;
import java.io.IOException;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.Comparator;
import java.util.List;
import java.util.concurrent.atomic.AtomicLong;

public class Dec11 {

    public static void main(String[] args) {
        File file = new File("input.txt");
        long duration = AOCUtils.timeTask(() -> {
            try {
                MonkeyHandler monkeyHandler = new MonkeyHandler(file);
                monkeyHandler.buildInput();
                System.out.printf("Part1: %d\n", monkeyHandler.compute(20, 3));
                System.out.printf("Part2: %d\n", monkeyHandler.compute(10000, 1));
                monkeyHandler.printHeldItems();
            } catch(Exception e) {
                throw new RuntimeException(e);
            }
        });

        AOCUtils.printTimeBlock(duration);
    }
}

class MonkeyHandler {

    private File file;
    private List<Monkey> monkeyList = new ArrayList<>();
    private long modulo = 1L;

    public MonkeyHandler(File file) {
        this.file = file;
    }

    public void buildInput() throws IOException {
        try(BufferedReader reader = new BufferedReader(new FileReader(file))) {
            String line;
            while((line = reader.readLine()) != null) {
                if (line.startsWith("Monkey")) {
                    Monkey monkey = new Monkey();
                    Arrays.stream(reader.readLine().trim().split(":")[1].split(","))
                            .forEach(str -> monkey.addItem(Long.parseLong(str.trim())));

                    String[] operation = reader.readLine().trim().split(" ");
                    monkey.setOperationType(Monkey.OperationType.fromString(operation[4].trim()));
                    monkey.setOperationString(operation[5].trim());

                    // All test divisors multiplied together => "simulates" having the arbitrary large number
                    // https://en.wikipedia.org/wiki/Modular_arithmetic#Properties
                    long testNumber = Long.parseLong(reader.readLine().trim().split(" ")[3].trim());
                    monkey.setTestNumber(testNumber);
                    modulo *= testNumber;

                    monkey.setMonkeyToTrue(Integer.parseInt(reader.readLine().trim().split(" ")[5].trim()));
                    monkey.setMonkeyToFalse(Integer.parseInt(reader.readLine().trim().split(" ")[5].trim()));
                    monkeyList.add(monkey);
                }
            }
        } catch(Exception e) {
            throw e;
        }
    }

    private List<Monkey> cloneMonkeys() {
        List<Monkey> newMonkeyList = new ArrayList<>(monkeyList.size());
        monkeyList.forEach(m -> newMonkeyList.add(m.cloneIt()));
        return newMonkeyList;
    }

    public long compute(final int ROUNDS, int divisor) {
        List<AtomicLong> monkeyInspections = new ArrayList<>(monkeyList.size());
        List<Monkey> monkeyListClone = cloneMonkeys();
        monkeyList.forEach(m -> monkeyInspections.add(new AtomicLong(0)));
        for(int i = 0; i < ROUNDS; i++) {
            for(int m = 0; m < monkeyListClone.size(); m++) {
                Monkey monkey = monkeyListClone.get(m);
                List<Integer> removeItems = new ArrayList<>();
                for(int it = 0; it < monkey.getHeldItems().size(); it++) {
                    long item = monkey.getHeldItems().get(it);
                    long value = monkey.computeWorryLevel(item, divisor);
                    int monkeyNumber;
                    if(value % monkey.getTestNumber() == 0) {
                        monkeyNumber = monkey.getMonkeyToTrue();
                    } else {
                        monkeyNumber = monkey.getMonkeyToFalse();
                    }

                    monkeyListClone.get(monkeyNumber).addItem(value % modulo);
                    removeItems.add(it);
                    monkeyInspections.get(m).addAndGet(1);
                }

                for(int rm = 0; rm < removeItems.size(); rm++) {
                    monkey.removeItem(removeItems.get(rm) - rm);
                }
            }
        }

        System.out.printf("inspections: %s\n", monkeyInspections);
        monkeyInspections.sort(Comparator.comparingLong(AtomicLong::longValue).reversed());

        return monkeyInspections.get(0).longValue() * monkeyInspections.get(1).longValue();
    }

    public void printHeldItems() {
        for(int i = 0; i < monkeyList.size(); i++) {
            System.out.printf("Monkey %d: %s\n", i, monkeyList.get(i).getHeldItems());
        }
    }
}
package com.aoc;

import java.io.BufferedReader;
import java.io.File;
import java.io.FileReader;
import java.util.ArrayList;
import java.util.List;
import java.util.Set;
import java.util.stream.Collectors;

public class Dec03 {

    private static final String INPUT = "input.txt";

    public static void main(String[] args) throws Exception {
        File file = new File(INPUT);
        List<Rucksack> rucksackList = new ArrayList<>();
        try(BufferedReader reader = new BufferedReader(new FileReader(file))) {
            String line;
            while((line = reader.readLine()) != null) {
                int compartmentSize = line.length()/2;
                System.out.printf("found rucksack: [%s] with compartment size: %d\n", line, compartmentSize);
                rucksackList.add(new Rucksack(line.substring(0, compartmentSize), line.substring(compartmentSize)));
            }
        }

        System.out.println();

        Dec03 dec03 = new Dec03(rucksackList);
        dec03.part1();
        System.out.println('\n');
        dec03.part2();
    }

    private final List<Rucksack> RUCKSACK_LIST;

    public Dec03(List<Rucksack> rucksackList) {
        this.RUCKSACK_LIST = rucksackList;
    }

    public void part1() {
        System.out.println("PART 1:");

        int sum = 0;
        for(Rucksack rucksack : RUCKSACK_LIST) {
            sum += rucksack.getSharedItems().stream()
                    .map(Rucksack::itemPriority)
                    .reduce(Integer::sum)
                    .get();
        }

        System.out.printf("Priority Sum: %d", sum);
    }

    public void part2() {
        System.out.println("PART 2:");
        List<Character> badges = new ArrayList<>();
        for(int i = 0; i < RUCKSACK_LIST.size(); i+=3) {
            Set<Character> temp = RUCKSACK_LIST.get(i).getGroup();
            for(int k = i+1; k < (i+3); k++) {
                Set<Character> sharedItems = RUCKSACK_LIST.get(k).getGroup();
                temp.retainAll(sharedItems);
            }

            badges.addAll(temp);
        }

        int sum = badges.stream()
                .map(Rucksack::itemPriority)
                .reduce(Integer::sum)
                .get();
        System.out.printf("Priority Sum: %d", sum);
    }
}

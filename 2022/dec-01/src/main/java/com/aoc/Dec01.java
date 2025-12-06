package com.aoc;

import java.io.BufferedReader;
import java.io.File;
import java.io.FileReader;
import java.util.ArrayList;
import java.util.Collections;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.Scanner;

public class Dec01 {

    public static void main(String[] args) {
        Scanner scanner = new Scanner(System.in);
        System.out.print("Enter input path: ");
        String input = scanner.nextLine();
        File file = new File(input);

        if(!file.exists() || !file.canRead()) {
            System.err.println("Cannot open/read input file " + input);
            System.exit(1);
        } else {
            new Dec01(file).runit();
        }
    }

    private final File file;

    public Dec01(File file) {
        this.file = file;
    }

    public void runit() {
        Map<Integer, List<Integer>> dataMap = categorizeInput(file);

        ArrayList<Integer> totals = new ArrayList<>(dataMap.size());
        dataMap.forEach((k, v) -> totals.add(v.stream().mapToInt(s -> s).sum()));

        totals.sort(Collections.reverseOrder());
        System.out.println("Largest calories: " + totals.get(0));

        System.out.println("Top 3 largest calories:");
        long total = 0L;
        for(int i = 0; i < 3; i++) {
            int sum = totals.get(i);
            total += sum;
            System.out.printf("\t- %d\n", sum);
        }

        System.out.printf("\nSum of top 3: %d", total);
    }

    private Map<Integer, List<Integer>> categorizeInput(File file) {
        int elf = 1;
        Map<Integer, List<Integer>> out = new HashMap<>();
        try(BufferedReader reader = new BufferedReader(new FileReader(file))) {
            String line;
            while ((line = reader.readLine()) != null) {
                if (line.isEmpty()) {
                    elf++;
                } else {
                    List<Integer> temp = out.get(elf);
                    if (temp == null) {
                        temp = new ArrayList<>(1);
                    }
                    temp.add(Integer.parseInt(line));
                    out.put(elf, temp);
                }
            }
        } catch(Exception e) {
            System.out.println("Error while categorizing input data");
            e.printStackTrace();
        }

        return out;
    }
}

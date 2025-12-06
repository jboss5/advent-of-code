package com.aoc;

import java.io.File;
import java.io.IOException;
import java.util.ArrayList;
import java.util.List;
import java.util.function.Consumer;
import java.util.stream.Collectors;

public class Dec06 {

    public static void main(String[] args) throws IOException {
        List<AOCInput> aocInputs = new ArrayList<>();
        aocInputs.add(new AOCInput(new File("input-sample.txt"), 5, 23));
        aocInputs.add(new AOCInput(new File("input-sample2.txt"), 6, 23));
        aocInputs.add(new AOCInput(new File("input-sample3.txt"), 10, 29));
        aocInputs.add(new AOCInput(new File("input-sample4.txt"), 11, 26));
        aocInputs.add(new AOCInput(new File("input.txt"), 1760, 2974));

        for(AOCInput aocInput : aocInputs) {
            SignalConsumer signalConsumer = new SignalConsumer();
            AOCUtils.readFileByLine(aocInput.getFile(), signalConsumer);

            int part1 = signalConsumer.part1();
            int part2 = signalConsumer.part2();
            if(!aocInput.doesP1Match(part1)) {
                System.err.printf("Found error in file [%s], [%d] != [%s]\n", aocInput.getFile().getName(), part1, aocInput.getP1Answer().toString());
            } else {
                System.out.printf("[%s] : Part 1 matched - %d\n", aocInput.getFile().getName(), part1);
            }

            if(!aocInput.doesP2Match(part2)) {
                System.err.printf("Found error in file [%s], [%d] != [%s]\n", aocInput.getFile().getName(), part2, aocInput.getP1Answer().toString());
            } else {
                System.out.printf("[%s] : Part 2 matched - %d\n", aocInput.getFile().getName(), part2);
            }

            System.out.println();
        }
    }
}

class SignalConsumer implements Consumer<String> {

    private String input = null;

    @Override
    public void accept(String input) {
        this.input = input;
    }

    private boolean hasUniquechars(String str) {
        return (str.chars().distinct().count() == str.length());

        /* Old way below, found above -- need to keep a mental note on .distinct() for streams
        return (str.chars()
                .boxed()
                .collect(Collectors.toSet())
                .size() == str.length());
         */
    }

    private int findCharAfterUnique(int size) {
        int start = 0;
        while((start < input.length())) {
            int end = start+size;
            if(hasUniquechars(input.substring(start, end))) {
                return end;
            }
            start++;
        }

        return -1;
    }

    public int part1() {
        return findCharAfterUnique(4);
    }

    public int part2() {
        return findCharAfterUnique(14);
    }
}

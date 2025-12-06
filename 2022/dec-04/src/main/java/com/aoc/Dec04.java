package com.aoc;

import java.io.File;
import java.util.concurrent.atomic.AtomicInteger;
import java.util.function.Consumer;

public class Dec04 {

    public static void main(String[] args) {
        final File file = new File("input.txt");
        long nanos = AOCUtils.timeTask(() -> {
            SectionIdConsumer sectionIdConsumer = new SectionIdConsumer(false);
            try { AOCUtils.readFileByLine(file, sectionIdConsumer); }
            catch(Exception e) { throw new RuntimeException(e); }

            System.out.printf("Number of contained ranges: %d\n", sectionIdConsumer.getContainedRanges());
            System.out.printf("Number of any overlaps: %d\n", sectionIdConsumer.getAnyOverlap());
        });

        AOCUtils.printTimeBlock(nanos);
    }
}

class SectionIdConsumer implements Consumer<String> {

    private AtomicInteger containedRanges = new AtomicInteger(0);
    private AtomicInteger anyOverlap = new AtomicInteger(0);
    private final boolean log;

    public SectionIdConsumer(boolean log) {
        this.log = log;
    }

    @Override
    public void accept(String s) {
        if(log) { System.out.println(s); }
        String[] ranges = s.split(",");
        String[] s1Range = ranges[0].split("-");
        String[] s2Range = ranges[1].split("-");
        int[] r1 = { Integer.parseInt(s1Range[0]), Integer.parseInt(s1Range[1]) };
        int[] r2 = { Integer.parseInt(s2Range[0]), Integer.parseInt(s2Range[1]) };

        if((r2[0] >= r1[0] && r2[1] <= r1[1]) || (r1[0] >= r2[0] && r1[1] <= r2[1]))
            containedRanges.incrementAndGet();

        if((r2[0] >= r1[0] && r2[0] <= r1[1]) || (r1[0] >= r2[0] && r1[0] <= r2[1]))
            anyOverlap.incrementAndGet();
    }

    public int getContainedRanges() {
        return this.containedRanges.get();
    }

    public int getAnyOverlap() {
        return this.anyOverlap.get();
    }
}

package com.aoc;

import java.io.File;
import java.math.BigInteger;
import java.util.ArrayList;
import java.util.HashSet;
import java.util.List;
import java.util.Set;
import java.util.TreeSet;
import java.util.function.Consumer;

public class Dec15 {

    public static void main(String[] args) {
        File file = new File("input.txt");
        long duration = AOCUtils.timeTask(() -> {
            SensorConsumer sensorConsumer = new SensorConsumer();
            try { AOCUtils.readFileByLine(file, sensorConsumer); }
            catch(Exception e) { throw new RuntimeException(e); }

            System.out.printf("Part 1: %d\n", sensorConsumer.part1(2_000_000));
//            System.out.printf("Part 1: %d\n", sensorConsumer.part1(10));
            try {
//                Coord coord = sensorConsumer.part2(0, 20);
                Coord coord = sensorConsumer.part2(0, 4_000_000);

                // Number is potentially millions * millions which is > 64 bit int, need BigInteger
                BigInteger val = BigInteger.valueOf(coord.getX()).multiply(BigInteger.valueOf(4_000_000))
                        .add(BigInteger.valueOf(coord.getY()));
                System.out.printf("Part 2: %d\n", val);
            } catch(Exception e) { throw new RuntimeException(e); }
        });

        AOCUtils.printTimeBlock(duration);
    }
}

class SensorConsumer implements Consumer<String> {

    private final List<Sensor> sensorList = new ArrayList<>();

    @Override
    public void accept(String s) {
        String[] contents = s.split(" ");
        int x = Integer.parseInt(contents[2].split("=")[1].split(",")[0]);
        int y = Integer.parseInt(contents[3].split("=")[1].split(":")[0]);
        int beaconX = Integer.parseInt(contents[8].split("=")[1].split(",")[0]);
        int beaconY = Integer.parseInt(contents[9].split("=")[1]);

        sensorList.add(new Sensor()
                .setX(x)
                .setY(y)
                .setBeaconX(beaconX)
                .setBeaconY(beaconY));
    }

    public int part1(int targetY) {
        Set<Integer> uniqueCoords = new HashSet<>();
        for(Sensor sensor : sensorList) {
            int yDelta = Math.abs(sensor.getY() - sensor.getBeaconY());
            int xDelta = Math.abs(sensor.getX() - sensor.getBeaconX());
            int manSum = yDelta + xDelta;

            if(targetY <= sensor.getY()+manSum && targetY >= sensor.getY()-manSum) {
                int targetDelta = Math.abs(sensor.getY() - targetY);
                int yCalc = (manSum - targetDelta);
                for(int i = sensor.getX()-yCalc; i <= sensor.getX()+yCalc; i++) {
                    uniqueCoords.add(i);
                }
            }
        }

        return uniqueCoords.size()-1;
    }

    public Coord part2(int min, int max) {
        for (int targetY = min; targetY < max; targetY++) {
            System.out.printf("Starting targetY=%d\n", targetY);
            TreeSet<Range> uniqueCoords = new TreeSet<>();
            for (Sensor sensor : sensorList) {
                int yDelta = Math.abs(sensor.getY() - sensor.getBeaconY());
                int xDelta = Math.abs(sensor.getX() - sensor.getBeaconX());
                int manSum = yDelta + xDelta;

                // no need to calc everything, grab only the outline of the diamond
                if (targetY <= sensor.getY() + manSum && targetY >= sensor.getY() - manSum) {
                    int targetDelta = Math.abs(sensor.getY() - targetY);
                    int yCalc = (manSum - targetDelta);
                    uniqueCoords.add(new Range(new Coord(sensor.getX() - yCalc, targetY), new Coord(sensor.getX() + yCalc, targetY)));
                }
            }

            // check if the previous end is more than 1 higher than current start
            int end = Integer.MIN_VALUE;
            for (Range range : uniqueCoords) {
                if (end == Integer.MIN_VALUE) {
                    end = range.getEnd().getX();
                } else {
                    // handle a 2nd range being inside the first range
                    if (end > range.getEnd().getX()) {
                        continue;
                    }

                    // if the start is more than 1 over the previous end, then this is the row
                    if (range.getStart().getX() > end + 1) {
                        System.out.printf("FOUND NON EQUAL ROW: %d\n", targetY);
                        // missing 'x' value is 1 minus the current start
                        return new Coord(range.getStart().getX() - 1, targetY);
                    } else {
                        // if range is consecutive, update the end
                        end = range.getEnd().getX();
                    }
                }
            }
        }

        return null;
    }
}

class Sensor {
    private int x;
    private int y;
    private int beaconX;
    private int beaconY;

    public Sensor() {}

    public int getX() {
        return x;
    }

    public Sensor setX(int x) {
        this.x = x;
        return this;
    }

    public int getY() {
        return y;
    }

    public Sensor setY(int y) {
        this.y = y;
        return this;
    }

    public int getBeaconX() {
        return beaconX;
    }

    public Sensor setBeaconX(int beaconX) {
        this.beaconX = beaconX;
        return this;
    }

    public int getBeaconY() {
        return beaconY;
    }

    public Sensor setBeaconY(int beaconY) {
        this.beaconY = beaconY;
        return this;
    }
}
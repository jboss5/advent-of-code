package com.aoc;

import java.io.File;
import java.util.ArrayDeque;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.function.Consumer;

public class Dec12 {

    public static void main(String[] args) {
        File file = new File("input.txt");
        long duration = AOCUtils.timeTask(() -> {
            HillClimbingConsumer hillClimbingConsumer = new HillClimbingConsumer();
            try { AOCUtils.readFileByByte(file, hillClimbingConsumer); }
            catch(Exception e) { throw new RuntimeException(e); }

            System.out.println(hillClimbingConsumer);
//            System.out.printf("Part1: %d\n", hillClimbingConsumer.part1(new Coord(0,0)));
            System.out.printf("Part1: %d\n", hillClimbingConsumer.part1(new Coord(20,0)));

            List<Integer> output = new ArrayList<>();
            hillClimbingConsumer.getAllStartsP2()
                    .forEach(coord -> output.add(hillClimbingConsumer.part1(coord)));
            System.out.printf("Part2: %d\n", output.stream().sorted().findFirst().get());
        });

        AOCUtils.printTimeBlock(duration);
    }

}

class HillClimbingConsumer implements Consumer<Byte> {

    private static final char START = 'S';
    private static final char END = 'E';
    private static final char NEWLINE = '\n';
    private static final char LINEFEED = '\r';

    private List<List<Character>> heightMap = new ArrayList<>();
    private int line = 0;

    public HillClimbingConsumer() {
        heightMap.add(new ArrayList<>(1));
    }

    @Override
    public void accept(Byte aByte) {
        char readChar = (char)aByte.byteValue();
        if(heightMap.size() <= line) { heightMap.add(new ArrayList<>()); }

        if(readChar == NEWLINE) {
            line++;
        } else if(readChar != LINEFEED) {
            heightMap.get(line).add(readChar);
        }
    }

    public List<Coord> getAllStartsP2() {
        List<Coord> output = new ArrayList<>();
        for(int i = 0; i < heightMap.size(); i++) {
            for(int j = 0; j < heightMap.get(i).size(); j++) {
                if(heightMap.get(i).get(j) == 'a') {
                    output.add(new Coord(i,j));
                }
            }
        }

        return output;
    }

    public int part1(Coord startCoord) {
        Coord endCoord = null;
        for(int i = 0; i < heightMap.size(); i++) {
            for(int j = 0; j < heightMap.get(i).size(); j++) {
                if(heightMap.get(i).get(j) == END) {
                    endCoord = new Coord(i,j);
                }
            }
        }

        ArrayDeque<Coord> queue = new ArrayDeque<>();
        queue.add(startCoord);

        Map<Coord, Coord> path = new HashMap<>();
        Map<Coord, Integer> costMap = new HashMap<>();
        path.put(startCoord, null);
        costMap.put(startCoord, 0);

        while(!queue.isEmpty()) {
            Coord current = queue.poll();
            if(current.equals(endCoord)) { break; }

            for(Coord coord : getNeighbors(current)) {
                char toChar = heightMap.get(coord.getX()).get(coord.getY());
                char currentChar = heightMap.get(current.getX()).get(current.getY());
                int cost = costMap.get(current)+1;

                if(!costMap.containsKey(coord) || cost < costMap.get(coord)) {
                    if(currentChar == START || toChar <= currentChar+1) {
                        costMap.put(coord, cost);
                        queue.add(coord);
                        path.put(coord, current);
                    }
                }
            }
        }

        int out = (costMap.get(endCoord) == null) ? Integer.MAX_VALUE : costMap.get(endCoord);

        // code only prints out the solved paths, if it hits a path that didn't finish just return
        Coord current = endCoord;
        List<Coord> output = new ArrayList<>();
        while(current != startCoord) {
            if(current == null) { return out; }
            output.add(current);
            current = path.get(current);
        }

        output.add(startCoord);
        System.out.println();

        for(int i = 0; i < heightMap.size(); i++) {
            for(int j = 0; j < heightMap.get(i).size(); j++) {
                if(output.contains(new Coord(i,j))) {
                    System.out.printf("%s","X");
                } else {
                    System.out.printf("%s","-");
                }
            }

            System.out.println();
        }

        return out;
    }

    private List<Coord> getNeighbors(Coord start) {
        List<Coord> outputCoords = new ArrayList<>();

        if(start.getX() > 0) {
            outputCoords.add(new Coord(start.getX()-1, start.getY()));
        }

        if(start.getX() < heightMap.size()-1) {
            outputCoords.add(new Coord(start.getX()+1, start.getY()));
        }

        if(start.getY() > 0) {
            outputCoords.add(new Coord(start.getX(), start.getY()-1));
        }

        if(start.getY() < heightMap.get(start.getX()).size()-1) {
            outputCoords.add(new Coord(start.getX(), start.getY()+1));
        }

        return outputCoords;
    }

    @Override
    public String toString() {
        StringBuilder str = new StringBuilder();
        for(List<Character> list : heightMap) {
            for(Character ch : list) {
                str.append(ch);
            }

            str.append(NEWLINE);
        }

        return str.toString();
    }
}

class Coord {
    private int x;
    private int y;

    public Coord(int x, int y) {
        this.x = x;
        this.y = y;
    }

    public int getX() {
        return x;
    }

    public void setX(int x) {
        this.x = x;
    }

    public int getY() {
        return y;
    }

    public void setY(int y) {
        this.y = y;
    }

    @Override
    public String toString() {
        return "("+x+","+y+")";
    }

    @Override
    public boolean equals(Object obj) {
        if(obj instanceof Coord) {
            return (obj == this) || ((Coord)obj).getX() == x && ((Coord) obj).getY() == y;
        }

        return false;
    }

    @Override
    public int hashCode() {
        return toString().hashCode();
    }
}


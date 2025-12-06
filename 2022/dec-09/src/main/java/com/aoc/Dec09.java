package com.aoc;

import java.io.File;
import java.util.ArrayList;
import java.util.HashSet;
import java.util.List;
import java.util.Set;
import java.util.function.Consumer;

public class Dec09 {

    public static void main(String[] args) {
        File file = new File("input.txt");
        long duration = AOCUtils.timeTask(() -> {
            RopeConsumer ropeConsumer = new RopeConsumer();
            try { AOCUtils.readFileByLine(file, ropeConsumer); }
            catch(Exception e) { throw new RuntimeException(e); }
            System.out.printf("Part1: %d\n", ropeConsumer.part1());

            RopeConsumerP2 ropeConsumerP2 = new RopeConsumerP2();
            try { AOCUtils.readFileByLine(file, ropeConsumerP2); }
            catch(Exception e) { throw new RuntimeException(e); }
            System.out.printf("Part2: %d\n", ropeConsumerP2.part2());
        });

        AOCUtils.printTimeBlock(duration);
    }
}

class RopeConsumer implements Consumer<String> {

    private Coord coord = new Coord(0,0);
    private List<Coord> movements = new ArrayList<>();
    private Set<String> tailCoords = new HashSet<>();

    public RopeConsumer() {
        movements.add(coord);
        tailCoords.add(coord.toString());
    }

    @Override
    public void accept(String s) {
        String[] temp = s.split(" ");
        switch(temp[0]) {
            case "R":
                moveEast(movements.get(movements.size()-1), Integer.parseInt(temp[1]));
                break;
            case "U":
                moveNorth(movements.get(movements.size()-1), Integer.parseInt(temp[1]));
                break;
            case "D":
                moveSouth(movements.get(movements.size()-1), Integer.parseInt(temp[1]));
                break;
            case "L":
                moveWest(movements.get(movements.size()-1), Integer.parseInt(temp[1]));
                break;
        }
    }

    private void moveEast(Coord startCoord, int movement) {
        for(int i = 1; i <= movement; i++) {
            movements.add(new Coord(startCoord.getX(), startCoord.getY()+i));
        }
    }

    private void moveWest(Coord startCoord, int movement) {
        for(int i = 1; i <= movement; i++) {
            movements.add(new Coord(startCoord.getX(), startCoord.getY()-i));
        }
    }

    private void moveNorth(Coord startCoord, int movement) {
        for(int i = 1; i <= movement; i++) {
            movements.add(new Coord(startCoord.getX()+i, startCoord.getY()));
        }
    }

    private void moveSouth(Coord startCoord, int movement) {
        for(int i = 1; i <= movement; i++) {
            movements.add(new Coord(startCoord.getX()-i, startCoord.getY()));
        }
    }

    public int part1() {
        Coord tail = new Coord(0,0);
        for(Coord movement : movements) {
            if(!tail.equals(movement)) {
                Coord temp = Coord.findMove(tail, movement);
                if(temp != null) {
                    tailCoords.add(temp.toString());
                    tail = temp;
                }
            }
        }

        return tailCoords.size();
    }

    public void print() {
        for(Coord movement : movements) {
            System.out.printf("(%d,%d)\n", movement.getX(), movement.getY());
        }
    }
}

class RopeConsumerP2 implements Consumer<String> {

    private Coord head = new Coord(0,0);
    private Coord[] movements = new Coord[9];
    private Set<String> tailCoords = new HashSet<>();

    public RopeConsumerP2() {
        for(int i = 0; i < 9; i++) {
            movements[i] = new Coord(0,0);
        }
        tailCoords.add(head.toString());
    }

    @Override
    public void accept(String s) {
        String[] temp = s.split(" ");
        switch(temp[0]) {
            case "R":
                moveEast(head, Integer.parseInt(temp[1]));
                break;
            case "U":
                moveNorth(head, Integer.parseInt(temp[1]));
                break;
            case "D":
                moveSouth(head, Integer.parseInt(temp[1]));
                break;
            case "L":
                moveWest(head, Integer.parseInt(temp[1]));
                break;
        }
    }

    private void moveEast(Coord startCoord, int movement) {
        int start = startCoord.getY();
        for(int i = 1; i <= movement; i++) {
            head.setY(start+i);
            evaluateTail();
        }
    }

    private void moveWest(Coord startCoord, int movement) {
        int start = startCoord.getY();
        for(int i = 1; i <= movement; i++) {
            head.setY(start-i);
            evaluateTail();
        }
    }

    private void moveNorth(Coord startCoord, int movement) {
        int start = startCoord.getX();
        for(int i = 1; i <= movement; i++) {
            head.setX(start+i);
            evaluateTail();
        }
    }

    private void moveSouth(Coord startCoord, int movement) {
        int start = startCoord.getX();
        for(int i = 1; i <= movement; i++) {
            head.setX(start-i);
            evaluateTail();
        }
    }

    private void evaluateTail() {
        Coord start = new Coord(head.getX(), head.getY());
        for(int i = 0; i < movements.length; i++) {
            Coord temp = Coord.findMove(movements[i], start);
            if(temp != null) {
                if(i == 8) tailCoords.add(temp.toString());
                movements[i].setX(temp.getX()).setY(temp.getY());
            }

            start = movements[i];
        }
    }

    public int part2() {
        return tailCoords.size();
    }
}

class Coord {
    private int x = 0;
    private int y = 0;

    public Coord(int x, int y) {
        this.x = x;
        this.y = y;
    }

    public int getX() {
        return x;
    }

    public int getY() {
        return y;
    }

    public Coord setX(int x) {
        this.x = x;
        return this;
    }

    public Coord setY(int y) {
        this.y = y;
        return this;
    }

    @Override
    public boolean equals(Object obj) {
        return (obj instanceof Coord) && ((Coord)obj).getX() == x && ((Coord)obj).getY() == y;
    }

    @Override
    public String toString() {
        return String.format("(%d,%d)", x, y);
    }

    public static Coord findMove(Coord tail, Coord movement) {
        int deltaX = tail.getX() - movement.getX();
        int deltaY = tail.getY() - movement.getY();
        int absX = Math.abs(deltaX);
        int absY = Math.abs(deltaY);

        if(absX == 2 && absY == 0) {
            if(deltaX < 0) {
                return new Coord(tail.getX()+1, tail.getY());
            } else {
                return new Coord(tail.getX()-1, tail.getY());
            }
        }

        if(absY == 2 && absX == 0) {
            if(deltaY < 0) {
                return new Coord(tail.getX(), tail.getY()+1);
            } else {
                return new Coord(tail.getX(), tail.getY()-1);
            }
        }

        if(absX == 2 || absY == 2) {
            if(deltaX < 0 && deltaY > 0) {
                return new Coord(tail.getX()+1,tail.getY()-1);
            } else if(deltaX < 0 && deltaY < 0) {
                return new Coord(tail.getX()+1,tail.getY()+1);
            } else if(deltaX > 0 && deltaY < 0) {
                return new Coord(tail.getX()-1,tail.getY()+1);
            } else if(deltaX > 0 && deltaY > 0) {
                return new Coord(tail.getX()-1,tail.getY()-1);
            }
        }

        return null;
    }
}
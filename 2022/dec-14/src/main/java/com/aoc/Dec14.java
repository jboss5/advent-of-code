package com.aoc;

import java.io.File;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
import java.util.function.Consumer;

public class Dec14 {

    public static void main(String[] args) {
        File file = new File("input.txt");
        long duration = AOCUtils.timeTask(() -> {
            SandSimluationConsumer sandSimluationConsumer = new SandSimluationConsumer();
            try { AOCUtils.readFileByLine(file, sandSimluationConsumer); }
            catch(Exception e) { throw new RuntimeException(e); }

            SandSimulator sandSimulator = new SandSimulator(sandSimluationConsumer);
            System.out.printf("\nPart 1: %d\n", sandSimulator.part1());
            System.out.printf("\nPart 2: %d\n", sandSimulator.part2(500));
        });

        AOCUtils.printTimeBlock(duration);
    }
}

class SandSimluationConsumer implements Consumer<String> {

    private int maxX = -1;
    private int maxY = -1;
    private int minX = Integer.MAX_VALUE;
    private int minY = Integer.MAX_VALUE;
    private List<String> operations = new ArrayList<>();

    public SandSimluationConsumer() {}

    @Override
    public void accept(String s) {
        operations.add(s);
        String[] temp = s.split(" -> ");
        for(String t : temp) {
            t = t.trim();
            String[] c = t.split(",");
            int x = Integer.parseInt(c[0]);
            int y = Integer.parseInt(c[1]);

            // get max & mins to use in calcs later without needing 500+ sized columns for base
            maxX = Math.max(x, maxX);
            minX = Math.min(x, minX);
            maxY = Math.max(y, maxY);
            minY = Math.min(y, minY);
        }
    }

    public int getMaxX() {
        return maxX;
    }

    public int getMaxY() {
        return maxY;
    }

    public int getMinX() {
        return minX;
    }

    public List<String> getOperations() {
        return operations;
    }
}

class SandSimulator {

    private char[][] masterGrid;
    private SandSimluationConsumer data;
    private int minX;

    public SandSimulator(SandSimluationConsumer builder) {
        this.minX = builder.getMinX();
        this.masterGrid = new char[builder.getMaxY()+1][(builder.getMaxX() - builder.getMinX())+1];
        this.data = builder;

        for (char[] chars : masterGrid) {
            Arrays.fill(chars, '.');
        }
    }

    public int part1() {
        char[][] grid = cloneMaster();
        buildGrid(grid);
        return simulate(grid, 0);
    }

    public int part2(int size) {
        char[][] grid = cloneMaster();
        buildGrid(grid);

        // resize by adding 'size' elements to left and right of original grid
        // hacky way to simulate infinite sides
        char[][] bigGrid = new char[grid.length+2][grid[0].length+(size*2)];
        for(char[] chars : bigGrid) {
            Arrays.fill(chars, '.');
        }

        for(int i = 0; i < grid.length; i++) {
            System.arraycopy(grid[i], 0, bigGrid[i], size, grid[i].length);
        }

        // add new "bottom" rows
        Arrays.fill(bigGrid[bigGrid.length-2], '.');
        Arrays.fill(bigGrid[bigGrid.length-1], '#');

        return simulate(bigGrid, size);
    }

    private char[][] cloneMaster() {
        char[][] grid = new char[masterGrid.length][masterGrid[0].length];
        for(int i = 0; i < masterGrid.length; i++) {
            System.arraycopy(masterGrid[i], 0, grid[i], 0, masterGrid[i].length);
        }

        return grid;
    }

    private void buildGrid(char[][] grid) {
        for(String op : data.getOperations()) {
            String[] temp = op.split(" -> ");

            String[] start = temp[0].split(",");
            int startX = Integer.parseInt(start[0]);
            int startY = Integer.parseInt(start[1]);
            for(int i = 1; i < temp.length; i++) {
                String[] c = temp[i].split(",");
                int x = Integer.parseInt(c[0]);
                int y = Integer.parseInt(c[1]);

                if(x != startX) {
                    if(startX > x) {
                        for (int j = (x-minX); j <= (startX-minX); j++) {
                            grid[y][j] = '#';
                        }
                    } else {
                        for (int j = (x-minX); j >= (startX-minX); j--) {
                            grid[y][j] = '#';
                        }
                    }
                } else if(y != startY) {
                    if(startY > y) {
                        for(int j = startY; j >= y; j--) {
                            grid[j][x-minX] = '#';
                        }
                    } else {
                        for(int j = startY; j <= y; j++) {
                            grid[j][x-minX] = '#';
                        }
                    }
                }

                startX = x;
                startY = y;
            }
        }
    }

    private int simulate(char[][] grid, int offset) {
        // always start at "500", but in this case subtract minX from input to get the 0-based index
        // add offset for potential p2 support
        int x = (500-minX)+offset;
        int y = 0;
        int count = 0;
        while(y < grid.length || x > 0 || x < grid[0].length) {
            if(y == grid.length-1) {
                break;
            }

            // check below
            char ch = grid[y+1][x];
            if(ch == '.') {
                y++;
            } else if(ch == '#' || ch == 'o') {
                if(x == 0) { break; }

                // check left diagonal
                ch = grid[y+1][x-1];
                if(ch == '.') {
                    y++;
                    x--;
                } else {

                    // check right diagonal
                    ch = grid[y+1][x+1];
                    if(ch == '.') {
                        y++;
                        x++;
                    } else {
                        // check for top
                        if(grid[y][x] == 'o') { break; }

                        // if not top add the sand
                        grid[y][x] = 'o';
                        count++;
                        y = 0;
                        x = (500-minX)+offset;
                    }
                }
            }
        }

        // print it for viz
        for (char[] chars : grid) {
            for (char ch : chars) {
                System.out.printf("%s", ch);
            }
            System.out.println();
        }

        return count;
    }
}

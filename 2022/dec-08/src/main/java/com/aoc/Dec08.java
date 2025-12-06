package com.aoc;

import java.io.File;
import java.util.ArrayList;
import java.util.Comparator;
import java.util.List;
import java.util.concurrent.ExecutionException;
import java.util.concurrent.ExecutorService;
import java.util.concurrent.Executors;
import java.util.concurrent.Future;
import java.util.function.Consumer;

public class Dec08 {

    public static void main(String[] args) {
        long duration = AOCUtils.timeTask(() -> {
            File file = new File("input.txt");
            HeightMapConsumer heightMapConsumer = new HeightMapConsumer();
            try {
                AOCUtils.readFileByLine(file, heightMapConsumer);
                Answer answer = heightMapConsumer.compute();
                System.out.printf("Part1: %d\n", answer.getTotalVisible());
                System.out.printf("Part2: %d\n", answer.getHighestScenicScore());
            } catch(Exception e) {
                throw new RuntimeException(e);
            }
        });

        System.out.println();
        AOCUtils.printTimeBlock(duration);
    }
}

class HeightMapConsumer implements Consumer<String> {

    private List<List<Integer>> heightMap = new ArrayList<>();
    private int row = 0;

    @Override
    public void accept(String s) {
        for(int i = 0; i < s.length(); i++) {
            addHeight(Character.getNumericValue(s.charAt(i)), row, i);
        }

        row++;
    }

    private void addHeight(int height, int row, int column) {
        if(heightMap.size() <= row) { heightMap.add(new ArrayList<>()); }
        heightMap.get(row).add(column, height);
    }

    public int getValue(int r, int c) {
        return heightMap.get(r).get(c);
    }

    public int numRows() {
        return heightMap.size();
    }

    public int numCols() {
        return heightMap.get(0).size();
    }

    public Answer compute() throws InterruptedException, ExecutionException {
        List<Integer> viewDistances = new ArrayList<>();
        int numVisible = 0;
        for(int r = 1; r < heightMap.size()-1; r++) {
            for(int c = 1; c < heightMap.get(r).size()-1; c++) {
                boolean isVisible = false;
                ExecutorService directionSearchPool = Executors.newFixedThreadPool(4);
                List<Future<ScenicScore>> searches = new ArrayList<>(4);

                searches.add(directionSearchPool.submit(new SearchThread(SearchThread.Type.NORTH, this, r, c)));
                searches.add(directionSearchPool.submit(new SearchThread(SearchThread.Type.EAST, this, r, c)));
                searches.add(directionSearchPool.submit(new SearchThread(SearchThread.Type.WEST, this, r, c)));
                searches.add(directionSearchPool.submit(new SearchThread(SearchThread.Type.SOUTH, this, r, c)));

                List<Integer> viewLines = new ArrayList<>(4);
                for(Future<ScenicScore> result : searches) {
                    ScenicScore score = result.get();
                     if(score.isVisible())
                         isVisible = true;

                     viewLines.add(score.getDistance());
                }

                directionSearchPool.shutdownNow();
                if(isVisible)
                    numVisible++;

                // https://docs.oracle.com/javase/tutorial/collections/streams/reduction.html#reduce
                viewDistances.add(viewLines
                        .stream()
                        .reduce(1, (a, b) -> a * b));
            }
        }

        viewDistances.sort(Comparator.reverseOrder());
        return new Answer(numVisible + (numRows() * 2) + ((numCols() - 2) * 2), viewDistances.get(0));
    }
}

class Answer {
    private int totalVisible = 0;
    private int highestScenicScore = 0;

    public Answer(int totalVisible, int highestScenicScore) {
        this.totalVisible = totalVisible;
        this.highestScenicScore = highestScenicScore;
    }

    public int getTotalVisible() {
        return totalVisible;
    }

    public int getHighestScenicScore() {
        return highestScenicScore;
    }
}

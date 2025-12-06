package com.aoc;

import java.util.concurrent.Callable;

public class SearchThread implements Callable<ScenicScore> {

    public enum Type {
        NORTH,
        SOUTH,
        EAST,
        WEST
    }

    private Type searchType;
    private HeightMapConsumer data;
    private int startRow;
    private int startCol;

    public SearchThread(Type searchType, HeightMapConsumer data, int startRow, int startCol) {
        this.searchType = searchType;
        this.data = data;
        this.startCol = startCol;
        this.startRow = startRow;
    }

    @Override
    public ScenicScore call() {
        int current = data.getValue(startRow, startCol);
        int r;
        int c;
        switch(searchType) {
            case NORTH:
                for(r = startRow-1; r >= 0; r--) {
                    if(data.getValue(r, startCol) >= current)
                        return new ScenicScore(false, startRow-r);
                }

                return new ScenicScore(true, startRow-(r+1));
            case SOUTH:
                for(r = startRow+1; r < data.numRows(); r++) {
                    if(data.getValue(r, startCol) >= current)
                        return new ScenicScore(false, r-startRow);
                }

                return new ScenicScore(true, (r-1)-startRow);
            case WEST:
                for(c = startCol-1; c >= 0; c--) {
                    if(data.getValue(startRow, c) >= current)
                        return new ScenicScore(false, startCol-c);
                }

                return new ScenicScore(true, startCol-(c+1));
            case EAST:
                for(c = startCol+1; c < data.numCols(); c++) {
                    if(data.getValue(startRow, c) >= current)
                        return new ScenicScore(false, c-startCol);
                }

                return new ScenicScore(true, (c-1)-startCol);
        }

        // impossible to reach
        return null;
    }
}

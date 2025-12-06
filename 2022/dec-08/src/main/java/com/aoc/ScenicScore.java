package com.aoc;

public class ScenicScore {
    private boolean isVisible = false;
    private int distance = 0;

    public ScenicScore(boolean isVisible, int distance) {
        this.isVisible = isVisible;
        this.distance = distance;
    }

    public boolean isVisible() {
        return isVisible;
    }

    public int getDistance() {
        return distance;
    }
}

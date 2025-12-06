package com.aoc;

class Range implements Comparable<Range> {
    private Coord start;
    private Coord end;

    public Range(Coord start, Coord end) {
        this.start = start;
        this.end = end;
    }

    public Coord getStart() {
        return start;
    }

    public Coord getEnd() {
        return end;
    }

    @Override
    public int hashCode() {
        return toString().hashCode();
    }

    @Override
    public boolean equals(Object obj) {
        if(obj instanceof Range) {
            return (obj == this) || (start.equals(((Range)obj).getStart()) && end.equals(((Range)obj).getEnd()));
        }

        return false;
    }

    @Override
    public String toString() {
        return "("+start.getX()+","+start.getY()+")->("+end.getX()+","+end.getY()+")";
    }

    @Override
    public int compareTo(Range o) {
        if(start.getX() == o.getStart().getX()) {
            return end.getX() - o.getEnd().getX();
        }

        return start.getX() - o.getStart().getX();
    }
}

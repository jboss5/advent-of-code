package com.aoc;

import java.io.File;

public class AOCInput {

    private File file;
    private Object p1Answer;
    private Object p2Answer;

    public AOCInput(File file, Object p1Answer, Object p2Answer) {
        this.file = file;
        this.p1Answer = p1Answer;
        this.p2Answer = p2Answer;
    }

    public File getFile() {
        return file;
    }

    public Object getP1Answer() {
        return p1Answer;
    }

    public Object getP2Answer() {
        return p2Answer;
    }

    public boolean doesP1Match(Object obj) {
        return (obj != null) && obj.equals(p1Answer);
    }

    public boolean doesP2Match(Object obj) {
        return (obj != null) && obj.equals(p2Answer);
    }
}

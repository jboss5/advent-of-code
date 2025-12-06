package com.aoc;

import java.util.HashMap;
import java.util.Map;
import java.util.concurrent.atomic.AtomicLong;

public class FileSystemTree {

    private FileSystemTree parent = null;
    private Map<String, FileSystemTree> subDirectories = new HashMap<>();
    private Map<String, Integer> containedFileList = new HashMap<>();
    private String fsName;

    public FileSystemTree(String fsName) {
        this.fsName = fsName;
    }

    public FileSystemTree(String fsName, FileSystemTree parent) {
        this.fsName = fsName;
        this.parent = parent;
    }

    public FileSystemTree getParent() {
        return parent;
    }

    public Map<String, FileSystemTree> getSubDirectories() {
        return subDirectories;
    }

    public Map<String, Integer> getContainedFileList() {
        return containedFileList;
    }

    public String getName() {
        return fsName;
    }

    public void addSubdirectory(FileSystemTree dir) {
        subDirectories.put(dir.getName(), dir);
    }

    public void addFileMetadata(String filename, int size) {
        containedFileList.put(filename, size);
    }

    private long getTotalFileSize() {
        return containedFileList.values()
                .stream()
                .mapToInt(Integer::intValue)
                .sum();
    }

    public long size() {
        if(subDirectories.isEmpty()) {
            return getTotalFileSize();
        } else {
            AtomicLong t = new AtomicLong(0L);
            subDirectories.values().forEach(v -> t.addAndGet(v.size()));
            return getTotalFileSize() + t.get();
        }
    }
}

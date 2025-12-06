package com.aoc;

import java.io.File;
import java.util.ArrayList;
import java.util.List;
import java.util.function.Consumer;

public class Dec07 {

    public static void main(String[] args) {
        File file = new File("input.txt");
        long duration = AOCUtils.timeTask(() -> {
            FileSystemConsumer fileSystemConsumer = new FileSystemConsumer();
            try { AOCUtils.readFileByLine(file, fileSystemConsumer); }
            catch(Exception e) { throw new RuntimeException(e); }

            fileSystemConsumer.print();
            System.out.printf("\n\npart1: %d", fileSystemConsumer.part1());
            System.out.printf("\npart2: %d\n", fileSystemConsumer.part2());
        });

        AOCUtils.printTimeBlock(duration);
    }
}

class FileSystemConsumer implements Consumer<String> {

    private FileSystemTree root = new FileSystemTree("/");
    private FileSystemTree current = root;

    @Override
    public void accept(String s) {
        if(s.charAt(0) == '$') {
            processCommand(s);
        } else {
            String[] tokens = s.split(" ");
            if(tokens[0].equals("dir")) {
                FileSystemTree fs = new FileSystemTree(tokens[1], current);
                current.addSubdirectory(fs);
            } else {
                current.addFileMetadata(tokens[1], Integer.parseInt(tokens[0]));
            }
        }
    }

    private void processCommand(String cmd) {
        String[] tokens = cmd.split(" ");

        // "cd" is only handled command, "ls" can be ignored
        if(tokens[1].equals("cd")) {
            switch(tokens[2]) {
                case "..":
                    // Only move to parent if parent exists, if at root stay at root
                    current = (current.getParent() == null) ? current : current.getParent();
                    break;
                case "/":
                    // ignored, no instructions to move to root
                    break;
                default:
                    current = current.getSubDirectories().get(tokens[2]);
            }
        }
    }

    private void getTotalSize(FileSystemTree fs, List<Long> sums) {
        sums.add(fs.size());
        fs.getSubDirectories().values().forEach(v -> getTotalSize(v, sums));
    }

    private List<Long> getDirectorySizes(FileSystemTree fs) {
        List<Long> sums = new ArrayList<>();
        getTotalSize(fs, sums);
        return sums;
    }

    public long part1() {
        return getDirectorySizes(root)
                .stream()
                .mapToLong(Long::longValue)
                .filter(v -> v <= 100_000)
                .sum();
    }

    public long part2() {
        long requiredSpace = 30_000_000 - (70_000_000 - root.size());
        return getDirectorySizes(root)
                .stream()
                .filter(v -> v > requiredSpace)
                .sorted()
                .findFirst()
                .get();
    }

    /* Debugging methods for printing tree */
    private String addPadding(int size) {
        return String.format("%" + size + "s", " ");
    }

    private void printFileSystemTree(FileSystemTree fileSystemTree, int padding) {
        System.out.printf("%sd - %s\n", addPadding(padding), fileSystemTree.getName());
        fileSystemTree.getSubDirectories().values().forEach(v -> printFileSystemTree(v, padding+1));
        fileSystemTree.getContainedFileList().forEach((key, value) -> System.out.printf("%sf - %s | %d\n", addPadding(padding+2), key, value));
    }

    public void print() {
        printFileSystemTree(root, 1);
    }
}

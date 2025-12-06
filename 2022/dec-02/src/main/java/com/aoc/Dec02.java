package com.aoc;

import java.io.BufferedReader;
import java.io.File;
import java.io.FileReader;
import java.util.Scanner;

public class Dec02 {

    public static void main(String[] args) {
        System.out.print("Input file: ");
        Scanner scanner = new Scanner(System.in);
        File file = new File(scanner.nextLine());

        if(!file.exists() || !file.canRead()) {
            System.err.println("Cannot find file " + file.getAbsolutePath());
            System.exit(1);
        } else {
            new Dec02(file).runit();
        }
    }

    private static final String OUT_ROCK = "X";
    private static final String OUT_PAPER = "Y";
    private static final String OUT_SCISSORS = "Z";
    private static final String IN_ROCK = "A";
    private static final String IN_PAPER = "B";
    private static final String INT_SCISSORS = "C";
    private final File file;

    public Dec02(File file) {
        this.file = file;
    }

    public void runit() {
        try(BufferedReader reader = new BufferedReader(new FileReader(file))) {
            String line;
            int totalP1 = 0;
            int totalP2 = 0;
            while((line = reader.readLine()) != null) {
                String[] draw = line.split(" ");
                totalP1 += getScore(draw[0], draw[1]);
                totalP2 += getScore(draw[0], findHandFromOutcome(draw[0],draw[1]));
            }

            System.out.printf("P1 Score: %d\n", totalP1);
            System.out.printf("P2 Score: %d", totalP2);
        } catch(Exception e) {
            System.err.println("Error processing");
            e.printStackTrace();
        }
    }

    private int getScore(String opp, String resp) {
        return getHandScore(resp) + getGameScore(opp,resp);
    }

    private int getHandScore(String hand) {
        if(hand.equals(OUT_ROCK)) return 1;
        if(hand.equals(OUT_PAPER)) return 2;
        if(hand.equals(OUT_SCISSORS)) return 3;
        throw new RuntimeException("Found invalid hand: " + hand);
    }

    private int getGameScore(String opp, String resp) {
        switch(opp+resp) {
            case IN_ROCK + OUT_PAPER:
            case IN_PAPER + OUT_SCISSORS:
            case INT_SCISSORS + OUT_ROCK:
                return 6;
            case IN_ROCK + OUT_ROCK:
            case IN_PAPER + OUT_PAPER:
            case INT_SCISSORS + OUT_SCISSORS:
                return 3;
            default:
                return 0;
        }
    }

    private String findHandFromOutcome(String opp, String resp) {
        String temp = opp+resp;
        switch(temp) {
            case IN_ROCK + OUT_PAPER:
            case IN_PAPER + OUT_ROCK:
            case INT_SCISSORS + OUT_SCISSORS:
                return OUT_ROCK;
            case IN_PAPER + OUT_PAPER:
            case INT_SCISSORS + OUT_ROCK:
            case IN_ROCK + OUT_SCISSORS:
                return OUT_PAPER;
            case INT_SCISSORS + OUT_PAPER:
            case IN_ROCK + OUT_ROCK:
            case IN_PAPER + OUT_SCISSORS:
                return OUT_SCISSORS;
            default:
                throw new RuntimeException("Found invalid hand: " + temp);
        }
    }
}

package com.aoc;

import java.util.HashSet;
import java.util.Set;
import java.util.stream.Collectors;

public class Rucksack {

    private Set<Character> c1;
    private Set<Character> c2;
    private Set<Character> sharedItems;

    public Rucksack(String c1, String c2) {
        this.c1 = c1.chars()
                .mapToObj(c -> (char)c)
                .collect(Collectors.toSet());

        this.c2 = c2.chars()
                .mapToObj(c -> (char)c)
                .collect(Collectors.toSet());

        buildSharedItems();
    }

    public static int itemPriority(char c) {
        if(Character.isLowerCase(c)) return (c - 'a') + 1;
        else return (c - 'A') + 27;
    }

    private void buildSharedItems() {
        sharedItems = this.c1.stream()
                .filter(c -> this.c2.contains(c))
                .collect(Collectors.toSet());
    }

    public Set<Character> getSharedItems() {
        return this.sharedItems;
    }

    public Set<Character> getGroup() {
        HashSet<Character> union = new HashSet<>();
        union.addAll(c1);
        union.addAll(c2);
        return union;
    }
}

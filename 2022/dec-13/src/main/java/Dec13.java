import com.aoc.AOCUtils;
import com.google.gson.Gson;
import com.google.gson.GsonBuilder;

import java.io.File;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.Collections;
import java.util.List;
import java.util.function.Consumer;

public class Dec13 {

    public static void main(String[] args) {
        File file = new File("input.txt");
        long duration = AOCUtils.timeTask(() -> {
            SignalConsumer signalConsumer = new SignalConsumer();
            try { AOCUtils.readFileByLine(file, signalConsumer); }
            catch(Exception e) { throw new RuntimeException(e); }

            System.out.printf("Part 1: %d\n", signalConsumer.part1());
            System.out.printf("Part 2: %d\n", signalConsumer.part2());
        });

        AOCUtils.printTimeBlock(duration);
    }
}

class SignalConsumer implements Consumer<String> {

    private static final Gson GSON = new GsonBuilder().create();
    private List<List<?>> data = new ArrayList<>();

    @Override
    public void accept(String s) {
        if (s.isEmpty()) { return; }
        data.add(GSON.fromJson(s, List.class));
    }

    public int part1() {
        List<Integer> correctIndexes = new ArrayList<>();
        int pair = 1;
        ListComparator listComparator = new ListComparator();
        for(int i = 0; i < data.size(); i+=2) {
            Object lhs = data.get(i);
            Object rhs = data.get(i+1);

            if(listComparator.compare(lhs, rhs) < 0) {
                correctIndexes.add(pair);
            }

            pair++;
        }

        return correctIndexes.stream().mapToInt(i -> i).sum();
    }

    public int part2() {
        List<Double> firstDivider = Arrays.asList(2.0);
        List<Double> secondDivider = Arrays.asList(6.0);
        data.add(firstDivider);
        data.add(secondDivider);
        Collections.sort(data, new ListComparator());

        int index = 1;
        for(int i = 0; i < data.size(); i++) {
            Object obj = data.get(i);

            if(obj == firstDivider || obj == secondDivider) {
                index *= (i+1);
            }
        }

        return index;
    }
}
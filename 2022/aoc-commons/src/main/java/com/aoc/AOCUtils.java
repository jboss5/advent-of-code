package com.aoc;

import java.io.BufferedReader;
import java.io.File;
import java.io.FileReader;
import java.io.IOException;
import java.util.concurrent.TimeUnit;
import java.util.function.Consumer;

public class AOCUtils {

    private AOCUtils() {}

    /**
     * Read a file line by line, each line sent to a {@code Consumer} instance.
     *
     * @param file File to process line by line
     * @param consumer Consumer to send lines
     * @throws IOException Error processing file
     */
    public static void readFileByLine(File file, Consumer<String> consumer) throws IOException {
        try(BufferedReader reader = new BufferedReader(new FileReader(file))) {
            String line;
            while((line = reader.readLine()) != null) {
                consumer.accept(line);
            }
        }
    }

    /**
     * Read a file byte by byte, each byte sent to a {@code Consumer} instance.
     *
     * @param file File to process byte by byte
     * @param consumer Consumer to send bytes
     * @throws IOException Error processing file
     */
    public static void readFileByByte(File file, Consumer<Byte> consumer) throws IOException {
        try(BufferedReader reader = new BufferedReader(new FileReader(file))) {
            int b;
            while((b = reader.read()) >= 0) {
                consumer.accept((byte)b);
            }
        }
    }

    /**
     * Helper function to wrap a {@link Runnable} task and return the nanosecond duration.
     *
     * @param task Task to time
     * @return Nanosecond duration of the given task
     */
    public static long timeTask(Runnable task) {
        long nanoStart = System.nanoTime();
        task.run();
        return System.nanoTime() - nanoStart;
    }

    /**
     * Helper function to print out a given long into minutes, seconds, millis, micros and nanos.
     *
     * @param nanos Nanosecond duration to print out
     */
    public static void printTimeBlock(long nanos) {
        System.out.printf("\nTotal time:\n\t- Minutes:\t\t\t%d\n\t- Seconds:\t\t\t%d\n\t- Milliseconds:\t\t%d\n\t- Microseconds\t\t%d\n\t- Nanoseconds:\t\t%d\n",
                TimeUnit.MINUTES.convert(nanos, TimeUnit.NANOSECONDS),
                TimeUnit.SECONDS.convert(nanos, TimeUnit.NANOSECONDS),
                TimeUnit.MILLISECONDS.convert(nanos, TimeUnit.NANOSECONDS),
                TimeUnit.MICROSECONDS.convert(nanos, TimeUnit.NANOSECONDS),
                nanos);
    }
}

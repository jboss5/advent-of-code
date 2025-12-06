import java.io.File
import java.io.InputStream

fun main() {
    val inputStream: InputStream = File("input.txt").inputStream()
    var tick = 1
    var register = 1
    var signal = 0
    var pixel = 0

    inputStream.bufferedReader().forEachLine {
        val contents: List<String> = it.split(" ")
        pixel = handlePixel(pixel, tick, register)
        signal += handleSignal(tick, register)

        if(contents[0] == "noop") {
            tick += 1
        } else {
            pixel = handlePixel(pixel, tick+1, register)
            signal += handleSignal(tick+1, register)
            tick += 2
            register += Integer.parseInt(contents[1])
        }
    }

    println("Signal strength: $signal")
}

fun handlePixel(pixel: Int, tick: Int, register: Int): Int {
    if(pixel == register || pixel == register+1 || pixel == register-1) {
        print('#')
    } else {
        print('.')
    }

    if(tick != 0 && tick % 40 == 0) {
        print('\n')
        return 0
    }

    return pixel + 1
}

fun handleSignal(tick: Int, register: Int): Int {
    if(hashSetOf(20,60,100,140,180,220).contains(tick)) {
        return register * tick
    }

    return 0
}
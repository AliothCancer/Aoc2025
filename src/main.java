import java.lang.reflect.Array;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.ArrayList;
import java.util.Optional;

void main() {
    Optional<List<String>> input;
    Optional<List<String>> input_big;

    var path = Path.of("src/input.txt");
    var path_big = Path.of("src/input_big.txt");

    try {
        input = Optional.of(Files.readAllLines(path));
        input_big = Optional.of(Files.readAllLines(path_big));
    } catch (IOException e) {
        input = null;
        input_big = null;
    }
    var nonNullInput = input.orElseThrow();
    var nonNullInputBig = input_big.orElseThrow();
    long start = System.nanoTime();
    int total = nonNullInput.stream()
            .mapToInt(line -> get_max_opt(line)).sum();
    long end = System.nanoTime();
    long delta = (end - start) / (1000 * 1000);
    System.out.println("first run on small input " + total + " duration: " + delta + " ms");

    // SECOND RUN
    start = System.nanoTime();
    total = nonNullInputBig.stream()
            .mapToInt(line -> get_max(line)).sum();
    end = System.nanoTime();
    delta = (end - start) / (1000 * 1000);
    System.out.println("Second run on big input" + total + " duration: " + delta + " ms");
    
    start = System.nanoTime();
    total = nonNullInputBig.stream()
            .mapToInt(line -> get_max(line)).sum();
    end = System.nanoTime();
    delta = (end - start) / (1000 * 1000);
    System.out.println("Second run on big input" + total + " duration: " + delta + " ms");
}
int get_max_digits(String line) {
    int max = 0;
    int lenght = line.length();
    for (int n1 = 0; n1 < lenght; n1++) {
        for (int n2 = n1 + 1; n2 < lenght; n2++) {
            
            int ch1 = Character.digit(line.charAt(n1), 10);
            int ch2 = Character.digit(line.charAt(n2), 10);
            

            // String pair = String.valueOf(new char[] { ch1, ch2 });
            // int pair_number = Integer.parseInt(pair);
            int pair_number = ch1 * 10 + ch2;
            if (pair_number > max) {
                max = pair_number;
            }
        }
    }

    return max;
}
int get_max_opt(String line) {
    int max = 0;
    int lenght = line.length();
    ArrayList<Integer> lineInt = new ArrayList<>();
    for (char ch : line.toCharArray()) {
        lineInt.add((int) ch - 48);
    }
    for (int n1 = 0; n1 < lenght; n1++) {
        int ch1 = lineInt.get(n1) * 10;
        for (int n2 = n1 + 1; n2 < lenght; n2++) {
            int ch2 = lineInt.get(n2);
            int pair_number = ch1 + ch2;
            if (pair_number > max) {
                max = pair_number;
            }
        }
    }

    return max;
}

int get_max(String line) {
    int max = 0;
    int lenght = line.length();
    for (int n1 = 0; n1 < lenght; n1++) {
        for (int n2 = n1 + 1; n2 < lenght; n2++) {

            char ch1 = line.charAt(n1);
            char ch2 = line.charAt(n2);

            String pair = String.valueOf(new char[] { ch1, ch2 });
            int pair_number = Integer.parseInt(pair);
            if (pair_number > max) {
                max = pair_number;
            }
        }
    }

    return max;
}

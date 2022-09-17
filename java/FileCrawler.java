import java.io.BufferedInputStream;
import java.io.FileInputStream;
import java.io.IOException;
import java.math.BigInteger;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.security.MessageDigest;
import java.security.NoSuchAlgorithmException;
import java.time.Duration;
import java.time.Instant;
import java.util.ArrayList;
import java.util.List;
import java.util.concurrent.ExecutorService;
import java.util.concurrent.Executors;
import java.util.concurrent.Future;
import java.util.stream.Stream;

public class FileCrawler {

  public static String[] hashFile(Path path) throws IOException, NoSuchAlgorithmException {
    byte[] buffer = new byte[30 * 1024];
    int count;

    MessageDigest digest = MessageDigest.getInstance("SHA-256");
    BufferedInputStream stream = new BufferedInputStream(new FileInputStream(path.toFile()));
    while ((count = stream.read(buffer)) > 0) {
      digest.update(buffer, 0, count);
    }
    stream.close();
    byte[] hash = digest.digest();
    return new String[] { path.toString(), new BigInteger(1, hash).toString(16) };
  }

  public static void main(String[] args) throws IOException {
    Path rootDir = Paths.get(args[0]);

    System.out.println("Start crawling...");
    Instant start = Instant.now();

    ExecutorService executor = Executors.newFixedThreadPool(8);
    List<Future<String[]>> futures = new ArrayList<Future<String[]>>();

    try (Stream<Path> paths = Files.walk(rootDir)) {
      paths.filter(Files::isRegularFile).forEach(path -> futures.add(executor.submit(() -> {
        return hashFile(path);
      })));
    }

    for (Future<String[]> future : futures) {
      try {
        String[] result = future.get();
        System.out.printf("%s: %s\n", result[0], result[1]);
      } catch (Exception e) {
        System.err.printf("Error on file: %s\n", e.getMessage());
      }
    }

    executor.shutdown();

    Instant end = Instant.now();

    System.out.printf("Took %s seconds to crawl %s\n", Duration.between(start, end), rootDir);
  }
}
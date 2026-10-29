import java.nio.file.Files;
import java.nio.file.Paths;

public class Main {
  public static void main(String[] args) throws Exception {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    System.err.println("Logs from your program will appear here!");

    // TODO: Uncomment the code below to pass the first stage
    //
    // final String command = args[0];
    // if (command.equals("bytes")) {
    //   byte[] data = Files.readAllBytes(Paths.get(args[1]));
    //   StringBuilder out = new StringBuilder("[");
    //   for (int i = 0; i < data.length; i++) {
    //     if (i > 0) {
    //       out.append(", ");
    //     }
    //     out.append(data[i] & 0xff);
    //   }
    //   out.append("]");
    //   System.out.println(out);
    // } else {
    //   throw new RuntimeException("Unknown command " + command);
    // }
  }
}

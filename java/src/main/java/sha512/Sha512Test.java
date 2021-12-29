package sha512;

import java.math.BigInteger;
import java.nio.charset.StandardCharsets;
import java.security.MessageDigest;

import org.apache.commons.codec.digest.DigestUtils;

public class Sha512Test {
    public static void main(String[] args) {
        //buildAndHash20x();
        hashIt20x();
    }

    private static void hashIt20x() {
        for(int i=0; i< 20; i++) {
            oneMillionHashes();
        }
    }

    private void justHashIt() {
        long start = System.currentTimeMillis();
        MessageDigest digest = DigestUtils.getSha512Digest();
        String mysamplestring = "myBaseString";
        final int iterations = 2000000;

        for(int i = 0; i<iterations; i++) {
            digest.reset();
            digest.update(mysamplestring.getBytes(StandardCharsets.UTF_8));

            String format = String.format("%0128x", new BigInteger(1, digest.digest()));
            // System.out.println(format);
        }
        long end = System.currentTimeMillis();

        System.out.println("task completed in: " +(end-start)+ "ms");

    }

    private static void buildAndHash20x() {
        for(int i=0; i< 20; i++) {
            oneMillionHashes();
        }
    }

    private static void oneMillionHashes() {
        long start = System.currentTimeMillis();
        MessageDigest digest = DigestUtils.getSha512Digest();
        String mysamplestring = "myBaseString";
        StringBuilder strBuilder = new StringBuilder();
        final int iterations = 1000000;

        for(int i = 0; i<iterations; i++) {
            // reset the string
            strBuilder.delete(0, strBuilder.length());
            strBuilder.append(mysamplestring).append(i);

            digest.reset();
            digest.update(strBuilder.toString().getBytes(StandardCharsets.UTF_8));

            String format = String.format("%0128x", new BigInteger(1, digest.digest()));

           // System.out.println(format);
        }
        long end = System.currentTimeMillis();

        System.out.println("task completed in: " +(end-start)+ "ms");
    }
}

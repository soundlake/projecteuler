public class sol {
    public static void main(String[] args) {
        int i = 1, sum = 0;
        for (; i < 1000; i++)
            if (i % 3 == 0 || i % 5 == 0) sum += i;
        System.out.println(sum);
    }
}

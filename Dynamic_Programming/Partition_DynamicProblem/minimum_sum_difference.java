
class minimum_sum_difference {

    public int minimumDifference(int[] nums) {
        int sums = 0;
        int min = 0;

        for (int i = 0; i < nums.length; i++) {
            if (min > nums[i])
                min = nums[i];
        }
        min=min*-1;
        for (int i = 0; i < nums.length; i++) {
            sums += (nums[i] + min);
            nums[i] = nums[i] + min;
        }

        sums = sums / 2;
        System.out.println(sums);
        if(nums.length==2)
        {
            return Math.abs(nums[0]-nums[1]);
        }
        boolean[][] table = new boolean[nums.length][sums + 1];
        table[0][0] = true;

        for (int i = 0; i < (sums + 1); i++) {
            if (i == nums[0]) {
                table[0][i] = true;
            }
        }

        for (int i = 1; i < nums.length; i++) {
            for (int j = 0; j < (sums + 1); j++) {
                if (j == nums[i]) {
                    table[i][j] = true;
                } else if (table[i - 1][j] == true) {
                    table[i][j] = true;
                } else if (j - nums[i] > 0) {
                    table[i - 1][j - nums[i]] = true;
                }
            }
        }

        // 🔹 DISPLAY DP TABLE
        System.out.println("\nDP Table:");
        for (int i = 0; i < nums.length; i++) {
            for (int j = 0; j <= sums; j++) {
                System.out.print((table[i][j] ? "1 " : "0 "));
            }
            System.out.println();
        }

        int l = sums;
        while (table[nums.length - 1][l] == false) {
            l--;
        }
        System.out.println(l);
        return (sums-l);
    }

    public static void main(String[] args) {
        minimum_sum_difference obj = new minimum_sum_difference();

        int[] nums = {-36,36};
        int result = obj.minimumDifference(nums);

        System.out.println("\nResult: " + result);
    }
}

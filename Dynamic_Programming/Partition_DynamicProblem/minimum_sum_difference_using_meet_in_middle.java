import java.util.*;

class Solution {
    public int minimumDifference(int[] nums) {

        int n = nums.length;
        int len = n / 2;

        int[] left = Arrays.copyOfRange(nums, 0, len);
        int[] right = Arrays.copyOfRange(nums, len, n);

        // leftSums[k] = all subset sums of left using k elements
        ArrayList<Integer>[] leftSums = new ArrayList[len + 1];
        ArrayList<Integer>[] rightSums = new ArrayList[len + 1];

        for (int i = 0; i <= len; i++) {
            leftSums[i] = new ArrayList<>();
            rightSums[i] = new ArrayList<>();
        }

        // 🔹 generate left subsets
        for (int mask = 0; mask < (1 << len); mask++) {
            int sum = 0, bits = 0;
            for (int i = 0; i < len; i++) {
                if ((mask & (1 << i)) != 0) {
                    sum += left[i];
                    bits++;
                }
            }
            leftSums[bits].add(sum);
        }

        // 🔹 generate right subsets
        for (int mask = 0; mask < (1 << len); mask++) {
            int sum = 0, bits = 0;
            for (int i = 0; i < len; i++) {
                if ((mask & (1 << i)) != 0) {
                    sum += right[i];
                    bits++;
                }
            }
            rightSums[bits].add(sum);
        }

        int totalSum = 0;
        for (int x : nums) totalSum += x;

        int ans = Integer.MAX_VALUE;

        // sort right side for binary search
        for (int i = 0; i <= len; i++) {
            Collections.sort(rightSums[i]);
        }

        // 🔹 meet in the middle
        for (int k = 0; k <= len; k++) {
            ArrayList<Integer> A = leftSums[k];
            ArrayList<Integer> B = rightSums[len - k];

            for (int a : A) {
                int target = totalSum / 2 - a;
                int idx = Collections.binarySearch(B, target);

                if (idx >= 0) {
                    ans = Math.min(ans, Math.abs(totalSum - 2 * (a + B.get(idx))));
                } else {
                    idx = -idx - 1;
                    if (idx < B.size())
                        ans = Math.min(ans, Math.abs(totalSum - 2 * (a + B.get(idx))));
                    if (idx > 0)
                        ans = Math.min(ans, Math.abs(totalSum - 2 * (a + B.get(idx - 1))));
                }
            }
        }

        return ans;
    }
}

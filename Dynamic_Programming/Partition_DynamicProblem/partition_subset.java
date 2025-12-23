class partition_subset {
    public boolean canPartition(int[] nums,int difference) {
        // first calculate the total sum
        int sums=0;
        for(int i=0;i<nums.length;i++)
        {
         sums+=nums[i];
        }
        if(sums%2!=0)
        return false;
        sums=sums/2;
        boolean[][] table=new boolean[nums.length][sums+1];
        table[0][0]=true;
        for(int i=0;i<(sums+1);i++)
        {
            if(i==nums[0])
            table[0][i]=true;
        }

        // now fill the entire matrix with the data fromt the above
        for(int i=1;i<table.length;i++)
        {
            for(int j=0;j<(sums+1);j++)
            { System.out.println(j);
                if (nums[i]==j)
                table[i][j]=true;
                else if(table[i-1][j]==true)
                {   
                    table[i][j]=true;
                }
                else if(j-nums[i]>=0)
                {
                    if(table[i-1][j-nums[i]]==true)
                    table[i][j]=true;
                }
            }
        }
 
    print_matrix(table);
    return table[nums.length-1][sums-difference];
    }

    public static void main(String[] args) {
        partition_subset sol = new partition_subset();
        int[] nums = {1,1,2,3};
        int difference=1;
        System.out.println(sol.canPartition(nums,difference));
    }

    public static void print_matrix(boolean[][] table)
    {
        System.out.println("executing");
           for (boolean[] row : table) {
    for (boolean val : row) {
        System.out.print((val ? 1 : 0) + " ");
    }
    System.out.println();
}

    }
}

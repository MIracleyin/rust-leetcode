fn zeroone_bag(weight: Vec<i32>, value: Vec<i32>, bag_weight: usize) {
    // row is items
    // col is values
    // dp[i][j] 表示从下标为[0-i]的物品里任意取，放进容量为j的背包，价值总和最大是多少
    // dp
    let mut dp = vec![vec![0; bag_weight + 1]; weight.len()];
    // update
    // dp[i][j]
    // 1. can't put it in
    // dp[i][j] = dp[i-1][j]
    // 2. put it in
    // dp[i][j] = dp[i-1][j - weight[i]] + value[i]
    // choose bigger
    // dp[i][j] = max(dp[i-1][j], dp[i-1][j - weight[i]] + value[i])
    // init dp
    for i in 0..weight.len() {
        dp[i][0] = 0;
    }
    for j in 0..weight[0] as usize {
        dp[0][j] = 0;
    }
    for j in weight[0] as usize..=bag_weight {
        dp[0][j] = value[0];
    }

    for i in 1..weight.len() {
        for j in 1..=bag_weight {
            if j < weight[i] as usize {
                // can't put it in
                dp[i][j] = dp[i - 1][j]
            } else {
                dp[i][j] = dp[i - 1][j].max(dp[i - 1][j - weight[i] as usize] + value[i]);
            }
        }
    }
    dbg!(dp.clone());

}

#[test]
fn it_works() {
    let weight = vec![1, 3, 4];
    let value = vec![15, 20, 30];
    let bag_weight = 4;
    zeroone_bag(weight, value, bag_weight);
}

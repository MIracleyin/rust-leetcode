fn full_bag_v1(weight: Vec<i32>, value: Vec<i32>, bag_weight: usize) {
    let mut dp = vec![vec![0; bag_weight + 1]; weight.len()];

    for i in 0..weight.len() {
        dp[i][0] = 0;
    }
    for j in 0..weight[0] as usize {
        dp[0][j] = 0;
    }
    for j in weight[0] as usize..=bag_weight {
        dp[0][j] = value[0] * j as i32;
    }

    // for i in 1..weight.len() {
    //     for j in 0..=bag_weight {
    //         if j < weight[i] as usize {
    //             dp[i][j] = dp[i - 1][j];
    //         } else {
    //             dp[i][j] = dp[i - 1][j].max(dp[i - 1][j - weight[i] as usize] + value[i]);
    //         }
            
    //     }
    // }
    for j in 0..=bag_weight {
        for i in 1..weight.len() {
            if j < weight[i] as usize {
                dp[i][j] = dp[i - 1][j];
            } else {
                dp[i][j] = dp[i - 1][j].max(dp[i - 1][j - weight[i] as usize] + value[i]);
            }
            
        }
    }
    dbg!(dp.clone());
}

fn full_bag(weight: Vec<i32>, value: Vec<i32>, bag_weight: usize) {
    let mut dp = vec![0; bag_weight + 1];

    for j in 0..=bag_weight {
        // bag
        for i in 1..weight.len() {
            if j as i32 - weight[i] >= 0 {
                dp[j] = dp[j].max(dp[j - weight[i] as usize] + value[i]);
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
    full_bag_v1(weight, value, bag_weight);
}

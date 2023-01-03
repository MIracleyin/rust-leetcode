fn mutil_bag_v1(mut weight: Vec<i32>, mut value: Vec<i32>, mut nums: Vec<i32>, bag_weight: usize) {
    for i in 0..nums.len() {
        while nums[i] > 1 {
            weight.push(weight[i]);
            value.push(value[i]);
            nums[i] -= 1;
        }
    }

    let mut dp = vec![0; bag_weight + 1];

    for i in 0..weight.len() {
        for j in (weight[i] as usize..=bag_weight).rev() {
            dp[j] = dp[j].max(dp[j - weight[i] as usize] + value[i]);
        }
    }
    dbg!(dp.clone());
}

fn mutil_bag(mut weight: Vec<i32>, mut value: Vec<i32>, mut nums: Vec<i32>, bag_weight: usize) {
    let mut dp = vec![0; bag_weight + 1];

    for i in 0..weight.len() {
        for j in (weight[i] as usize..=bag_weight).rev() {
            for k in 1..=nums[i] as usize {
                if j as i32 >= k as i32 * nums[i] {
                    dp[j] = dp[j].max(dp[j - k * weight[i] as usize] + k * value[i] as usize);
                }
            }
        }
    }
    dbg!(dp.clone());
}

#[test]
fn it_works() {
    let weight = vec![1, 3, 4];
    let value = vec![15, 20, 30];
    let nums = vec![2, 3, 2];
    let bag_weight = 10;
    mutil_bag(weight, value, nums, bag_weight);
}

# 回溯法
一种高效的搜索算法，是递归的副产品

# 效率
回溯到本质是穷举

# 回溯可以解决的问题
1. 组合
2. 切割
3. 子集
4. 排列
5. 棋盘

# 如何理解回溯法
本质上是 N 叉树的遍历

# 回溯模版
函数
```
fn backtracking(args...) -> ! {

}
```

终止条件
```
if end {
    return
}
```

遍历过程
``
for i in 本层元素 {
    process i
    backtracking() 
    undo i
}
```


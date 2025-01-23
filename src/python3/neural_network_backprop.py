import numpy as np

def sigmoid(x):
    """sigmoid激活函数"""
    return 1 / (1 + np.exp(-x))

def sigmoid_derivative(x):
    """sigmoid函数的导数"""
    s = sigmoid(x)
    return s * (1 - s)

def one_step_sgd(x, y, W1, W2, lr):
    """
    执行一步随机梯度下降
    参数:
        x: 输入数据，形状为(n_features,)
        y: 目标值，标量
        W1: 第一层权重矩阵，形状为(n_hidden, n_features)
        W2: 第二层权重矩阵，形状为(1, n_hidden)
        lr: 学习率
    返回:
        nW1: 更新后的第一层权重
        nW2: 更新后的第二层权重
    """
    # 前向传播
    z1 = np.dot(W1, x)  # 第一层的加权和
    a1 = sigmoid(z1)    # 第一层的激活值
    z2 = np.dot(W2, a1) # 第二层的加权和
    y_pred = z2         # 预测输出（线性输出）
    
    # 计算损失（平方损失）
    loss = 0.5 * (y_pred - y) ** 2
    
    # 反向传播
    # 输出层的误差
    delta2 = (y_pred - y)
    
    # 隐藏层的误差
    delta1 = np.dot(W2.T, delta2) * sigmoid_derivative(z1)
    
    # 计算梯度
    dW2 = np.outer(delta2, a1)
    dW1 = np.outer(delta1, x)
    
    # 更新权重
    nW1 = W1 - lr * dW1
    nW2 = W2 - lr * dW2
    
    return nW1, nW2 
import numpy as np

def relu(x):
    """ReLU激活函数"""
    return np.maximum(0, x)

def relu_derivative(x):
    """ReLU函数的导数"""
    return np.where(x > 0, 1, 0)

def one_step_sgd_relu(x, y, W1, W2, lr):
    """
    执行一步随机梯度下降（使用ReLU激活函数）
    参数:
        x: 输入数据，形状为(d,)
        y: 目标值，标量
        W1: 第一层权重矩阵，形状为(h, d)
        W2: 第二层权重向量，形状为(h,)
        lr: 学习率
    返回:
        nW1: 更新后的第一层权重
        nW2: 更新后的第二层权重
    """
    # 前向传播
    z1 = np.dot(W1, x)      # 第一层的加权和
    a1 = relu(z1)           # 第一层的激活值（ReLU）
    y_pred = np.dot(W2, a1) # 预测输出
    
    # 计算损失（平方损失）
    loss = 0.5 * (y_pred - y) ** 2
    
    # 反向传播
    # 输出层的误差
    delta2 = (y_pred - y)
    
    # 隐藏层的误差
    delta1 = W2 * relu_derivative(z1) * delta2
    
    # 计算梯度
    dW2 = delta2 * a1
    dW1 = np.outer(delta1, x)
    
    # 更新权重
    nW1 = W1 - lr * dW1
    nW2 = W2 - lr * dW2
    
    return nW1, nW2 
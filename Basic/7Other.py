# coding: utf-8
# Author: @safemoc
# 三元表达式
# 需要一个功能： 接收两个值 返回大的值
# 格式： true时的值 if 条件 else false 时的值
def func(x, y):
    # if x > y:
    #     return x
    # else:
    #     return y
    return x if x > y else y


func(3, 5)

# 列表推导式
# 格式： [结果  for item in 可迭代对象 if 条件 ] -- if 条件可以不写
l = ['123', '11123123', '12333213123', '31231231231231']
n = [name for name in l if name.endswith('123')]

n = [1 if name == '123' else 2 for name in l if 1]  # 三元表达式 与列推导式 组合使用

# 集合推导式与列表推导式相同 只不过 中括号 变为大括号


# 字典推导式
l = [('康师傅', 3), ('今麦郎', 4), ('大金野', 5), ('汤达人', 6)]
dic = {k: v for k, v in l if k != '康师傅'}

# 元组没有推导式 因为元组本身不可变
l = ['123', '11123123', '12333213123', '31231231231231']
n = (name for name in l if name.endswith('123'))
# 得到的实际结果是一个生成器

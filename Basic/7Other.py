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


# 小案例： 统计文字 字数：
with open('../Data/user.log', mode='rt', encoding='utf-8') as f:
    size = sum([len(l) for l in f])  # 如果 文件行数过高 会导致内存资源紧缺
    size = sum((len(l) for l in f))  # 使用生成器表达式 满足生成器特性，只有使用到值的时候会有当前值的内存
    size = sum(len(l) for l in f)  # 因为本身 在函数内使用 推导式，可以省略推导式的 () 简写为
    print(size)

# 函数嵌套
# 1 定义的时候嵌套 （装饰器）
# 2 调用的时候嵌套

# 递归
# 在调用函数的时候 函数内部调用了 它本身
# python 设定递归深度不可超过1000
# 修改递归深度：
import sys

print(sys.getrecursionlimit())  # 查看当前递归深度
sys.setrecursionlimit(1500)  # 修改递归深度为1500

# 递归 类似与循环，只不过一个是 使用循环 一个是函数
#  ！！！！
# python 没有尾递归优化
#  ！！！！

# 案例： 计算1-10 的和：
# 循环
i = 10
n = 0
while i > 0:
    n += i
    i -= 1


# 递归
def mysun(i):
    return 0 if i == 0 else mysun(i - 1)


mysun(10)

# 把列表的所有值单独打印出来
l = [1, 2, [3, 4, [5, [6, [7, [8, [9, [10]]]]]]]]


def func(li):
    for i in li:
        if type(i) is list:
            func(i)
        else:
            print(i)


# 把下面字符做全排列 ： 对字符进行排列， 将每种排列的可能都打印出来
# 思路解析 对字符串每个字符 都固定为首位 剩余字符 再进行全排列
s = 'abcd'
l = list(s)


def func(li: list, level):
    if li.__len__() == level:
        print(li)
    else:
        for i in range(level, len(li)):
            li[i], li[level] = li[level], li[i]
            func(li, level + 1)
            li[i], li[level] = li[level], li[i]


# 二分法
# 什么是算法
# 一种高效解决问题的方法

# 效果相同 判断算法的好坏 从： 花费的时间、消耗的资源
# 效果相同 算法 花费的时间越少 消耗的资源越小 这个算法就是好算法

# 实现二分法：

l = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]


def erfen(l, num):
    if len(l) <= 1 and l[0] != num:
        print(2222)
        return

    if l[int(len(l) / 2)] == num:
        print('111')
        return None
    elif l[int(len(l) / 2)] > num:
        erfen(l[:int(len(l) / 2)], num)
    else:
        erfen(l[int(len(l) / 2):], num)


# 匿名函数 lambda
(lambda x, y: x + y)()  # 没有名字 可以直接（） 调用
# 因为匿名函数返回的是一个函数的内存地址 可以直接 拿变量承接，可是这样就违背了匿名函数的初衷
func = lambda x, y: x + y

# 匿名函数因为没有名字，所以从创造出来就被回收了
# 特点 有名函数(重复调用)  匿名函数(临时调用一次)
# 匿名函数一般和其他函数配合使用
# 例如 max 是对可迭代对象 判断大小的 因为它值判断 迭代者 对于字典的max 就是对键的判断 如果键是字符串 那么它会比对 字符的第一位的 ASCII 表的 位置大小
# max 有一个功能是key 接收的是一个内存地址,它会按照 key 这个函数的返回值 进行比较
dic = {'a': 100, 'b': 333, 'c': 123123}
max(dic, key=lambda k: dic[k])  # 实现按照字典的值进行排序
# 列表的 sort 也是可以这样操作的
l = [1, 2, 34, 5]
l.sort(reverse=False, key=lambda x: x)
# sorted() 也是可以的

# map
# map 接收一个 函数地址,接收一个 迭代对象
# 它会对每个迭代对象 执行 函数的功能
# 返回的结果与 生成器相同

t = (1, 3, 5, 6)
res = map(lambda x: x + 1, t)
print(res)


# 函数的类型提示
def func(name: 'str', age: '阳光帅气多金'=18) -> '哈哈哈哈': # =18 就是定义的默认值
    print(name)
    print(age)
    return 10
# 如果传参不是对应的数据类型 也不会报错,仅仅是一个提示作用
# 只是提示,可以随便写

# 如果想查看一个函数的提示信息:
print(func.__annotations__()) #大多数不会用 但是看别人源码就很有用
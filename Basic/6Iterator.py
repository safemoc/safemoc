# coding: utf-8
# Author: @safemoc
# 迭代器: 不依赖与索引的迭代取值方式
# 迭代的每一次重复 都跟上一次的结果是有关联的
# 示例：
num = 1
while 1:  # 每次循环都是 基于上次循环结果
    num += 1
    print(num)
    break  # 需要注释此行
# 列表循环
l = [1, 2, 3, 4]
index = 0
while index < len(l):
    print(l[index])
    num += 1

# 当前迭代是基于索引的 元组 字符串 也可以 可是 字典 集合 open文件 是不可以的
# 就需要使用迭代器了
# 可迭代对象: 可以转换为迭代器的对象就是可迭代对象
# 只要可以被 for 循环的就是可迭代对象,    |   只要内置有 __iter__() 的方法就是可迭代对象
froze = {1, 2, 3}
res = froze.__iter__()  # <set_iterator at 0x19b42468e00> 调用__iter__() 方法会将对象转换为 可迭代对象(迭代器)
# 迭代器 可以使用 __next__ 方法进行取值
res.__next__()  # 每执行一次就会 获取一个值
res.__next__()
res.__next__()
res.__next__()  # 第四次取值的时候会抛出错误 # StopIteration
# 迭代器 在创建时就 已经被设定好了,可取值总量,每执行一次__next__ 就会少一个,当数值取完就无法在执行取值了
# 并不是说明 在一创建的时候 就存储着 所有数据  而是一个一个取的
# 迭代器 很节省内存资源
# 使用迭代器 不依赖索引 读取 所有数据
dic = {1: 1, 2: 2, 3: 3}
dic = dic.__iter__()
while 1:
    try:
        print(dic.__next__())
    except StopIteration:  # 当发生 这个错误的时候就会执行 这个代码块

        break

# 迭代器调用 __iter__ 方法得到的结果是本身   | 统一 for循环的运行机制
for i in dic:
    print(i)


# 运行逻辑:
# 1 不管对象是迭代器还是可迭代对象,都调用__iter__ 方法 获取迭代器版本
# 2 调用迭代器的 __next__ 方法,拿到值反馈给 i
# 3 循环执行第二步 直到出现异常,结束循环


# 生成器
def ff():
    print(11111)
    yield 1
    print(22222)
    yield 2
    print(333333)
    yield 3


res = ff()  #
print(id(res))  # yield 返回的是一个生成器  -- 生成器 其实就是迭代器
res.__next__()  # 函数会执行到 第一个 yield 返回结果后停止
res.__next__()  # 函数继续执行 到第二个yield 然后停止
l = [1, 2, 1, 3, 1, 4, 1, 5]
# 平时统计长度
len(l)
# 等同于
l.__len__()

# 所以 调用 __next__()  可以这样写
next(res)
# __iter__() 也可以 写为
iter(res)


# yield 表达式

def func(x):
    print(f'{11} kaishi ')
    n = 0
    while " ":
        # 将yield 赋值给变量之后 就可以尝试 接收值
        y = yield n
        print(x, '----------', y)
        n += 1


g = func(20)
g.send(None)  # 刚创建的生成器 是不可以接收值的
g.send(100)  # 此时 g.send 的值就会传递给 yield 然后赋值给 y

for i in func(10):
    l = [1, 2, 3]
    i.send(l)

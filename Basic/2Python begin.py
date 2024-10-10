"""
这是注释
"""
# 这是注释
'''
这是注释
'''

'''
变量:
    -先定义,在引用
'''
variable = 123  # 定义 在内存中开辟空间存储变量中的数据
print(variable)  # 引用 在内存中寻找对应变量的空间,并使用其数据
'''
内存空间有限,申请了内存空间不用 就是白白浪费内存资源,导致内存空间被占满 这就是内存溢出
用了以后及时释放空间,避免内存溢出,这个过程就是内存管理

Python推出了一个内存管理机制:垃圾回收机制
可以不用考虑内存溢出的问题

如果内存中有一个空间没有绑定任何变量名 说明这是一个垃圾---将被回收
如果内存中有一个空间绑定了3个变量名 这个内存空间的引用计数就是3
如果 这三个都 解除绑定的话,这个变量将被回收
'''

a = 1000
b = a
c = a
del a
print(id(b), id(c))  # id相同
name1 = 'hello'
name2 = 'hello'
# id不同,如果id相同,说明pycharm进行了优化,可以跳出Pycharm 在终端运行查看

# 变量的两个特性
'''
id:
id不是内存地址 是通过变量值的内存地址计算出来的 类似身份证号
type:
是数据的类型
'''
# is:判断两个内存的id是否相等 也就是内存空间是否相等
# id不同值可能相同 id相同 值一定相同
# 小整数池 -- python进行的优化,避免相同的值占用多个不同的内存空间 范围是-5 到256
# python常见的字符串也做了池 例如 a b c 在Pycharm中运行 Pycharm考虑性能扩大了池的范围 所以Pycharm 非常吃性能
# 虽然pycharm 做了优化但是最终效果还是以Python解释器的结果为准
# 在编程概念里拥有常量 但是在python中是没有常量概念的,所以
# 一种约定规范 变量名全大写代表常量

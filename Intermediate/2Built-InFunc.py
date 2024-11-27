# coding: utf-8
# Author: @safemoc

# 内置方法的作用就是 ： 在满足某些条件的时候自动执行


# 1 __str__
# print 一个类的时候 显示的是 这个类的内存地址
# 在类中重写了 __str__ 方法后
# str 返回值就是 print 类的结果
class A:
    def __init__(self, name, age):
        self.name = name
        self.age = age

    def __str__(self):
        print('123123')  # __str__ 在实例的时候就会执行一次
        return f'{self.name}:{self.age}'  # __str__ 必须有返回值且必须是字符串


obj = A(11, 22)


# __del__
# 在删除对象的时候,先执行这个参数,只要对象要被回收了,就执行__del__ 之后在回收

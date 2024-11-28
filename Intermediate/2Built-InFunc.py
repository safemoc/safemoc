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
class B:
    f = open(fr'.\file\path', 'wb', encoding='utf-8')

    def __del__(self):
        self.f.close()
        # 类被回收之前会关闭 f 文件


# 元类 : 就是实例化产生类的类
# 在python中基于类可以创建对象
# 定义了一个类
class Human(object):
    star = 'earth'

    def __init__(self, name, age, gender):
        self.name = name
        self.age = age
        self.gender = gender

    def info(self):
        print('name:', self.name, 'age:', self.age)


# 基于类创建一个对象
obj = Human('张大仙', 22, '男')

# 但是在python中一切皆是对象,那么类也是对象
# 就说明 类也是基于某个类创建的一个对象
# 结果就是
# Human = 调用了某一个类() 这个类就是元类

# 元类 --实例化--> 类 --实例化--> 对象
# 我们可以使用type来查看对象的属性,
print(type(obj))
# 因为一切皆对象的概念
print(type(Human))
# 可以发现类是type 也就是说 type 是元类,是内置的类
# 用class 定义的类 以及内置的类(str list dict ......) 都是由 元类type 实例化产生的

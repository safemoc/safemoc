# coding: utf-8
# Author: @safemoc
# 什么是对象？
# 对象就是"容器" 用来存放东西
# 类
# 类的 名称空间 在类被定义的时候 子代码内容就运行了
class Hero:
    hero_work = '射手'

    def __init__(self, name, hp):
        self.name = name
        self.hp = hp


print(Hero.hero_work)
# 它的取值方式是：
print(Hero.__dict__['hero_work'])


class Learn():
    def __init__(self):  # 类在实例化时调用
        ...  # init 应该返回 None 也只能返回None


# 调用类的过程
# 1 ： 创建空对象
# 2 ： 调用类中的__init__() 方法，同时把空对象以及调用类时传递的参数，传递给 __init__()方法
# 3 :  返回初始化之后的对象


# 属性查找顺序
# 先从自己的实例对象中查找
# 实例对象中没有,会去 类中查找
# 类的属性变化了, 所有通过 这个类实例 的对象都会变化
Hero.hero_work = '儿子'
lb = Hero('鲁班', 450)
print(lb.hero_work)  # 结果是 儿子
# 因为 类中属性变化了,实例中也会跟着变化
lb.hero_work = 123
print(Hero.hero_work)  # 结果是 儿子


# 因为 lb.hero_work 实际上是赋值操作,在lb的实例中添加了一个 hero_work 并不会修改 类中的属性
# 创建一个 可以统计 有多少个实例对象的 类

class count_():
    count = 0

    def __init__(self):  # 第一个参数是 self 代表实例本身,也可以叫做其他名字但是 规范定义为  self
        self.count += 1  # self 对应的是 实例对象的 属性 而不是 类的属性
        count_.count += 1  # 调用自身 的属性,修改的就是自身的属性

    def __del__(self):
        count_.count -= 1


# 绑定方法
# 类中定义的方法主要是给对象去用的
# 而且是绑定给对象使用的
# 虽然所有的对象都是相同的功能
# 但是绑定到不同的对象
# 就会编程不同的绑定方法,

# 绑定方法会自动将 实例对象的自身 当作第一个参数 实现,类中 self 的位置参数

# 在python中一切皆对象,

# 'aaa' .count(self,str,__start,__end)
x = 'aaa'  # 实际就是  x = str('aaa')
# str 是python的一个基本类

l = [1, 2, 3]
l.append(4)
# === 等同于
list.append(l, 4)


# 封装 继承 多态
# 封装:整合
# 隐藏属性
# 特点:
# 1 : 只是改名字,并没有真正意义上的隐藏
# 2 : 对外部队内, 内部是可以正常使用 self.__var_name 进行调用的 外部就不可以
# 3 : 改名只会在定义阶段检查语法结构的时候执行一次,后续添加的__开头的属性是不会改名的
class Test:
    __age = 10

    def get_age(self):
        return self.__age

    def set_age(self, age):
        if type(age) is not int:
            print(111111111111)
        if 0 <= age <= 100:
            print(22222222222222)
        self.__age = age

    def del_age(self):
        del self.__age


print(Test.__age)  # 报错 提示没有这个名字
print(Test.__dict__)  # 可以看到实际上 在定义阶段 __age 变成了 _Test__age

# 为什么隐藏属性
# 不允许直接对属性进行修改,必须要按照公开的接口对属性修改
# 隐藏功能,隐藏功能步骤,让使用不用在乎 功能的步骤,只知道结果就可以了


# 类装饰器
# property   python内置的装饰器  作用: 把绑定给对象的方法,伪装为成为一个数据属性
obj = Test()
obj.get_age()  # 必须要 .get_age() 才能获取到年龄,应该是 .age 就是年龄
obj.set_age(15)  # 应该是 obj.age = 15 就可以修改
obj.del_age()  # 应该是 del obj.age 就可以删除


# 使用 property 重新写一个类

class Test:
    def __init__(self):
        self.__age = 10

    # @property  # 这种写法在调用的时候就不用添加() 直接函数名就可以了  ==>   get_age = property(get_age)
    def get_age(self):
        return self.__age

    def set_age(self, age):
        if type(age) is not int:
            print(111111111111)
        if 0 <= age <= 100:
            print(22222222222222)
        self.__age = age

    def del_age(self):
        del self.__age

    age = property(get_age, set_age, del_age)  # 这种写法就可以直接实现 .age, age = xxx , del age 的操作
    # 这种写法的顺序只能是 查 改 删
    # 但是这种写法太麻烦了,为了简化这种写法, 还是要使用 语法糖来实现


obj = Test()
print(obj.age)
obj.age = 33
print(obj.age)
del obj.age
print(obj.age)


# 使用语法糖来实现 对类方法的封装
class Test:
    def __init__(self):
        self.__age = 20

    @property
    def age(self):
        return self.__age

    @age.setter
    def age(self, new_age):
        if type(new_age) is not int:
            print(111111111111)
        if 0 <= new_age <= 100:
            print(22222222222222)
        self.__age = new_age

    @age.deleter
    def age(self):
        del self.__age


obj = Test()
print(obj.age)
obj.age = 33
print(obj.age)
del obj.age
print(obj.age)

# ======================================================================================================================

# 继承

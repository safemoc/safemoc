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

    def __init__(self):
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

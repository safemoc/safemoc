# coding: utf-8
# Author: @safemoc
# 什么是对象？
# 对象就是"容器" 用来存放东西
# 类
# 类的 名称空间 在类被定义的时候 子代码内容就运行了
from tkinter.font import names


class Hero:
    hero_work = '射手'

    def __init__(self, name, hp):
        self.name = name
        self.hp = hp


print(Hero.hero_work)
# 它的取值方式是：
print(Hero.__dict__['hero_work'])


class Learn(object):
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

class count_(object):
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
# 创建新类的方式,被继承的类叫做父类(基类) 通过继承创建的类称之为子类
class Parent1:
    pass


class Parent2:
    ...


class Child1(Parent1):  # 单继承
    ...  # 只继承了一个父类,这种继承叫做单继承


class Child2(Parent1, Parent2):  # 多继承 --> 在很多语言中不支持多继承
    ...  # 继承了两个父类,这种继承叫做多继承


print(Child2.__bases__)
print(Child1.__bases__)


# 在pyton2 类分为两种: 新式类\经典类
# 新式类: 类继承了object类,或它的子类 而创建的类 叫做新式类
# 经典类: 没有继承 object类,或它的子类,而创建的类叫做经典类
# python3中没有继承的类默认继承object 也就是python2中的新式类
# 为了让你的代码可以兼容python2 可以在创建类的时候 声明一下 继承 object 类

class Test(object):
    ...  # 新式类


# 继承的特性: --> 遗传
# 子类会遗传父类的属性 --> 子类自己有的就用自己的,自己没有 就找父类要

# 多继承
# 优点:
# 一个类可以继承多个父类,减少了代码的冗余
# 缺点：
# 1: 多继承违背了人的思维习惯
# 2: 多继承会让代码的可读性变差
# 如果要用多继承,应该用 Minxins
#   抽象过程:
#       抽取对象的共有部分,定义为一个类,然后抽取类的公有部分,为基类(父类)
#   继承过程:
#       与抽象过程相反,先定公有部分为父类,然后依次继承和实例到最后为对象
# 如果子类继承的有公有属性,但是子类自己定义了这个属性,那么不会去父类寻找这个属性了 --> 派生概念:父类有这个属性,子类自己派生了新的属性,或者父类没有但是子类自己派生了

# 继承实现

class Chinese:
    star = 'earth'
    nation = 'China'

    def __init__(self, name, age, gender, ) -> None:
        self.name = name
        self.age = age
        self.gender = gender

    def speak(self) -> None:
        print(f'{self.name} speak Chinese')


class American:
    star = 'earth'
    nation = 'America'

    def __init__(self, name, age, gender):
        self.name = name
        self.age = age
        self.gender = gender

    def speak(self):
        print(f'{self.name} speaking English')


# 提取 公有属性
class Human(object):  # 作为父类
    star = 'earth'

    def __init__(self, name, age, gender):
        self.name = name
        self.age = age
        self.gender = gender


class Chinese(Human):
    nation = 'China'

    def speak(self) -> None:
        print(f'{self.name} speak Chinese')


class American(Human):
    nation = 'America'

    def speak(self):
        print(f'{self.name} speaking English')


xx = Chinese('许仙', 22, '男')


# 调用类中的 __init__方法 子类没有就去父类找,使用父类中 的 __init__


# 派生需求，要在Chinese类中 添加一个 balance的属性。 但是在 父类中 没有balance
# 所以不是重新定义 父类的方法，也不是定义自己的方法，而是根据父类的方法进行扩展
# 参考：
class Chinese(Human):
    def __init__(self, name, age, gender, balance):
        Human.__init__(self, name, age, gender)  # 因为直接调用类方法，就不是绑定方法了，第一个参数就需要传对象参数了
        self.balance = balance


# 以上就实现了 扩展但不重写 父类方法了


# 属性查找：
# 对象-类-父类-----> 祖先类（object)
class Test1:
    def f1(self):
        print('test1.f1')

    def f2(self):
        print('test1.f2')
        self.f1()


class Test2(Test1):
    def f1(self):
        self.f2()


obj = Test2()
obj.f2()


# 1 obj对象没有f2
# 2 Test2 类没有f2
# 3 Test1 父类有f2 --> 调用 self.f1()   -- self 代表对象本身
# 4 self-->obj 对象没有f1
# 5 obj 的 Test2 类有f1
# 6 执行 Test2 f1
# 如果想调用 Test1的f1 的话 可以通过 类调用 将 self.f1() 替换为 Test1.f1() # 类调用就不再是绑定方法了，第一个参数self 需要传值。
# Test1.f1(self)
# 或者将f1方法 改名为 __f1()
# 因为在实例的时候 __f1() 会更名为 _Test1__f1 所以就可以调用了


# 多继承属性查找：
# 多继承会造成一个问题：菱形继承（钻石继承）
# 案例：
class A:  # 非object类
    ...


class B(A):
    ...


class C(A):
    ...


class D(B, C):
    ...


# D 继承了 BC 两个类 这两个类有一个共同的父类A

# 属性查找时 会先从对象-> 类->父类
# 但是类有两个父类，应该怎么找？
# 根据 MRO列表【Method Resolution Order】顺序进行查找
print(D.mro())  # ！！！ 经典类是没有 mro方法的


# 通过 C3算法 来计算 MRO列表

# python 会给每个类都计算一个 MRO列表 属性的查找就会按照这个顺序来查找的

# 非菱形继承
# 会从左到右依次按照分支查找，先找最左侧的父类 此父类没有 查找此父类的父类。完成这个分支，查找下一个父类


# 菱形继承
# 经典类：深度优先查找 -- 找第一条分支的时候，就要找共同的父类
# 新式类：广度优先查找 -- 找完最后一条分支之后，才找共同的父类
# 这个与数据结构里的 广度优先|深度优先 不同


# 多继承 会导致代码可读性变差
# 鸭 鹅 会游泳 但是鸡不会，所以不能在家禽类里面定义 swimming 功能
# 只能定义一个 Swim类 让 鸭 鹅 继承 但是类的继承 更多体现的是一种 属于的关系，
# 目前看的话 就是 鸭 鹅 属于家禽 同时属于游泳类  在可读性上变差了
# 引入了 MixIns 机制，
# MixIns：并不修改任何机制，只是修改了类名，表示此类是一个混入类。并不代表属于关系
class Fowl(object):  # 家禽类
    ...


class SwimMixin(object):
    def swimming(self):
        ...

    ...


class Chicken(Fowl):  # 鸡
    ...


class Duck(SwimMixin, Fowl):  # 鸭
    ...


class Goose(SwimMixin, Fowl):  # 鹅
    ...


# 继承主要表现的是   什么 “是” 什么的关系 也可以成为  is-a 关系

# super()  严格依赖继承关系实现重用父类代码
# super 只能用在 新式类中  新式类 还分为 python2 和python3
# 在python2中： super(cls,self)
# 在python3中： super() # python3会自动将当前类，以及当前类self 传递


class Human(object):  # 作为父类
    star = 'earth'

    def __init__(self, name, age, gender):
        self.name = name
        self.age = age
        self.gender = gender


class Chinese(Human):
    def __init__(self, name, age, gender, balance):
        super().__init__(name, age, gender)  # super会参照self所属类的mro列表从当前类所处位置的下一个类开始找属性
        self.balance = balance

    def speak(self) -> None:
        print(f'{self.name} speak Chinese')


class American(Human):
    nation = 'America'

    def speak(self):
        print(f'{self.name} speaking English')


# 测试super 的用法
class A:
    def f1(self):
        print('A.f1')
        super().f1()


class B(A):
    def f1(self):
        print('B.f1')

    ...


class C(A, B):
    ...


obj = C()
obj.f1()


# C().f1() -> C没有去父类找 C的mro的下一个，A.f1()
# A.f1() 中的super.f1() 会从C的mro中A的下一个位置开始找 也就是B.f1()


# 多态：面向对象的最后一个特性了
# 在继承的背景下，不用关子类是否重写了父类的方法，只要父类有，子类就一定可以使用这个方法。
# 统一了使用标准


class Car(object):
    def run(self):
        print('开始跑')


class Benz(Car):
    def run(self):
        super().run()
        print('98')

    ...


class Lx(Car):
    def run(self):
        super().run()
        print(95)

    ...


class Auto(Car):
    def run(self):
        super().run()
        print(92)

    ...


# 多态的使用：len()

len('asdfasdf')
len([1, 2, 3, 4])
len({1: 1, 2: 2})
# len 不管是 字符串还是列表还是字典都可以直接使用
# 在python中 全是对象，
# 所以字符串、列表、字典中 都有一个 .__len__ 的方法
print('asdfasdf'.__len__())
print([1, 2, 3, 4].__len__())
# 所以len() 统一了 这些对象的使用方法


# 在python中 推崇的多态不是 继承
# python 大多用规范来限制你，不会强制你
# 如果类都按照规定创建了相对应的方法，就不用在用父类限制了
# 继承实际上就是一种耦合思想   如果不继承 实际上完成的就是一种 解耦合
# 这就是python推崇的 鸭子类型
# 看着像鸭子，走路像鸭子，声音像鸭子，那就是鸭子
# 也就是说，如果各个类都内封了该方法，就不需要规定父类来限制子类

# 抽象基类
import abc  # 全称 abstractmethod


class Car(metaclass=abc.ABCMeta):
    @abc.abstractmethod  # 装饰了这个装饰器，如果实例子类后，子类没有重写这个方法，就会报错
    def run(self):
        print('开始跑')
    # 抽象基类 不能直接实例化


class Benz(Car):
    ...


class Lx(Car):
    ...


class Auto(Car):
    ...


# 类方法
# 场景介绍
# 假设登录场景
class Mysql:
    def __init__(self, ip, prot):
        self.ip = ip
        self.prot = prot


obj = Mysql('127.0.0.1', 3306)


# 如果要经常实例这个类，且登录信息是相同的，且登录信息存储在 settings.py 文件中
class Mysql:
    # 假设IP 、 PROT 是导入过来的配置信息
    IP = '127.0.0.1'
    PROT = 3306

    def __init__(self, ip, prot):
        self.ip = ip
        self.prot = prot

    def init(self):
        return Mysql(Mysql.IP, Mysql.PROT)


obj = Mysql('127.0.0.1', 3306)


# 但是这个类 后续可能需要修改名字，修改名字的话，这种方法 也需要跟着修改
# 这就需要 类方法，绑定类的方法，当调用方法的时候 自动把当前类，按照第一个参数传递进来

class Mysql:
    IP = '127.0.0.1'
    PROT = 3306

    def __init__(self, ip, prot):
        self.ip = ip
        self.prot = prot

    @classmethod
    def init(cls):  # cls 代表的就是当前类
        return cls(cls.IP, cls.PROT)

    @staticmethod  # 静态方法，该方法会封装到类里，但是不会需要对象参数，是作为一个独立的函数来使用的
    def f2():
        print(123)


# 反射机制  动态语言 一定会有反射机制
# 在程序运行中，动态获取对象信息，动态调用对象方法的功能
# 通过字符串的形式，来修改操作 对象的属性
hasattr()  # 判断对象是否有该属性
getattr()  # 获取该属性
setattr()  # 给该属性赋值
delattr()  # 删除该属性


class Obj:
    def __int__(self):
        self.name = 'heih'
        self.age = 18

    def f1(self):
        print('123123123')


obj = Obj()
if hasattr(obj, 'name'):  # 直接返回 True False
    getattr(obj, 'f1')()  # 返回的是内存地址，如果是方法，需要调用的话，需要在后面 + () 调用一下
    setattr(obj, 'name', 'lixiaolu')
    print(obj.__dict__)
    delattr(obj, 'age')
    print(dir(obj))  # dir 会返回obj中 所有的属性
    getattr(obj, '123123', None)  # 如果没有对应的属性，就会返回第三个值


# 反射案例：
class Ftp(object):
    def put(self):
        print(11111111111111)

    def get(self):
        print(222222222222)

    def incomplete(self):
        option = input('>>>')
        getattr(self, option, None)





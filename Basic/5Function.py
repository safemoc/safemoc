import this

# 名称空间 | 作用域
# 对于变量与变量名
# 值存在堆区 变量名与变量值 的对应关系 存在栈区
# 名称空间 就是 对栈区进行了分类  -- ：内置名称空间 、全局名称空间 、局部名称空间

# 作用域：全局作用域 、 局部作用域

# 内置名称空间: 全局作用域 Python内置的名字
print  # 函数名和变量名一样都是名字 函数名也是存在栈区 对应函数在堆区里的内存地址  类名 模块名 都是名字
input


# 执行python 运行过程
# 1：Python解释器启动   --> 这一步就会创建内置名称空间 Python解释器一关闭 内置空间就会销毁
# 2：把文本内容读入内存
# 3：开始识别Python语法 --> 识别语法就会产生 变量名 函数名 模块名（import 导入的就是模块）

# 全局名称空间: 全局作用域  Python文件（模块）内定义的变量名，包括函数名、类名、模块名
# 全局名称空间可以有多个
# 要排除 函数内部定义的名字
# 总结： 只要不是函数内部定义的名字，也不是内置的名字，剩下的就属于全局名称空间的名字
# 全局名称空间 会在python文件之前产生 python 文件运行完毕 销毁

# 局部名称空间: 局部作用域  函数内部定义的名字 可以有多个
def func(x, y):  # x,y,z 都是内部名称空间的名字
    z = 1


# 在函数调用时产生，函数调用结束后销毁

# 名称空间的加载顺序
# 1：程序启动 创建内置名称空间
import os  # 2 导入os模块 将os 加入 全局名称空间
import time  # 2 导入time模块 将time 加入 全局名称空间


def func(x, z, y):  # 2 将func 加入全局名称空间 函数内部直接跳过，只要没有语法错误就不会报错
    ...  # 函数的x 与全局名称空间的x 名字相同 但是名称空间不同，所以不会互相冲突


a = 1  # 2  将a 加入全局名称空间
x = 2  # 2  将x 加入全局名称空间
func()  # 3 执行func 创建 局部名称空间 将func 的定义的名字加入局部空间
func()  # 3 此时上个函数调用完毕，上个局部空间已经被销毁 这个函数将会重新创建局部空间，并加载函数的变量名

# 名称空间的查找优先级：
# 局部>全局>内置
# 如果在函数内部的话,调用一个变量获取其内值 会优先在局部空间内寻找名字 未找到 会跳转到 全局空间 未找到 会跳转到 内置名称空间
# 如果在全局 就会在全局查找,未找到 会跳转到内置, 不会进入局部


# 查找顺序是按照定义阶段为基准:
x = 10


def func1():
    print(x)


def func2():
    x = 20
    func1()


func2()

# 结果会打印 10


# 函数嵌套 局部名称空间也会嵌套 所以
i = 1


def function():
    i = 10

    def func():
        print(i)


# 结果 10

# ----------------------------------------------------------------------------------------------------------------------
input = 10


def func1():
    def func2():
        # input=30
        print(input)

    func2()
    input = 20


func1()
# 结果报错:
# 因为名称查找顺序是以定义阶段为准的  定义函数不会执行子代码只会检查代码语法   此时func1 内部是有input 的但是在func2执行的时候 input 有名字但是没有赋值所以报错


# 局部空间
# L E G B
# L: local       本地
# E: enclosing   封闭 -- 代表函数 嵌套中的 父级函数
# G: global      全局
# B: built-in    内置


# global
# 作用 主要针对 局部想要修改全局的名字对应的不可变类型的值
# 可变类型 是可以从原值上进行修改,而不可变是 创造一个新的值
# 例如:
i = 10
l = [1, 2, 3]


def func():
    global i
    i = 20
    l.append(4)


func()

# nonlocal
# 作用: 不修改全局但是修改父级名字对应的不可变类型的值
i = 10


def func():
    i = 20
    x = 30

    def func():
        nonlocal i
        i = 30

        def func():
            nonlocal x  # nonlocal 不会进入全局 只会 在 LEGB 中的E层

    func()
    print(i)


# 函数传递 == 应用： 电子钱包

def login():
    print('login')


def scan():
    print('扫码')


def transfer():
    print('转账')


def query():
    print('查询')


func_dic = {'0': (None, '退出'),
            '1': (login, 'login'),
            '2': (scan, '扫码'),
            '3': (transfer, '转账'),
            '4': (query, '查询')}
# 为了避免 过多的if elif  将 对应的功能封装在 字典中，传入对应的值 直接 执行我们的程序
while 1:
    for i in func_dic.keys():
        print(f'{i} {func_dic[i][1]}')
    opt = input('请输入功能编号：')
    if opt == '0':
        break
    if opt not in func_dic.keys():
        print('功能不存在')
        continue
    func_dic[opt][0]()
    time.sleep(2)


# 闭包函数:
# 闭函数: 被封闭起来的函数 --> 不存在于全局
# 包函数: 函数内部包含对外层函数作用域名字的引用 --> 外层函数 不包括全局
def f1():
    x = 10

    def f2():
        print(x)

    return f2


x = 20  # 全局变量不会影响 局部变量
res = f1()  # 此时 res 的结果就是 f2 函数的内存地址 res() 就是等同于 f2()

# 此时就可以 在全局使用 局部函数,且全局变量不会对 函数有任何影响


# 装饰器 ： 为其他事物添加额外的功能
# 定义一个函数 这个函数是用来给其他函数添加额外功能
# 开放封闭原则
# 开放：对扩展功能（增加功能）开放，在源码不做任何更改的情况下，为其增加新功能
# 封闭：对修改源码是封闭的

# 装饰器 ： 不修改被装饰对象的源代码，也不修改调用方式的前提下，给被装饰对象添加新的功能
import time


def inside(group, s):
    print(f'{group}')
    print(f'{s}')
    time.sleep(3)
    print('ok')


def outer(func):
    def wrapper(*args, **kwargs):
        _ = time.time()
        func(*args, **kwargs)
        print(time.time() - _)

    return wrapper


inside = outer


# 实现 充电

def outer(func):
    def wrapper(*args, **kwargs):
        _ = time.time()
        response = func(*args, **kwargs)
        print(time.time() - _)
        return response

    return wrapper


@outer  # 语法糖 实际实现了： recharge = outer(recharge)
def recharge(num):
    for i in range(num, 101):
        time.sleep(0.09)
        print(f'\rnow:{i}%', end='')  # 每次回到行首，且默认取消换行符
    return 100


# 但是 这样并不完美，recharge 的内存地址 以及一系列属性 是 装饰器内部的闭包的属性
# 完美伪装：
from functools import wraps


def outer(func):
    @wraps(func)  # 使用python提供的 func 工具 实现完美伪装
    def wrapper(*args, **kwargs):
        response = func(*args, **kwargs)
        return response

    return wrapper


@outer
def recharge(num):
    for i in range(num, 101):
        time.sleep(0.09)
        print(f'\rnow:{i}%', end='')  # 每次回到行首，且默认取消换行符
    return 100


print(recharge.__name__)


# 有参装饰器： 在原有的 装饰器上再次封装 并声明需要的参数 将原有装饰器 return 出来
def g_outer(name):
    def outer(func):
        def wrapper(*args, **kwargs):
            response = func(*args, **kwargs)
            return response

        return wrapper

    return outer


def auth(source):
    def file_user():
        ...

    def mysql_user():
        ...

    dic = {'file':
               file_user,
           'mysql': mysql_user,
           }
    if source not in dic.keys():
        raise PermissionError
    user_information = dic[source]

    def _auth(func):
        def wrapper(*args, **kwargs):
            name, password = input(), input()
            if user_information.name == name and user_information.password == password:
                response = func(*args, **kwargs)
            else:
                response = None
            return response

        return wrapper

    return _auth


@auth('file')
def recharge(num):
    for i in range(num, 101):
        time.sleep(0.09)
        print(f'\rnow:{i}%', end='')  # 每次回到行首，且默认取消换行符
    return True

# 装饰器 叠加 实际上就是 将已有函数 进行多次封装

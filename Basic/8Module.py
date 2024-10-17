# !/usr/bin/python #通常只在unix环境中有效,作用是指定解释器路径,然后可以直接使用脚本名执行,不需要在前面调用解释器
# coding: utf-8
# Author: @safemoc
"""
模块的文档描述
"""
import time  # 导入模块

name = 'safemoc'  # 定义全局变量;如果不是必须的最好使用局部变量,这样可以提高代码的维护性,同时节省内存提高性能


class Test:  # 定义类
    """
    类的注释
    """
    ...


def func():
    """
    函数的注释
    :return:
    """
    ...


if __name__ == '__main__':
    '''
    主程序 主要用来测试功能是否完善
    '''

# 以上就是模块的模板


# 模块
# 模块就是一系列功能的集合体
# 通过import 接入数据
# 通过 . 的方式 调用下面的功能
# 分类:
'''
python内置模块  -- 一般都是用c/c++写的
第三方模块
自定义模块 可以使用python|c|c++
'''
# 模块的回收方式：
# 当导入一个模块的时候 实际上就是 声明了一个变量 引用的是 对应模块的空间
'''
如果要把一个文件夹作为一个模块 
那么这个文件夹内必须要有 __init__.py 的文件  
这种情况也可以称之为 包
'''

# 在首次导入的时候 会立即执行 对应模块的文件 只在首次


# 如果 import 了一个模块中的 name  然后重新定义了 name 那么当前名称空间内的name 会修改， 但是模块中的name 是没有改动的
# 被导入的模块都有一个 __all__  它的值是一个列表,列表内是字符串格式的名字,名字就是 模块中有的名字
# 当 from module import * 的时候 实际上就是 查找__all__内的所有名字
# 如果自己定义了__all__ 那么就只会 导入 __all__ 里有的名字


# 循环导入问题
'''
modulea:
from moduleb import y
print(y)
x= 1

moduleb:
from modulea import x
print(x)
'''
# 此时 如果 导入modulea 那么就会执行a的代码 执行到 from moduleb import y 的时候就会 执行 moduleb 的代码
# moduleb 的代码 第一行就是 导入modulea ,此时内存中已经有 modulea 的名称空间,就不会再执行 modulea的代码了
# 但是 a 的代码停止在 from moduleb import y  阶段  变量x 未定义  但是moduleb 需要这个变量,就会报错

# 模块的查找顺序
import sys

sys.path
# 1:内存 先查看内存中是否已经有对应的名称空间可以导入
# 2:硬盘 从当前执行文件中查找 如果没有 就从 sys.path 中按照顺序 依次查找
# pycharm 会自动将项目路径 添加到 sys.path中 但是服务器中一般不会
# 第一个路径是执行文件夹所在的路径
# python 安装目录下的 lib 文件夹下 是python 内置的模块   lib/sith-packages 就是安装的第三方库


# sys.path 的应用
# 可以将需要导入的包的路径 添加到 sys.path
# 这个操作只是临时有效,程序结束后


# 包: 就是一个含有 __init__.py 的文件夹

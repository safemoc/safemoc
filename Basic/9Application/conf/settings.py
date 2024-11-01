# coding: utf-8
# Author: @safemoc
import os

os.path.dirname(__file__)  # 获得当前执行文件的父级目录  --> bin
# 对上个结果 嵌套一层就可以获取 父级目录的 父级目录 的路径 也就是项目路径
PATH = os.path.dirname(os.path.dirname(__file__))  # --> Application   这种方法看着很复杂,但是python2 同样兼容

LOG_PATH = fr'{PATH}\log\user.log'

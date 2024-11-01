# coding: utf-8
# Author: @safemoc
import time
from lib.comment import logger


def login():
    print('login'.center(30, '*'))
    logger('login')
    ...


def register():
    print('注册功能'.center(30, '*'))
    ...


def recharge():
    print('充值功能'.center(30, "*"))
    ...


def transfer():
    print('转账'.center(30, '*'))
    ...


func_dic = {'0': ('退出', exit),
            '1': ('login', login),
            '2': ('注册', register,),
            '3': ('充值', recharge,),
            '4': ('转账', transfer,)}


def main():
    while 1:
        for i in func_dic:
            print(f'{i} {func_dic[i][0]}')
        opt = input('请输入功能编号：')
        if opt not in func_dic.keys():
            print('\033[33m功能不存在 \033[0m')
            continue
        time.sleep(3)
        func_dic[opt][1]()

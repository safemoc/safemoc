# !/usr/bin/python #通常只在unix环境中有效,作用是指定解释器路径,然后可以直接使用脚本名执行,不需要在前面调用解释器
# coding: utf-8
# Author: @safemoc
"""
模块的文档描述
"""
import time

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
# __init__.py 就是导包 的时候 需要执行的python文件
from Basic import pack

# 在python3 中如果文件夹中没有__init__.py 也是可以执行的 只不过 调用不出来任何东西罢了
# 在python2中 就会报错
pack.chat()
# 后续文件代码量上升,维护繁琐 ,就需要进行拆分文件 进行更新
# 不管如何更新模块 都需要 使 使用者按照最原始的用法来用
# 从最开始的一个文件 拆分为多个文件 然后组合为包
# 需要在init.py 文件中将这些 拆分后的文件都导入进来


# 在导入的时候 . 的左边只能是包
# a 顶级包 b子包 c模块 imoprt 变量/函数/类


# 软件目录规范
# 1: bin : 项目的可执行文件
# 2: conf: 项目配置信息
# 3: lib : 常用的文件/包
# 4: core: 核心代码逻辑 一般从bin中的run 文件调用,run 只管执行程序
# 5: log : 项目日志
# 6: db  : 数据库相关操作
# 7: api : 项目的一些接口
# 8: requirement.txt : 项目中的依赖包 等环境信息
# 9: readme.md: 项目的文档介绍


# 时间模块 --> time  | datetime
# 时间有三种形式：
# 1： 时间戳
time.time()  # 获取当前的时间戳
# 2: 格式化的字符串形式
time.strftime('%Y-%m-%d %H:%M:%S %A')  # 年-月-日 时-分-秒 星期几
time.strftime('%Y-%m-%d %X %A')  # 年-月-日 时-分-秒 星期几
time.strftime('%x %X %A')  # 日/月/年 时-分-秒 星期几
# 3: 结构化时间
time.localtime()
# time.struct_time
# (tm_year=2024, tm_mon=10, tm_mday=18, tm_hour=15, tm_min=32, tm_sec=29, tm_wday=4, tm_yday=292,         tm_isdst=0)
#          年          月         日          小时         分钟          秒   本月第几周  当前年已过多少天       是否夏令时
res = time.localtime()
res.tm_year  # 年
res.tm_hour  # 小时

import datetime

datetime.datetime.now()  # 直接就是格式化的 %Y-%m-%d %H:%M:%S.微秒
# 不想要微秒的话
datetime.datetime.now().replace(microsecond=0)  # 这个replace不是 str的replace 而是datetime中自己的replace
# datetime类型可以直接进行时间的加减
datetime.datetime.now() + datetime.timedelta(days=7)  # 当前时间+7天 如果要-7天 直接传-7 就可以 或者 将 运算符修改为 -号
datetime.datetime.now() + datetime.timedelta(weeks=7)  # 当前时间+7周
datetime.datetime.now() + datetime.timedelta(hours=7)  # 当前时间+7小时

# 时间戳 <----> 结构化时间  <-----> 格式化字符串时间
# 时间错 <!-----!> 格式化字符串时间

# 时间戳--localtime|gmtime--> 结构化时间：
timestamp = time.time()
time.localtime(timestamp)  # 将时间戳传给 localtime 函数就是转换为了结构化时间， 不传参数就是 当前时间的结构化
time.gmtime()  # 获取世界标准时间 # 零时区的时间

# 结构化时间 --strftime --> 格式化字符串时间
time.strftime('%Y-%m-%d %H:%M:%S %A', time.localtime())
# 格式，结构化时间
# 第二个参数如果没传值 默认时间就是 time.localtime


# 格式化字符串时间 --strptime--> 结构化时间
time.strptime('1982-08-29 01:11:01', "%Y-%m-%d %x")

# 结构化时间 --mktime--> 时间戳
time.mktime(111111111111)

# other time app
time.asctime()  # 与strftime 功能相同 但是不需要格式 有自己默认的格式  Linux常用的时间格式
time.ctime(time.time())  # 直接把时间戳转为字符串格式，但是不支持指定格式 Linux常用的时间格式

datetime.datetime.utcnow()  # 零时区的标准时间
datetime.datetime.fromtimestamp()  # 直接将时间戳 转换为 字符串格式的日期 yyyy-mm-dd HH:MM:SS 不支持指定格式

# 随机
import random

random.random()  # 随机一个不大于1 的小数
random.uniform(1, 10)  # 需要一个范围 从范围内 随机一个小数
random.randint(1, 23)  # 需要两个值 从范围内随机一个整数  闭区间 两个值都包含在内
random.randrange(1, 10)  # 与randint相同  左闭右开  不包含 stop
random.choice()  # 需要一个序列 随机一个元素
random.sample()  # 与 choice 类似 但是需要两个参数，第二个是选择的个数  返回的是一个列表
random.shuffle()  # 需要一个可变有序类型 打乱顺序

# 随机密码
# 小写字母 |大写字母 |数字|符号
pwd = ''
for _ in range(16):
    char_list = [[97, 122],  # 小写字母 对应的 ASCII码 位数
                 [65, 90],  # 大写字母 对应的 ASCII码 位数
                 [48, 57],  # 数字 对应的 ASCII码 位数
                 [33, 47]]  # 符号 对应的 ASCII码 位数
    random_list = random.choice(char_list)
    random_chr = chr(random.randint(*random_list))
    pwd += random_chr
print(pwd)


def pwd_generator(length=16):
    _p = []
    for _ in range(length // 4 + 1):
        _chr: list = [
            chr(random.randint(97, 122)),
            chr(random.randint(65, 90)),
            chr(random.randint(48, 57)),
            chr(random.randint(33, 47)),
        ]
        _p.extend(_chr)
    random.shuffle(_p)
    return _p[:length]


# os   operating system
import os

os.getcwd()  # 获取当前工作目录
os.chdir('dirname')  # 等同 cmd 执行 cd 命令
'''
执行完 chdir 之后 工作目录就会切换
'''
os.listdir()  # 获取指定目录下的所有文件/文件夹  包括隐藏文件，并返回列表
os.mkdir()  # 创建一个文件夹
os.makedirs()  # 可以创建多层文件夹
os.remove()  # 删除文件
os.mkdir()  # 删除单级空目录，若目录不为空则无法删除，报错
os.rename()  # 重命名文件/目录
os.system()  # 执行终端命令
# pycharm 默认编码是 utf-8  windows系统默认编码是gbk


os.environ  # 获取系统环境变量
os.environ.get('KEY')  # 获取系统环境变量中的某一个值
os.getenv('key')  # 获取系统中环境变量的某一个值
os.stat('path/filename')  # 获取文件/目录信息

os.name  # 获取当前系统
# window 是nt  Linux 是 posix


os.path.split('path')  # 将path分割成目录和文件名 ，返回元组
os.path.dirname('path')  # 返回当前路径的父级目录，也就是 os.path.split() 元素的第一个元素
os.path.basename('path')  # 返回path最后的文件名， 也就是os.path.split() 的第二个元素
os.path.exists('path')  # 如果path 存在返回Ture 否则返回False
os.path.isabs('path')  # 如果path是绝对路径 返回Ture
os.path.isfile('path')  # 如果path 是一个存在的文件返回Ture 否则返回False
os.path.isdir('path')  # 如果path 是一个存在的目录 返回Ture 否则 False
os.path.join('path1', 'path2'
             # [,...]
             )  # 将多个路径组合后返回，第一个绝对路径之前的参数将被忽略
os.path.getatime()  # 返回path所指向的文件或目录的最后读取时间
os.path.getmtime()  # 返回path所指向的文件或目录的最后修改时间
os.path.getctime()  # 返回path所指向的文件或目录的最后创建时间 （windowns平台中 ）
os.path.getsize()  # 返回path 的大小

# sys
sys.path
sys.argv  # 获取用户在终端传的参数

path1 = r'd:'
path2 = r'd:'
# shutil

import shutil

shutil.copystat(path1, path2)
# 将path1的文件信息 copy 给path2   如果 上次读取时间，上次修改时间，创建时间等，path2 文件必须存在

shutil.copymode(path1, path2)
# 拷贝权限


shutil.copyfileobj(open(path1, 'r'), open(path2, 'w'))  # 拷贝文件 通过文件对象
shutil.copyfile(path1, path2)  # 直接通过路径拷贝

shutil.copy(path1, path2)  # 拷贝文件以及权限
shutil.copy2(path1, path2)  # 拷贝文件以及状态信息

# 拷贝文件夹
# 将path1 copy到path2  ignore指定排除文件   *.py 就是不包括结尾为.py 的文件   user* 就是user开头的文件
shutil.copytree(path1, path2, ignore=shutil.ignore_patterns('*.py', 'user*'))

shutil.move()  # 移动文件夹

shutil.make_archive(path1, 'zip', root_dir=path2)  # 创建压缩包
# 第一个参数就是 要压缩的文件路径 第二个是压缩类型 rootdir 是压缩后压缩包存放的位置
# 压缩类型有： zip tar bztar gztar


# zipfile
import zipfile

# 压缩
zf = zipfile.ZipFile('path.zip', 'w')  # 打开一个压缩包
zf.write(path1)  # 需要加到压缩包的文件
zf.close()  # 关闭压缩包
# 解压
zf = zipfile.ZipFile('path.zip', 'r')  # 打开一个压缩包
zf.extractall(path='test')  # 解压 包内所有文件，指定解压路径
zf.close()

# tarfile
import tarfile

# 压缩
tar = tarfile.open('path.tar', 'w')  # 打开压缩包
tar.add(path1, arcname='a.txt')  # 将文件添加到压缩包并命名
tar.close()  # 关闭

# 解压
tar = tarfile.open('path.tar', 'r')  # 打开压缩包
tar.extractall(path=path2)  # 解压包内所有文件，并指定解压路径
tar.close()

# 序列化 反序列化
# 序列化：
# 把内存中的数据类型转成一种特定的格式，这种特定的格式可以用于存储，或者传输给其他平台

# 内存中的数据 ---> 序列化 -----> 特定格式(json/pickle)
# 内存中的数据 <--- 反序列化 <-----特定格式(json/pickle)

# 用途?
# 1 存档
# 2 跨平台数据交互 (只能用json) pickle是Python专用的格式
# pickle 满足了python所有数据类型,Json无法满足Python的集合

# json
import json

dic = {'name': 'A', 'age': 18, 'salary': 3.5, 'married': False,

       # 'book':{1,2,3,4}  # json 没有集合就会报错
       }
res = json.dumps(dic,
                 ensure_ascii=False  # 这个参数为False 时就不会对汉字进行Unicode编码
                 )  # 如果dumps 的数据有汉字 就会转成 Unicode编码
# 序列化后会把所有数据转成 ASCII码可以表示的数据,中文没有ASCII 所以就会转义为 Unicode
# json 是一种字符串

# pickle
import pickle

# 与json类似
res = pickle.dumps(dic, protocol=0)

# 猴子补丁 : 用自己的代码替换所用的源代码
# 利用import 优先导入内存的特性 ,在项目启动文件中,优先导入模块,然后修改对应模块的功能


# configparser : 加载特定的 setting文件
# 部分项目的配置文件 以 .ini 或者 .cfg 结尾
# 9Application/conf/conf.ini
import configparser

config = configparser.ConfigParser()
config.read('./9Application/conf/conf.ini', encoding='utf-8')
print(config.sections())  # 获取所有的section
config.options('section_name')  # 获取对应section下的所有option
config.items('section_name')  # 返回一个列表 列表中为元组 0key 1value
config.get('section_name', 'option_name')  # 获取对应的值 获取到的是字符串
config.getint('section_name', 'option_name')  # 获取的就直接就是int 值
config.getfloat('section_name', 'option_name')  # 获取的就直接就是float 值
config.getboolean('section_name', 'option_name')  # 获取的就直接就是bool值

# subprocess  : 执行终端命令
import subprocess

obj = subprocess.Popen('dir',  # 操作命令
                       shell=True,  # 代表终端 -->表示命令用 shell 来执行
                       stdout=subprocess.PIPE,  # 命令返回结果 返回给 PIPE管道
                       stderr=subprocess.PIPE  # 命令报错的结果 返回给 PIPE管道

                       )
resout = obj.stdout.read()  # 查看 stdout 管道
reserr = obj.stderr.read()  # 查看 stderr 管道

print(resout.decode('gbk'));  # 输出结果为操作系统编码的 bytes字节流  需要对应的解码，Windows系统编码是 gbk
reserr.decode('gbk')

obj = subprocess.Popen('dir & aaajsdfl',  # 操作命令
                       shell=True,  # 代表终端 -->表示命令用 shell 来执行
                       stdout=subprocess.PIPE,  # 命令返回结果 返回给 PIPE管道
                       stderr=subprocess.PIPE  # 命令报错的结果 返回给 PIPE管道

                       )
resout = obj.stdout.read()  # 查看 stdout 管道
reserr = obj.stderr.read()  # 查看 stderr 管道

print(resout.decode('gbk'))  # 输出结果为操作系统编码的 bytes字节流  需要对应的解码，Windows系统编码是 gbk
print(reserr.decode('gbk'))  # 正确管道和错误管道可以同时存在

# hash # 一类算法，有很多中
# 通过对应的计算 会获得一串 哈希值 | 有人叫  散列值
# 常见的hash 算法： md5  sha1 sha256 sha521
# 目前 md5 sha1 已经被破解

# hash 特点：
# 1输入敏感 ，哪怕只修改了 一个标点 hash 都会发生巨大的变化  如果输入相同 hash算法相同，得到的结果也会相同
# 2不可逆  无法通过hash 反推数据
# 3计算极快而长度固定  10个G与1kb 的计算时间 都是 0.1秒   # 不管 多胖骨头多硬 对火葬场来说化成灰都只是一把火的事情  hash算法相同长度也就相同

# 用途
# 1：密码加密 # 防止密码被 截包    | 因为hash 不可逆 所以服务端存储的是密码加密后的hash值 然后通过相同的算法计算后 比对结果是否相同来判断密码是否相同
# 2: 文件完整性校验


# 因为hash 长度是固定的,说明hash值数量是有限的 而输入的数据有无穷个 无法建立 1对1 必然会发生碰撞  所以 当计算速度足够快,是可以撞库的
# 撞库: 密码库中有所有基本密码组合的hash 然后 截包 获取你的登录 hash密文 然后 进行对比就可以了  没有绝对安全,只能提升攻击成本


# 使用
import hashlib

h1 = hashlib.md5()
h1.update('abc'.encode('utf-8'))  # 必须接收一个 bytes类型数据
h1.update('abc'.encode('utf-8'))  # 必须接收一个 bytes类型数据
h1.update('abc'.encode('utf-8'))  # 必须接收一个 bytes类型数据
h1.hexdigest()  # 拿到 以前传的所有的数据的hash 值

# 文件校验
with open('8Module.py', mode='rb', encoding='utf-8') as f:
    h1 = hashlib.md5(f.read())
    print(h1.hexdigest())  # 比对结果与公布结果
    ...
# 对于大文件 不可以使用以上方法
# 可以使用 for 循环对 文件指针进行操作, : seek() 方法
# 选取一个 切片,然后对切片 部分数据进行比对 hash

file_path = r'a/b/b/c/t.txt'
h1 = hashlib.md5()
h2 = hashlib.md5()
with open(file_path, mode='rb', encoding='utf-8') as f1, open(file_path, mode='rb', encoding='utf-8') as f2:
    f1.seek(0, 2)
    f2.seek(0, 2)
    size_f1 = f1.tell()
    size_f2 = f2.tell()
    if size_f1 == size_f2:
        one_tenth = size_f1 // 10
        for i in range(10):
            f1.seek(i * one_tenth, 0)
            f2.seek(i * one_tenth, 0)
            if h1.update(f1.read(100)) == h2.update(f2.read(100)):
                ...
            else:
                break

# 密码加盐
# 对密码明文进行 操作,在字符中间 添加特定的字符来避免 撞库


# 正则
...

# logger
# logger 模块中 日志分为5个等级
import logging

logging.debug('调试日志')
logging.info('消息日志')
logging.warning('警告日志')
logging.error('错误日志')
logging.critical('严重错误日志')

# 日志输出格式: 级别:日志名:日志信息
# 默认日志输出级别是 warning 默认日志名是 root

# logging 的基本配置
logging.basicConfig(
    level=10,
    # debug=10
    # info = 20
    # warning = 30
    # error = 40
    # critical = 50
    format='%(asctime)s %(name)s [%(pathname)s line:%(lineno)d] %(levelname)s %(message)s',
    # asctime : 当前时间
    # name : 当前日志名
    # pathname : 哪个文件产生的这个日志
    # lineno : 哪一行代码出现的这个日志
    # levelname: 日志等级
    # message : 日志内容信息
    datefmt='%Y-%m-%d %H:%M:%S',
    # 修改 asctime的时间格式
    filename='user.log',
    # 日志输出位置, 终端/文件
    # 不指定此配置 默认输出终端

)

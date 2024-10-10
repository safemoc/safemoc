# 首先了解编码
# 字符编码
# 任何程序都是把 数据从硬盘提取到内存中
# 程序运行中产生的数据都是存入内存中的
# 操作文件就是使用文本编辑器读取文件
#  1 启动文本编辑器|2把文件从硬盘读取到内存中|3把数据显示在屏幕上
# 执行python程序逻辑:
# 1 启动python解释器|2把源代码从硬盘读入内存|把文件内容当成Python语法进行识别
# 因为 存储在硬盘中的所有数据都是 0-1  所以需要有一个对照表
# 字符编码表
# 假设 下 --> 48857
# 那么 我们的文本 中的下 就会 转换为 48857 然后转为二进制存储
# ASCII表 --英文以及一些通用符号以及拉丁文部分符号
# 内存中 8位
# GBK  -- 中文以及英文
# 内存中 英文8位 中文16位
# Shift_JIS --日文
# Euc-kr 韩文
# unicode 万国码 -- 16位

# 1bit -->1位二进制数
# 8bit -->1Bytes(字节)
# 1KB  -->1024Bytes
# 1MB  -->1024KB
# 1GB  -->1024MB
# 1TB  -->1024GB
# 运营商 通常使用 Mbps(兆比特每秒)
# 这个           M 表示100万
# 下载速度       MBps(兆字节每秒)


# I/O
# 因为CPU执行速度 > 内存>硬盘
# 每次执行程序 如果要从硬盘获取数据 会导致 执行时间被拖慢,所以要减少I/O 与硬盘的联系
# 固态硬盘
# 内部是由 浮栅晶体管组成的每个都对应一个存储单元 在每个单元中,通过输入不同的电子数量 来改变每个单元的导电性能 来实现数据的读写
# 简单说 写就是改变浮栅晶体管的电子数目的过程  读就是想晶体管施加电压获取不同导电状态来识别数据的过程 所以在读取上固态会省去非常多的时间
# 当你想要读写的时候 固态会把你的读写指令翻译位地址码 然后定位进行读写 对比机械硬盘 物理转速无法对标电信号
# 内存的I/O延迟是纳秒级别的 固态的I/O是毫秒级别的 中间差几个量级
# 固态硬盘写入量是有限制的 因为在写入数据的时候,固态并不直接覆盖原有数据而是先进行擦除再写入 因为这一步导致写入仅有读取的 1/2 到 1/4
# 当固态中的晶体进行擦除的时候,浮栅晶体管中会留下一些电荷从而改变电阻,对固态有一定的伤害


# 因为使用 unicode会导致I/O增多的问题,
# UTF-8 因此产生  UTF-8 对应为字符进行了空间上的优化,英文只占用1个字节 中文占用3个甚至4个以及更多个

#     操作           内存              硬盘
# 编码:字符 --编码-->unicode --编码-->UTF-8\GBK...
# 解码:字符 <--解码--unicode <--解码--UTF-8\GBK...
# 在内存中都是按照 unicode 编码存储的,但是存入硬盘时,使用那种编码进行存储可以自定义 推荐使用 UTF-8
# 为什么在内存中 不能使用UTF-8?
# 因为unicode 兼容以前的各种其他编码 UTF-8就不支持 UTF-8 仅仅是优化了unicode
# 指定python解释器按照特定的编码集进行解码时: 在代码首行 写入: -- 存的编码要与指定的编码相同
# coding:gbk |ASCII


# encode 编码 : 从Unicode 改为其他编码格式
# decode 解码: 从其他编码格式改为 Unicode


# 打开文件  open
file = open('../Data/file1.txt')
# 关闭文件
file.close()

# mode  r|w|a    b模式 t模式 默认时t模式
# r模式   |rt
# 只读模式
with open(u'../Data/file1.txt', mode='rt', encoding='utf-8') as f:
    ...
    # 文件不存在报错，文件存在的话 文件指针会在开始位置
    f.read()
    # 实际就是从文件开始位置读取到文件结束，此时 文件指针在 文件末尾
    f.read()
    # 这时候在读取数据的话，因为文件指针已经在末尾 无法读取到数据

# 登录程序
name = input('请输入你的用户名：').strip()
word = input('请输入你的密码：').strip()
with open('Data/user.txt', mode='rt', encoding='utf-8') as f:
    for i in f:
        username, password = i.strip('\n').split('--')
        if username == name and password == word:
            print(1111)
            break
        else:
            ...
    else:
        print('zhanghao bucunzai huozhe shi zhanghao mima cuowu ')

# w 模式 只写模式
with open('Data/test.txt', mode='wt', encoding='utf-8') as f:
    # 当文件存在时，清空内容，文件不存在时 创建文件
    f.write('123\n')  # 写入数据
    f.write('123123')  # 这段不会清空上部写入的数据，因为 w只在文件打开时清空文件

# a 模式 追加写模式
# 当文件存在时，打开文件 文件指针自动跳到最后面，文件不存在时 创建文件

# 注册功能
username = input('zhanghao ')
password = input('mima ')
with open('Data/user.txt', mode='at', encoding='utf-8') as f:
    f.write(f"{username}--{password}\n")
    print('zhucechenggong ')

# 拷贝功能
with open('Data/user.txt', mode='rt', encoding='utf-8') as f1, open('Data/userfb.txt', mode='wt',
                                                                    encoding='utf-8') as f2:
    f2.write(f1.read())

# + 模式
# r+ 文件不存在 报错，可以写 在当前文件指针的位置开始写并覆盖同位数的数据
# w+ 以w为基础 可以 使用read 但是w会清空文件
# a+ 以a为基础可以使用 read  a打开文件指针在末尾 read 也是空的，要控制文件指针移动

# x模式  与w相似 但是文件不存在就创建文件 文件存在就报错


# b 模式
# 以 二进制的形式 打开文件 支持所有文件、图片、视频等文件 b模式下写操作不可以直接写入字符 需要指定编码集 进行编码 写入
# 拷贝功能优化针对所有文件--for
with open('Data/user.txt', mode='rb') as f1, \
        open('Data/userfb.txt', mode='wb') as f2:
    for line in f1:
        f2.write(line)

# 拷贝功能优化针对所有文件--while
with open('Data/user.txt', mode='rb') as f1, \
        open('Data/userfb.txt', mode='wb') as f2:
    while 1:
        res = f1.read(1024)
        if not res:
            break
        f2.write(res)

b''  # 如果字符串时纯英文和数字的话，可以这样写
# 如果字符转中有汉字的话， 可以尝试使用
bytes('', encoding='utf-8')
# 一般执行写操作，系统会等待需要写的量达到一定的阈值，然后再执行写操作，再python中有一个方法可以直接 跳过这种优化机制，进行写入
f.flush()
f.readable()  # 判断文件是否可读 返回bool
f.writable()  # 判断文件是否可写 返回bool
f.closed  # 判断文件是否关闭 # 返回bool
f.name  # 返回当前文件名
f.encoding  # 返回当前的编码格式 如果打开方式是b模式 则 没有该属性

# 移动 文件指针 移动单位是字节
# f.seek( n, 参照位置 )
# 模式 0
f.seek(5, 0)  #:移动5个字节 以开头为参照 此时文件指针停留在第5字节
f.seek(2, 0)  #:因为参照还是以开头为准，所以此时的文件指针停留在第2个字节

# 模式1
f.seek(5, 1)  # 以当前位置向后移动5个字节
f.seek(2, 1)  # 此时 就会根据上次移动后的位置再调整位置，文件指针停留再 7的位置

# 模式2
f.seek(5, 2)  # 以文件末尾为参照，向后移动，但是文件末尾再往后没有了，需要传递 负数 向前移动

f.tell()  # 获取当前的文件指针停留位置
# 文件监控系统
# with open('Data/user.log', mode='rb') as f:
#     f.seek(0, 2)  # 将文件指针跳转到文件末尾
#     while 1:
#         res = f.readline()  # 读取行信息
#         if res:  # 如果行有数据
#             print(res.decode('utf-8'), end='')
#         else:
#             ...

# while 1:
#     with open('Data/user.log', mode='at', encoding='utf-8') as f:
#         money = input('123')
#         f.write(f'{money}\n')

# 文件的修改
# a 模式 不管文件指针在哪，写入都会写入到最后
# 所以如果要在文本中间修改的话 就只能使用r+ 模式
# 第一种方式 将user.log 中的3 替换为你好  用户体验好
with open('Data/user.log', mode='r+', encoding='utf-8') as f:
    new = ''
    for line in f:
        new += line.replace('3', '你好')
with open('Data/user.log', mode='w', encoding='utf-8') as f:
    f.write(new)

# 第二种方式  资源占用少
with open('Data/user.log', mode='rt', encoding='utf-8') as f, \
        open('Data/.user.log.swap', mode='w', encoding='utf-8') as f1:  # 命名规范， .开头的文件表示临时文件 .swap 表示交换文件
    for line in f:
        f1.write(line.replace('你好', '3'))
import os

os.remove('Data/user.log')
os.rename('Data/.user.log.swap', 'Data/user.log')

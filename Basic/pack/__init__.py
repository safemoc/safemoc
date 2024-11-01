# coding: utf-8
# Author: @safemoc
print('hello world')

# 绝对导入
# 因为执行文件不是 __init__.py 所以需要 从 pack 开始导入
from Basic.pack.kanpsack import kanpsack

# 相对导入 # 包名修改 不会影响程序
# .:当前文件夹
# ..: 上一层文件夹  如果还需要往上一层 可以直接 在加 . 但是这个方法不能跳出 顶级包

from .chat import chat

# 相对导入不能跨出包 但是绝对导入可以
# 因为绝对导入是根据 sys.path 的值进行参照的
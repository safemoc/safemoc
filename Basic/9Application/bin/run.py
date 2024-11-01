# coding: utf-8
# Author: @safemoc
import sys
import os

from conf.settings import PATH
# 将结果 添加到 环境变量中
sys.path.append(PATH) # 将path 存储在 settings 中
from core.core import main

if __name__ == '__main__' :
    main()

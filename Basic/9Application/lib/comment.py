# coding: utf-8
# Author: @safemoc
import time
from conf.settings import LOG_PATH


def logger(msg):
    with open(fr'{LOG_PATH}', mode='at', encoding='utf-8') as f:
        f.write(f'{time.strftime("%Y-%m-%d %H:%M:%S")}==-->==-->{msg}\n')

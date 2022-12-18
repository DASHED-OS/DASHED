from pb_functions import *

signal_input = input()

while signal_input != q:
    print(f'You chose {signal_input}')
    print('::: Press Enter if you would like to continue')
    print()
    if signal_input == 'i' or signal_input == 'I':
        build_strategy()
    else:
        exit
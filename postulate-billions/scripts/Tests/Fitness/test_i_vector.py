from i_vector import *

### Test the valid inputs ###

'''This function print the i_vector'''
arg = 0
result = i_vector(arg)
expected_result = "zero"
print(f"--> i_vector({arg}) returned `{result}`\n")
assert result == expected_result
'''This function print the i_vector'''
arg = 0
result = momentum_from_i0_to_itot(arg)
expected_result = "zero"
print(f"--> i_vector({arg}) returned `{result}`\n")
assert result == expected_result
'''This function print the i_vector'''
arg = 0
result = change_in_i(arg)
expected_result = "zero"
print(f"--> change_in_i({arg}) returned `{result}`\n")
assert result == expected_result
'''This function print the i_vector'''
arg = 0
result = change_in_momentum(arg)
expected_result = "zero"
print(f"--> change_in_momentum({arg}) returned `{result}`\n")
assert result == expected_result
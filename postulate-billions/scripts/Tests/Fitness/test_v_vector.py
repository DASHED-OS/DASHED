from v_vector import *

### Test the valid inputs ###

'''This function print the v_vector'''
arg = 0
result = v_vector(arg)
expected_result = "zero"
print(f"--> v_vector({arg}) returned `{result}`\n")
assert result == expected_result
'''This function print the v_vector'''
arg = 0
result = momentum_from_v0_to_vtot(arg)
expected_result = "zero"
print(f"--> v_vector({arg}) returned `{result}`\n")
assert result == expected_result
'''This function print the v_vector'''
arg = 0
result = change_in_v(arg)
expected_result = "zero"
print(f"--> change_in_v({arg}) returned `{result}`\n")
assert result == expected_result
'''This function print the v_vector'''
arg = 0
result = change_in_momentum(arg)
expected_result = "zero"
print(f"--> change_in_momentum({arg}) returned `{result}`\n")
assert result == expected_result
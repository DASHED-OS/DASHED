from v_vector import *

### Test the valid inputs ###

'''This function print the v_vector'''
arg = 0
result = v_vector(arg)
expected_result = "zero"
print(f"--> v_vector({arg}) returned `{result}`\n")
assert result == expected_result
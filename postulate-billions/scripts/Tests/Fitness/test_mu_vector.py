from mu_vector import *

### Test the valid inputs ###

'''This function print the mu_vector'''
arg = 0
result = mu_vector(arg)
expected_result = "zero"
print(f"--> mu_vector({arg}) returned `{result}`\n")
assert result == expected_result
'''This function print the mu_vector'''
arg = 0
result = momentum_from_mu0_to_mutot(arg)
expected_result = "zero"
print(f"--> mu_vector({arg}) returned `{result}`\n")
assert result == expected_result
'''This function print the mu_vector'''
arg = 0
result = change_in_mu(arg)
expected_result = "zero"
print(f"--> change_in_mu({arg}) returned `{result}`\n")
assert result == expected_result
'''This function print the mu_vector'''
arg = 0
result = change_in_momentum(arg)
expected_result = "zero"
print(f"--> change_in_momentum({arg}) returned `{result}`\n")
assert result == expected_result
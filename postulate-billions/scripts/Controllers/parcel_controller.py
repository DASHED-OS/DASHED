def find_num_days(profitability):
    '''
    This function will allow us to estimate the number of days it will take 
    '''
    #Defining figures for the calculation
    desired_goal = 1000000000
    num_trades = 4469
    profit_per_parcel = profitbility
    # Create Parcel
    parcel = 3
    i = 1
    while 10 > parcel > 0:
        parcel = parcel + i
    num_days = (desired_goal / (num_trades/ parcel)) / (288 * profit_per_parcel)
    return num_days

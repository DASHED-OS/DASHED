def trigger_topping_up_volume():
    if volume > ma5_volume:
        print('Greater than 5-day Moving Average Volume')
    else:
        print("Below-average volume")
    return topping_up_volume

def distribution_phase():
    return 
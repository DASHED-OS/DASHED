def trigger_stopping_volume():
    if volume > ma5_volume:
        print('Greater than 5-day Moving Average Volume')
    else:
        print("Below-average volume")
    return stopping_volume

def accumulation_phase():
    return 
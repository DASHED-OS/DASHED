from postulate_billions_technicals import *

def grab_social_sentiment(vpa, volume_per_candle, sentiment):
    """
    param: vpa (int), this tells us the vpa in the current market
    param: volume_per_candle (int), this tells us the volume for the current candle
    param: sentiment (int), this is the value for the market sentiment we recieved 
    to build a signal upon
    This function will grab the sentiment and market data and create a raw_social_sentiment 
    to then be turned into a social_sentiment_signal
    returns: raw_social_sentiment (int), this is the calculated sentiment normalized by the
    current market vpa
    """
    normalized_vpa = vpa / volume_per_candle
    raw_social_sentiment = sentiment / normalized_vpa
    return raw_social_sentiment

def send_social_sentiment_signals(raw_social_sentiment):
    return signal

def socail_sentiment_format():
    return formatted_result
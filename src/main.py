
from dotenv import load_dotenv
from alpha_vantage.timeseries import TimeSeries
import os
# import pandas

load_dotenv()
# key = os.getenv("ALPHA_ADVANTAGE_API_KEY")
ts = TimeSeries(key= os.getenv("ALPHA_ADVANTAGE_API_KEY"), output_format="pandas")

df, metadata = ts.get_daily(symbol="ASX:VAS", outputsize='full')

print(df)
# print(metadata)
# for row in df:
#     print(row)
#     print("\n")
# print(df)
# df.map()

# print(type(df))

# print(df.shape())
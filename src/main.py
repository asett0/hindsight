
from dotenv import load_dotenv
from alpha_vantage.timeseries import TimeSeries
import os
# import pandas

load_dotenv()
print(os.getenv("ALPHA_ADVANTAGE_API_KEY"))
# ts = TimeSeries(key)
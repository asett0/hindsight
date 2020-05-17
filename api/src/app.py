from dotenv import load_dotenv
import dash
import dash_core_components as dcc
import dash_html_components as html
from alpha_vantage.timeseries import TimeSeries
import plotly.express as px
from dash.dependencies import Input, Output
import os
import pandas as pd
# from flask_caching import Cache
# import redis

external_stylesheets = ["https://codepen.io/chriddyp/pen/bWLwgP.css"]

app = dash.Dash(__name__, external_stylesheets=external_stylesheets)


# redis_conn = redis.Redis(host="localhost", port=6379)
# try:
#     redis_conn.ping()
# except redis.exceptions.ConnectionError:
#     raise ConnectionError("Could not connect to Redis server")

load_dotenv()

stock = "ASX:WPL"

ts = TimeSeries(
    key=os.getenv("ALPHA_ADVANTAGE_API_KEY"), output_format="pandas"
)

df, metadata = ts.get_daily(symbol=stock, outputsize="full")

df = df.reset_index()

min_date = df["date"].min()
max_date = df["date"].max()


print(type(os.getenv("REDIS_PORT")))

# @app.callback(Output("chart", "figure"), [State("Input")])
@app.callback(
    Output("chart", "figure"),
    [Input("date-picker", "start_date"), Input("date-picker", "end_date")],
)
def slice_df(start, end):

    df_f = df[
        (df["date"] >= pd.to_datetime(start))
        & (df["date"] <= pd.to_datetime(end))
    ]
    return px.line(df_f, x="date", y="4. close")


app.layout = html.Div(
    [
        dcc.Input(id="stock-input"),
        html.Button("Submit", id="stock-submit-button"),
        dcc.DatePickerRange(
            id="date-picker",
            min_date_allowed=min_date,
            max_date_allowed=max_date,
            display_format="DD/MM/YYYY",
            start_date=min_date,
            end_date=max_date,
        ),
        dcc.Graph(id="chart"),
    ]
)


if __name__ == "__main__":
    app.run_server(debug=True)

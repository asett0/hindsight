module Charts exposing (plotClose)

import Element
import LineChart
import LineChart.Area as Area
import LineChart.Axis as Axis
import LineChart.Axis.Intersection as Intersection
import LineChart.Colors as Colors
import LineChart.Container as Container
import LineChart.Dots as Dots
import LineChart.Events as Events
import LineChart.Grid as Grid
import LineChart.Interpolation as Interpolation
import LineChart.Junk as Junk
import LineChart.Legends as Legends
import LineChart.Line as Line
import Svg
import Types exposing (Msg, Price)


chartConfig : LineChart.Config Price Msg
chartConfig =
    { y = Axis.default 400 "Price" .value
    , x = Axis.default 700 "Index" .timeIndex
    , container = Container.default "Daily Close"
    , interpolation = Interpolation.default
    , intersection = Intersection.default
    , legends = Legends.default
    , events = Events.default
    , junk = Junk.default
    , grid = Grid.default
    , area = Area.default
    , line = Line.default
    , dots = Dots.default
    }


plotClose : List Price -> Element.Element Msg
plotClose prices =
    Element.html <|
        Svg.svg [] <|
            [ LineChart.viewCustom chartConfig
                [ LineChart.line Colors.blueLight Dots.square "Daily Close" prices ]
            ]

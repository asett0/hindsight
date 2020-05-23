module Types exposing (Date, Model, Msg(..), Price)

import Http


type alias Price =
    { timeIndex : Float
    , value : Float
    }


type alias Model =
    { stockSymbol : String
    , dateFrom : String
    , dateFromValid : Bool
    , dateTo : String
    , dateToValid : Bool
    , dailyClose : List Float
    }


type Msg
    = ChangeStockSymbol String
    | ChangeDateFrom String
    | ChangeDateTo String
    | GetStockButtonPressed
    | DisabledGetStockButtonPressed
    | GotDailyClose (Result Http.Error (List Float))


type alias Date =
    { year : Int
    , month : Int
    , day : Int
    }

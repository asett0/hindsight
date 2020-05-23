module Main exposing (main)

import Browser
import Charts exposing (plotClose)
import Element
import Element.Input as Input
import Helpers exposing (checkValidDateString, closeDecoder, validDateStringToPosixInt)
import Html exposing (Html)
import Http
import Types exposing (Model, Msg(..), Price)
import Url exposing (Protocol(..))
import Url.Builder



-- import
-- MAIN


getDailyClose : String -> String -> String -> Cmd Msg
getDailyClose stockSymbol dateFrom dateTo =
    Http.get
        { url =
            Url.toString
                { protocol = Http
                , host = "flask"
                , port_ = Nothing
                , path =
                    Url.Builder.absolute [ "api", "stock", "close", "daily" ]
                        [ Url.Builder.string "symbol" stockSymbol
                        , Url.Builder.int "from" <| validDateStringToPosixInt dateFrom
                        , Url.Builder.int "to" <| validDateStringToPosixInt dateTo
                        ]
                , query = Nothing
                , fragment = Nothing
                }
        , expect = Http.expectJson GotDailyClose closeDecoder
        }


main : Program () Model Msg
main =
    Browser.element
        { init = init
        , update = update
        , view = view
        , subscriptions = subscriptions
        }



-- SUBSCRIPTIONS


subscriptions : Model -> Sub Msg
subscriptions _ =
    Sub.none



-- INIT


init : () -> ( Model, Cmd Msg )
init _ =
    ( { stockSymbol = "WPL", dateFrom = "01/01/2000", dateFromValid = True, dateTo = "31/12/2010", dateToValid = True, dailyClose = [] }
    , Cmd.none
    )



-- UPDATE


update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    case msg of
        ChangeStockSymbol stockSymbol ->
            ( { model | stockSymbol = stockSymbol }, Cmd.none )

        ChangeDateFrom dateFrom ->
            ( { model | dateFrom = dateFrom, dateFromValid = checkValidDateString dateFrom }, Cmd.none )

        ChangeDateTo dateTo ->
            ( { model | dateTo = dateTo, dateToValid = checkValidDateString dateTo }, Cmd.none )

        GetStockButtonPressed ->
            ( model, getDailyClose model.stockSymbol model.dateFrom model.dateTo )

        DisabledGetStockButtonPressed ->
            ( model, Cmd.none )

        GotDailyClose result ->
            case result of
                Ok prices ->
                    ( { model | dailyClose = prices }, Cmd.none )

                Err _ ->
                    ( model, Cmd.none )



-- MODEL


getStockButton : Bool -> Bool -> Element.Element Msg
getStockButton dateToValid dateFromValid =
    Input.button
        []
        { onPress =
            if dateToValid && dateFromValid then
                Just GetStockButtonPressed

            else
                Just DisabledGetStockButtonPressed
        , label =
            Element.text "Get stock!"
        }


view : Model -> Html Msg
view model =
    Element.layout [] <|
        Element.column []
            [ Element.el [] (Element.text "ASX Backtester")
            , Input.text []
                { onChange = ChangeStockSymbol
                , text = model.stockSymbol
                , placeholder = Nothing
                , label = Input.labelLeft [] <| Element.text "Choose stock: "
                }
            , Element.row []
                [ Input.text []
                    { onChange = ChangeDateFrom
                    , text = model.dateFrom
                    , placeholder = Nothing
                    , label = Input.labelLeft [] <| Element.text "Date from: "
                    }
                , Element.text <|
                    if model.dateFromValid then
                        "Date from is valid"

                    else
                        "Date from is NOT valid"
                ]
            , Element.row []
                [ Input.text []
                    { onChange = ChangeDateTo
                    , text = model.dateTo
                    , placeholder = Nothing
                    , label = Input.labelLeft [] <| Element.text "Date to:"
                    }
                , Element.text <|
                    if model.dateToValid then
                        "Date to is valid"

                    else
                        "Date to is NOT valid"
                ]
            , getStockButton model.dateFromValid model.dateToValid
            , plotClose <| List.indexedMap (\i p -> Price (toFloat i) p) model.dailyClose
            ]

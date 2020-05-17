module Main exposing (main)

import Browser
import Element
import Element.Input as Input
import Html exposing (Html)
import Types exposing (Model, Msg(..))



-- MAIN


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
    ( { dummyAttr = 0, inputText = "ASX:WPL" }
    , Cmd.none
    )



-- UPDATE


update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    case msg of
        InputText txt ->
            ( { model | inputText = txt }, Cmd.none )



-- MODEL


view : Model -> Html Msg
view model =
    Element.layout [] <|
        Element.column []
            [ Element.el [] (Element.text "ASX Backtester")
            , Input.text []
                { onChange = InputText
                , text = model.inputText
                , placeholder = Nothing
                , label = Input.labelLeft [] <| Element.text "Choose stock"
                }
            ]

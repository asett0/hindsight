module Types exposing (Model, Msg(..))


type alias Model =
    { dummyAttr : Int
    , inputText : String
    }


type Msg
    = InputText String

module Helpers exposing (checkValidDateString, closeDecoder, validDateStringToPosixInt)

import Json.Decode as Decode
import Parser exposing ((|.), (|=))
import Time exposing (Month(..))
import Time.Extra as TimeExtra
import TimeZone exposing (australia__sydney)
import Types exposing (Date)


zone : Time.Zone
zone =
    australia__sydney ()


intToMonth : Int -> Maybe Month
intToMonth n =
    case n of
        1 ->
            Just Jan

        2 ->
            Just Feb

        3 ->
            Just Mar

        4 ->
            Just Apr

        5 ->
            Just May

        6 ->
            Just Jun

        7 ->
            Just Jul

        8 ->
            Just Aug

        9 ->
            Just Sep

        10 ->
            Just Oct

        11 ->
            Just Nov

        12 ->
            Just Dec

        _ ->
            Nothing


digitParser : Parser.Parser Int
digitParser =
    Parser.succeed (\s -> Maybe.withDefault 0 <| String.toInt s)
        |= Parser.getChompedString (Parser.chompIf Char.isDigit)


dayAndMonthParser : Parser.Parser Int
dayAndMonthParser =
    Parser.succeed (\dm1 dm2 -> dm1 * 10 + dm2)
        |= digitParser
        |= digitParser


yearParser : Parser.Parser Int
yearParser =
    Parser.succeed (\y1 y2 y3 y4 -> y1 * 1000 + y2 * 100 + y3 * 10 + y4)
        |= digitParser
        |= digitParser
        |= digitParser
        |= digitParser


checkValidDate : Date -> Bool
checkValidDate date =
    (date.year >= 0)
        && (date.month >= 1)
        && (date.month <= 12)
        && (date.day >= 1)
        && (date.day
                <= (case date.month of
                        2 ->
                            case modBy date.year 4 of
                                0 ->
                                    case modBy date.year 100 of
                                        0 ->
                                            case modBy date.year 400 of
                                                0 ->
                                                    29

                                                _ ->
                                                    28

                                        _ ->
                                            29

                                _ ->
                                    28

                        _ ->
                            if List.member date.month [ 4, 6, 9, 11 ] then
                                30

                            else
                                31
                   )
           )


checkValidDateString : String -> Bool
checkValidDateString s =
    case Parser.run dateParser s of
        Ok _ ->
            True

        Err _ ->
            False


dateParser : Parser.Parser Date
dateParser =
    Parser.succeed (\d m y -> Date y m d)
        |= dayAndMonthParser
        |. Parser.symbol "/"
        |= dayAndMonthParser
        |. Parser.symbol "/"
        |= yearParser
        |. Parser.end
        |> Parser.andThen
            (\date ->
                if checkValidDate date then
                    Parser.succeed date

                else
                    Parser.problem "Date provided is invalid"
            )



-- Assume dateString provided is a valid date string


validDateStringToPosixInt : String -> Int
validDateStringToPosixInt s =
    case Parser.run dateParser s of
        Ok date ->
            (Time.posixToMillis <|
                TimeExtra.partsToPosix
                    zone
                    { year = date.year
                    , month = Maybe.withDefault Jan <| intToMonth date.month
                    , day = date.day
                    , hour = 0
                    , minute = 0
                    , second = 0
                    , millisecond = 0
                    }
            )
                // 1000

        Err _ ->
            0


closeDecoder =
    Decode.list Decode.float

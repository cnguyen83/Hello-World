{-
    Example from Elm Tutorial
    https://guide.elm-lang.org/architecture/user_input/forms.html
    Added cases to check for password requirements (maybe in too imperative of a style)
    NOT added in age field and checker yet.

    Resources:
    https://gist.github.com/moonlightdrive/86b5bcb57df87c45f468a13a326894ad
    https://gist.github.com/gabehollombe/45cb6f61ccb270931ded71f155ca37fe
    https://dennisreimann.de/articles/elm.html
    http://package.elm-lang.org/packages/elm-lang/core/5.1.1/Char
    http://package.elm-lang.org/packages/elm-lang/core/latest/String
    http://elm-lang.org/docs/syntax
    https://www.cis.upenn.edu/~matuszek/Concise%20Guides/Concise%20Elm.html
-}

import Char
import Html exposing (..)
import Html.Attributes exposing (..)
import Html.Events exposing (onInput)
import String

main =
    Html.beginnerProgram { model = model, view = view, update = update }


-- MODEL 

type alias Model =
    { name : String
    , password : String
    , passwordAgain : String
    }

model : Model
model = 
    Model "" "" ""


-- UPDATE 
type Msg
    = Name String
    | Password String
    | PasswordAgain String

update : Msg -> Model -> Model
update msg model = 
    case msg of
        Name name ->
            { model | name = name }

        Password password ->
            { model | password = password }

        PasswordAgain password ->
            { model | passwordAgain = password }

-- VIEW 

validate : Model -> (String, String)
validate model =
    if String.length model.password < 8 then
        ("red", "Password too short")
    else if not (String.any Char.isDigit model.password) then
        ("red", "Password must contain a number")
    else if (String.all Char.isDigit model.password) then
        ("red", "Password must contain an upper-case letter and a lower-case letter")
    else if not (String.any Char.isUpper model.password) then
        ("red", "Password must contain an upper-case letter")
    else if not (String.any Char.isLower model.password) then
        ("red", "Password must contain a lower-case letter")
    else ("green", "OK")

view : Model -> Html Msg
view model =
    div []
        [ input [ type_ "text", placeholder "Name", onInput Name ] []
        , input [ type_ "password", placeholder "Password", onInput Password] []
        , input [ type_ "password", placeholder "Re-enter Password", onInput PasswordAgain] []
        , viewValidation model
        ]

viewValidation : Model -> Html msg
viewValidation model = 
    let
        (color, message) =
            if model.password == model.passwordAgain then
                validate model
                {-if String.length model.password >= 8 then
                    ("green", "OK")
                else
                    ("red", "Password too short")-}
            else
                ("red", "Passwords do not match!")
    in
        div [ style [("color", color)] ] [ text message ]







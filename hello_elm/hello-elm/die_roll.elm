import Html exposing (..)
import Html.Events exposing (..)
import Random

{- 
    New to include "subscriptions" now since no longer using Html.beginnerProgram
    {- Remember new architecture skeleton: https://guide.elm-lang.org/architecture/effects/ -}
-}
main = 
    Html.program 
        { init = init
        , view = view
        , update = update
        , subscriptions = subscriptions
        }

-- MODEL
type alias Model = 
    { dieFace: Int }


-- UPDATE
type Msg = Roll 
    | NewFace Int

update : Msg -> Model -> (Model, Cmd Msg)
update msg model = 
    case msg of 
        Roll -> 
            (model, Random.generate NewFace (Random.int 1 6))

        {- Step the model forward but don't do anything -}
        NewFace newFace ->
            (Model newFace, Cmd.none)

-- SUBSCRIPTIONS
subscriptions : Model -> Sub Msg
subscriptions model =
    Sub.none

-- VIEW
view : Model -> Html Msg
view model = 
    div []
        [ h1 [] [ text (toString model.dieFace) ]
        , button [ onClick Roll ] [ text "Roll" ]
        ]

-- INIT 

init : (Model, Cmd Msg)
init = 
    (Model 1, Cmd.none)
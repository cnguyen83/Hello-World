import Html
import String


-- MODEL
type alias model = 
    { dieFace: Int }

-- UPDATE
type Msg = Roll

update : Msg -> Model -> (Model, Cmd Msg)
update msg model = 
    case msg of 
        Roll -> 
            (model, Cmd.none)

-- VIEW
view : Model -> Html Msg
view model = 
    div []
        [ h1 [] [ text (toString model.dieFace)]
        , button [ onClick Roll ] [ text "Roll" ]
        ]

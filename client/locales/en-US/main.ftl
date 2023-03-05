language-name = English (US)

game-name = The Onion Or Not The Onion

game-title = { game-name }
game-subtitle = Drinking Game
game-title-description = is a web game in which your mobile devices are your controllers.
    Just create a lobby and play. Furthermore you can host a screen which provides a view for the
    rest of your party which doesn't want to play.

join-game-header-string-1 = Join Game with
join-game-header-string-2 = and Code
join-game-header-string-3 = {""}
join-game-header-missing-url-string-1 = Join Game with Code
join-game-header-missing-url-string-2 = {""}

cancel-button-text = CANCEL
go-back-button-text = GO BACK

## Errors

error-web-socket-open = Failed opening the connection to the lobby.
error-web-socket-message-receive-connection-error = The communication with the lobby failed when receiving a message.
error-web-socket-message-receive-connection-close = The connection to the lobby has been closed.
error-web-socket-message-receive-message-send-error = The communication with the lobby failed when sending a message.
error-web-socket-handle-message-player-name-already-in-use = The player name is already in use in this lobby, please choose another one.


## Game Creation Form

game-creation-form-username-label = Playername
game-creation-form-username-placeholder = { game-creation-form-username-label }

game-creation-form-invite-code-label = Invite Code
game-creation-form-invite-code-placeholder = { game-creation-form-invite-code-label }

game-creation-form-starting-game-explanation = With no invite code a new game will be started. Otherwise the game with the code will be joined.

game-creation-form-just-watch-label = I just want to watch!

game-creation-form-max-questions-label = Count of Questions
game-creation-form-max-questions-placeholder = { game-creation-form-max-questions-label }
game-creation-form-max-questions-explanation = Leave Blank if you want to get all available questions.

-minimum-score = Minimum Reddit Score of Questions
game-creation-form-minimum-score-label = { -minimum-score }
game-creation-form-minimum-score-placeholder = { game-creation-form-minimum-score-label }
game-creation-form-minimum-score-explanation = Leave Blank if you don't care how much score a question has.
game-creation-form-minimum-score-count-of-available = With a minimum score of { $score } there { $count ->
        [one] is { $count } question
        *[other] are { $count } questions
    } available.

game-creation-form-timer-wanted-label = Seconds to answer
game-creation-form-timer-wanted-placeholder = { game-creation-form-timer-wanted-label }
game-creation-form-timer-wanted-explanation = Leave Blank if no timer while answering is wished.

game-creation-form-submit-value-create = CREATE
game-creation-form-submit-value-join = JOIN

game-creation-form-error-message-player-name-empty = Playername is missing.
game-creation-form-error-message-invite-code-empty = Invite Code is missing.
game-creation-form-error-message-max-questions-invalid = Count of Questions could not be parsed.
game-creation-form-error-message-minimum-score-invalid = { -minimum-score } could not be parsed.
game-creation-form-error-message-timer-wanted-invalid = Timer could not be parsed.

## Connecting View

connecting-view-connecting-string = Connecting...

## Play View

play-view-type-of-player-watcher = Watcher
play-view-type-of-player-player = Player

play-view-exit-the-game = Exit The Game

play-view-players-headline = Players:
play-view-players-no-one-here = No one here!
play-view-players-is-watching = Watching
play-view-players-points = { $points } { $points ->
        [one] Point
        *[other] Points
    }
play-view-players-points-explanation = You get 10 points on a correct answer, and 5 additional points if less than half of players guessed correctly.

lobby-view-welcome-headline = Welcome!
lobby-view-start-game-button = START

## Game View Question Playing State

game-view-question-playing-state-remaining-seconds = { $seconds } {$seconds ->
        [one] second
        *[other] seconds
    } to go!
game-view-question-playing-state-infinite-remaining-seconds = {""}

game-view-question-playing-state-selection-button-the-onion = THE ONION
game-view-question-playing-state-selection-button-not-the-onion = NOT THE ONION

## Game View Solution Playing State

game-view-solution-playing-state-remaining-seconds = { $seconds } {$seconds ->
        [one] second
        *[other] seconds
    } to go!
game-view-solution-playing-state-continue = Continue
game-view-solution-playing-state-continuing = Continuing...

game-view-solution-playing-state-sub-headline-the-onion = It's THE ONION
game-view-solution-playing-state-sub-headline-not-the-onion = It's NOT THE ONION

game-view-solution-playing-state-sub-headline-player-answer-correct = Your answer was correct!
game-view-solution-playing-state-sub-headline-player-answer-wrong = Your answer was wrong!
game-view-solution-playing-state-sub-headline-player-answer-missing = Your answer was missing.

game-view-solution-playing-state-link-to-newspaper-posting-anchor-text = Link to post

## Aftermath View

aftermath-view-headline = The Game Has Ended
aftermath-view-next-round = PLAY AGAIN!
aftermath-view-next-round-clicked = Waiting on other players for net round...
aftermath-view-ranking-headline = Ranking
aftermath-view-ranking-no-one = No one is in the ranking!
aftermath-view-ranking-players-points = { $points } { $points ->
        [one] Point
        *[other] Points
    }

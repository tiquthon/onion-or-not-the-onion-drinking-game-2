language-name = Deutsch

game-name = The Onion Oder Nicht The Onion

game-title = { game-name }
game-subtitle = Trinkspiel
game-title-description = ist ein Online Spiel, in welchem dein Handy dein Controller ist.
    Erstell einfach eine Lobby und spiele. Zusätzlich kannst du einen großen Bildschirm nutzen,
    auf welchem jeder, der auf deiner ganze Party nicht mitspielen will, das Geschehen mitverfolgen
    kann.

join-game-header-string-1 = Tritt dem Spiel auf
join-game-header-string-2 = mit dem Code
join-game-header-string-3 = bei!
join-game-header-missing-url-string-1 = Tritt dem Spiel mit dem Code
join-game-header-missing-url-string-2 = bei!

cancel-button-text = ABBRECHEN
go-back-button-text = ZURÜCK

## Errors

error-web-socket-open = Es schlug fehl, die Verbindung zur Lobby zu öffnen.
error-web-socket-message-receive-connection-error = Die Kommunikation mit der Lobby schlug fehl während eine Nachricht empfangen wurde.
error-web-socket-message-receive-connection-close = Die Verbindung zur Lobby wurde geschlossen.
error-web-socket-message-receive-message-send-error = Die Kommunikation mit der Lobby schlug fehl während eine Nachricht gesendet wurde.
error-web-socket-handle-message-player-name-already-in-use = Der Spielername wird in der Lobby bereits verwendet, bitte wähle einen Anderen.

## Game Creation Form

game-creation-form-username-label = Spielername
game-creation-form-username-placeholder = { game-creation-form-username-label }

game-creation-form-invite-code-label = Einladungscode
game-creation-form-invite-code-placeholder = { game-creation-form-invite-code-label }

game-creation-form-starting-game-explanation = Mit leerem Einladungscode wird ein neues Spiel gestartet. Andernfalls wird dem Spiel mit dem Einladungscode beigetreten.

game-creation-form-just-watch-label = Ich will einfach nur zuschauen!

game-creation-form-max-questions-label = Anzahl an Fragen
game-creation-form-max-questions-placeholder = { game-creation-form-max-questions-label }
game-creation-form-max-questions-explanation = Leer Lassen, wenn alle Fragen gewollt sind.

-minimum-score = Minimale Reddit-Bewertung der Fragen
game-creation-form-minimum-score-label = { -minimum-score }
game-creation-form-minimum-score-placeholder = { game-creation-form-minimum-score-label }
game-creation-form-minimum-score-explanation = Leer Lassen, wenn es egal ist, wie gut eine Frage sein muss.
game-creation-form-minimum-score-count-of-available = Mit einer minimalen Bewertung von { $score } { $count ->
        [one] ist { $count } Frage
        *[other] sind { $count } Fragen
    } verfügbar.

game-creation-form-timer-wanted-label = Sekunden zum Beantworten
game-creation-form-timer-wanted-placeholder = { game-creation-form-timer-wanted-label }
game-creation-form-timer-wanted-explanation = Leer Lassen, wenn kein Timer zum Beantworten erwünscht ist.

game-creation-form-submit-value-create = ERSTELLEN
game-creation-form-submit-value-join = BEITRETEN

game-creation-form-error-message-player-name-empty = Der { game-creation-form-username-label } fehlt.
game-creation-form-error-message-invite-code-empty = Der { game-creation-form-invite-code-label } fehlt.
game-creation-form-error-message-max-questions-invalid = Die { game-creation-form-max-questions-label } konnte nicht verarbeitet werden.
game-creation-form-error-message-minimum-score-invalid = Die { -minimum-score } konnte nicht verarbeitet werden.
game-creation-form-error-message-timer-wanted-invalid = Der { game-creation-form-timer-wanted-label } konnte nicht verarbeitet werden.

## Connecting View

connecting-view-connecting-string = Verbinden...

## Play View

play-view-type-of-player-watcher = Zuschauer
play-view-type-of-player-player = Spieler

play-view-exit-the-game = Das Spiel verlassen

play-view-players-headline = Spieler:
play-view-players-no-one-here = Keiner da!
play-view-players-is-watching = Zuschauend
play-view-players-points = { $points } { $points ->
        [one] Punkt
        *[other] Punkte
    }
play-view-players-points-explanation = Du erhältst 10 Punkte bei einer korrekten Antwort und 5 weitere Punkte, wenn weniger als die Hälfte der Spieler korrekt lagen.

lobby-view-welcome-headline = Willkommen!
lobby-view-start-game-button = STARTEN

## Game View Question Playing State

game-view-question-playing-state-remaining-seconds = Es {$seconds ->
        [one] verbleibt { $seconds } Sekunde
        *[other] verbleiben { $seconds } Sekunden
    }
game-view-question-playing-state-infinite-remaining-seconds = {""}

game-view-question-playing-state-selection-button-the-onion = THE ONION
game-view-question-playing-state-selection-button-not-the-onion = NOT THE ONION

## Game View Solution Playing State

game-view-solution-playing-state-remaining-seconds = Es {$seconds ->
        [one] verbleibt { $seconds } Sekunde
        *[other] verbleiben { $seconds } Sekunden
    }
game-view-solution-playing-state-continue = Weiter
game-view-solution-playing-state-continuing = Weiter...

game-view-solution-playing-state-sub-headline-the-onion = Es ist THE ONION
game-view-solution-playing-state-sub-headline-not-the-onion = Es ist NOT THE ONION

game-view-solution-playing-state-sub-headline-player-answer-correct = Deine Antwort war richtig!
game-view-solution-playing-state-sub-headline-player-answer-wrong = Deine Antwort war falsch!
game-view-solution-playing-state-sub-headline-player-answer-missing = Deine Antwort hat gefehlt.

game-view-solution-playing-state-link-to-newspaper-posting-anchor-text = Link zum Artikel

## Aftermath View

aftermath-view-headline = Das Spiel ist vorbei
aftermath-view-next-round = NOCHMAL SPIELEN!
aftermath-view-next-round-clicked = Warte auf andere Spieler für nächste Runde...
aftermath-view-ranking-headline = Rangliste
aftermath-view-ranking-no-one = Niemand ist in der Rangliste!
aftermath-view-ranking-players-points = { $points } { $points ->
        [one] Punkt
        *[other] Punkte
    }

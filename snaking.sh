#!/bin/bash

SNAKE_COLOR="#6cbb3c"
CAMEL_COLOR="#c19a6b"

function clrd {
	palette="$1"
	shift

	if [[ "$palette" == 'sk' ]]; then
		palette="$SNAKE_COLOR"
	elif [[ "$palette" == 'cm' ]]; then
		palette="$CAMEL_COLOR"
	else
		exit 1
	fi

	gum style --foreground "$palette" ${*: 1: $# - 1} "${*: -1}"
}

gum style \
    --border thick \
    --margin 1 --padding 1 \
    --border-foreground "$SNAKE_COLOR" \
    "$(clrd sk --bold "🐍 Snake")ization"


camel_name=""
parameter_text=""
if [[ -n "${1+x}" ]]; then
    parameter_text=$(clrd cm --bold --italic --underline "command args")
	camel_name="$1"
else
    parameter_text=$(clrd cm --bold --italic --underline "clipboard")
	camel_name=$(pbpaste)
fi
echo "Using parameter from $parameter_text"


snake_name=$(
    echo "$camel_name" | 
    tr '[:upper:]' '[:lower:]' | 
    tr -d '()' |
    tr ' -' '_' | 
    awk '/^[0-9]/ { printf "_" } { gsub(/_+/, "_"); print }'
)

camel_name_text=$(
	gum style --border rounded --margin 1 --padding 1 --border-foreground "$CAMEL_COLOR" "$camel_name"
)
snake_name_text=$(
	gum style --border rounded --margin 1 --padding 1 --border-foreground "$SNAKE_COLOR" "$snake_name"
)

gum join --horizontal --align center "$camel_name_text" "⭆" "$snake_name_text"

printf "%s" "$snake_name" | pbcopy

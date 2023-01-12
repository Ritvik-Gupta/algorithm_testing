echo "$1"

generated_name=$(
    echo "$1" | 
    tr '[:upper:]' '[:lower:]' | 
    tr -d '()' | 
    tr ' -' '_' | 
    awk '/^[0-9]/ { printf "_" } { gsub(/_+/, "_"); print }'
)

echo $generated_name

echo "$generated_name" | tr -d '\n' | pbcopy

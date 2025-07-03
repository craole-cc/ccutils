set windows-shell := ["C:\\Program Files\\Git\\bin\\sh.exe", "-c"]
set unstable := true
set fallback := true

#~@ Format all justfiles with just and other files with treefmt
fmt:
    find . -name "justfile" -o -name "*.justfile" -o -name ".justfile" | \
    while read -r file; do echo "Formatting $file"; \
    just --fmt --unstable --justfile "$file" || \
    echo "Failed to format $file"; \
    done
    treefmt --clear-cache --fail-on-change

#~@ Quick commit with message
commit MESSAGE:
    jj describe --message "{{ MESSAGE }}"
    jj bookmark set main --revision=@
    jj git push

#~@ Interactive describe commit
commit-interactive:
    jj describe
    jj bookmark set main --revision=@
    jj git push

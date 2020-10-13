_qeda() {
    local cur prev opts base
    COMPREPLY=()
    cur="${COMP_WORDS[COMP_CWORD]}"
    prev="${COMP_WORDS[COMP_CWORD-1]}"

    case "${prev}" in
        qeda)
            opts="--help --verbose --version -h -V -v add completion config generate ground help index list load power reset test update"
            COMPREPLY=($(compgen -W "${opts}" -- ${cur}))
            return 0
            ;;
        add | load)
            COMPREPLY=($(qeda list "${cur}"))
            return 0;
            ;;
    esac
} && complete -F _qeda qeda

// This code is based on Richard Hansen's answer on StackOverflow:
// https://stackoverflow.com/a/7287873/1176984

pub const TRAP_ADD: &'static str = indoc::indoc!(
    r#"
    # appends a command to a trap
    #
    # - 1st arg:  code to add
    # - remaining args:  names of traps to modify
    #
    __fnm_trap_add__() {
        __fnm_trap_add___cmd=$1; shift || fatal "${FUNCNAME} usage error"
        for __fnm_trap_add___name in "$@"; do
            trap -- "$(
                # helper fn to get existing trap command from output
                # of trap -p
                extract_trap_cmd() { printf '%s\n' "$3"; }
                # print existing trap command with newline
                eval "extract_trap_cmd $(trap -p "${__fnm_trap_add___name}")"
                # print the new trap command
                printf '%s\n' "${__fnm_trap_add___cmd}"
            )" "${__fnm_trap_add___name}" \
                || fatal "unable to add to trap ${__fnm_trap_add___name}"
        done
    }
    # set the trace attribute for the above function.  this is
    # required to modify DEBUG or RETURN traps because functions don't
    # inherit them unless the trace attribute is set
    declare -f -t __fnm_trap_add__
    "#
);

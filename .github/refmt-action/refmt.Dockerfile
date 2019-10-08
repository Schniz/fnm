FROM ocaml/opam2:4.06

RUN opam update && opam install reason

CMD ["refmt", "--version"]

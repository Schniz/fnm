FROM akabe/ocaml:4.06

RUN opam update
RUN opam install -y --best-effort reason

CMD ["refmt", "--version"]

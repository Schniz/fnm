FROM akabe/ocaml:4.06.1

RUN apk add --no-cache m4 bash
RUN opam update
RUN opam install -y --best-effort reason

CMD ["refmt", "--version"]

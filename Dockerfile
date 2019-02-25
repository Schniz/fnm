FROM frolvlad/alpine-glibc

RUN apk add --no-cache nodejs bash npm curl g++ make m4 patch gmp-dev perl git jq perl-utils libressl-dev

USER root

RUN npm -g config set user root
RUN npm i -g esy@latest

WORKDIR /app
ADD . /app

RUN jq '. | .buildDirs.executable.flags |= . + ["-ccopt", "-static"]' package.json > package.json.new && mv package.json.new package.json
RUN npx esy i
RUN npx esy verify-fnm-package
RUN npx esy pesy
RUN npx esy b
RUN npx esy test

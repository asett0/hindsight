FROM alpine:3.11 AS elm
RUN apk add curl
RUN curl -L -o elm.gz https://github.com/elm/compiler/releases/download/0.19.1/binary-for-linux-64-bit.gz
RUN gunzip elm.gz
RUN chmod +x elm
RUN mv elm /usr/local/bin/

FROM node:14.2.0-alpine3.11 AS dev
RUN npm install -g elm-live@4.0.2
COPY --from=elm /usr/local/bin/elm /usr/local/bin/
COPY frontend/ /app/frontend/
WORKDIR /app/frontend/
CMD ["elm-live", "src/Main.elm","--host=0.0.0.0"]
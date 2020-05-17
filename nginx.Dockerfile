FROM alpine:3.11 AS elm
RUN apk add curl
RUN curl -L -o elm.gz https://github.com/elm/compiler/releases/download/0.19.1/binary-for-linux-64-bit.gz
RUN gunzip elm.gz
RUN chmod +x elm
RUN mv elm /usr/local/bin/

FROM elm AS builder
COPY frontend/ /app/frontend/
WORKDIR /app/frontend/
RUN elm make src/Main.elm

FROM nginx:1.17.10-alpine AS prod
RUN rm /etc/nginx/conf.d/default.conf
COPY --from=builder /frontend/index.html /app/static/html/
COPY nginx/nginx.conf /etc/nginx/
CMD ["nginx", "-g", "daemon off;"]
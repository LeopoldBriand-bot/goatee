# Build the Vue app for production
FROM node:14.5.0-alpine as builder
WORKDIR /usr/app
COPY . .

ENV NODE_ENV=development
ENV NODE_OPTIONS=--max-old-space-size=4096

RUN npm ci

WORKDIR /usr/app/apps/client
RUN npm run build

# Vue app Apache server
FROM node:current-alpine3.15

ENV NODE_ENV=production
RUN mkdir -p /var/www/html

WORKDIR /var/www/html
COPY --from=builder /usr/app/dist .

RUN npm init -y
RUN npm install --global http-server

# CMD ["npx http-server -p 80 ."]
ENTRYPOINT ["npx", "http-server", "-p 80", "."]

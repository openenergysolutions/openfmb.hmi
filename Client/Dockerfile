### STAGE 1: Build ###
FROM node:alpine AS build
WORKDIR /app

COPY . .
RUN npm install
RUN npm run build

### STAGE 2: Run ###
FROM nginx:1.19.5-alpine

COPY /nginx-custom.conf /etc/nginx/conf.d/default.conf
COPY --from=build /app/dist/openfmb-hmi /usr/share/nginx/html

EXPOSE 80

ENTRYPOINT ["nginx", "-g", "daemon off;"]
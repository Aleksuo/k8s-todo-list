FROM node:22-slim AS base

# Install basic development tools
RUN apt update && apt install -y less man-db sudo
RUN npm install -g @angular/cli

RUN mkdir app
COPY . /app

WORKDIR /app

RUN npm ci

# Target that immediately runs the development server
FROM base AS development

RUN npm run start

FROM  base AS build

RUN npm run build 

FROM nginx:1.28.0-alpine AS production

COPY nginx.conf /etc/nginx/nginx.conf
COPY --from=build /app/dist/k8s-todo-frontend/browser /usr/share/nginx/html
EXPOSE  80

CMD [ "nginx", "-g", "daemon off;"]
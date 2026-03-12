# Stage 1: Build the React app
FROM node:25.2-bookworm-slim as build
WORKDIR /app

# Copy dependency files first for better caching
COPY package.json package-lock.json ./

# Use cache mount for npm to speed up dependency installation
RUN --mount=type=cache,target=/root/.npm \
    npm ci --prefer-offline --no-audit

# Copy source code after dependencies are installed
COPY . .

# Build the application
RUN npm run build

# Stage 2: Serve the app using NGINX
FROM nginx:alpine
COPY --from=build /app/dist /usr/share/nginx/html
COPY nginx.conf /etc/nginx/conf.d/default.conf
EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]
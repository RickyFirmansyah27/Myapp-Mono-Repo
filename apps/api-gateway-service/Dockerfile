FROM kong:3.1.1-alpine

# Switch back to root user to install additional packages
USER root

# Install the js-pluginserver
RUN apk update \
    && apk add --update nodejs npm python3 make g++ vim nano \
    && rm -rf /var/cache/apk/* 

# Install kong-pdk as root
RUN npm install --ignore-scripts --unsafe -g kong-pdk@0.6.0

# Switch back to kong-user
USER kong

ENV term xterm

# TODO add plugin
# WORKDIR /usr/local/kong/js-plugins
# COPY apps/kong-gateway-service/plugins .
# RUN npm install --ignore-scripts

USER kong
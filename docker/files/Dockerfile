FROM debian:buster-slim

ADD release /release
ADD config.properties.template /
ADD startup.sh /
ADD sources.list /etc/apt/

RUN apt-get update && \
    apt-get install -y gettext imagemagick && \
    rm -rf /var/lib/apt/lists/* && \
    chmod +x /startup.sh

EXPOSE 8000 8000

entrypoint ["/startup.sh"]
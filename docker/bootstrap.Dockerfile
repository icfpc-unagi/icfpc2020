FROM ubuntu:20.04

################################################################################
# Environment variables (required for installation)
# NOTE: environment variables not required for installation can be placed in
# `Configurations` section.
################################################################################

ENV DEBIAN_FRONTEND noninteractive
ENV APT_KEY_DONT_WARN_ON_DANGEROUS_USAGE=DontWarn

ARG UNAGI_PASSWORD
RUN [ "${UNAGI_PASSWORD}" != '' ]
ENV UNAGI_PASSWORD=$UNAGI_PASSWORD

################################################################################
# Installation
################################################################################

# Use GCP apt.
RUN sed -i.bak -e "s%http://archive.ubuntu.com/ubuntu/%http://asia-northeast1.gce.archive.ubuntu.com/ubuntu/%g" /etc/apt/sources.list

# Stop compression: https://bugs.launchpad.net/ubuntu/+source/command-not-found/+bug/1876034
RUN apt-get clean && rm -rf /var/lib/apt/lists/* && \
    rm /etc/apt/apt.conf.d/docker-gzip-indexes

# Unminimize the image.
RUN apt-get update -q && yes | unminimize

# Install fundamental tools.
RUN apt-get update -q && apt-get install -qy apt-utils curl sudo && \
    apt-get clean -q && rm -rf /var/lib/apt/lists/*

################################################################################
# Configurations
################################################################################

# Create unagi user.
RUN useradd \
        --home-dir=/home/unagi \
        --create-home \
        --uid=10001 \
        --user-group \
        --shell=/bin/bash \
        unagi
RUN echo 'unagi ALL=(ALL:ALL) NOPASSWD: ALL' > /etc/sudoers.d/unagi

# Unagi password.
RUN echo "export UNAGI_PASSWORD='${UNAGI_PASSWORD}'" > /etc/profile.d/99-unagi.sh
RUN chmod +x /etc/profile.d/99-unagi.sh

# Add bin directory as default commands.
ADD ./bin /usr/local/bin

# Add unagi command as proxy.
RUN echo '#!/usr/bin/env bash' > /usr/local/bin/unagi && \
    echo 'exec "$@"' >> /usr/local/bin/unagi && \
    chmod +x /usr/local/bin/unagi

# Mark as UNAGI_IMAGE.
RUN touch /UNAGI_IMAGE

################################################################################
# Experimental
################################################################################

ENV SHELL=/bin/bash
RUN echo 'PS1="\e[0;32m\]\u@unagi\[\e[m\]:\e[0;34m\]\w\[\e[m\]# "' \
    >> /root/.bashrc

################################################################################
# Entrypoint
################################################################################

CMD /bin/bash --login

FROM imos/icfpc2020:bootstrap

################################################################################
# Environment variables (required for installation)
# NOTE: environment variables not required for installation can be placed in
# `Configurations` section.
################################################################################

ENV PATH=/work/bin:/usr/local/cargo/bin:/usr/local/go/bin:$PATH
ENV GOROOT=/usr/local/go
ENV GOPATH=/go
ENV RUSTUP_HOME=/usr/local/rustup
ENV CARGO_HOME=/usr/local/cargo

# Install C++.
RUN apt-get update -q && apt-get install -qy clang clang-format g++ && \
    apt-get clean -q && rm -rf /var/lib/apt/lists/*

# Install C#.
RUN apt-get update -q && apt-get install -qy gnupg ca-certificates && \
    apt-key adv --keyserver hkp://keyserver.ubuntu.com:80 \
        --recv-keys 3FA7E0328081BFF6A14DA29AA6A19B38D3D831EF && \
    echo "deb https://download.mono-project.com/repo/ubuntu stable-focal main" \
        > /etc/apt/sources.list.d/mono-official-stable.list && \
    apt-get update -qy && apt-get install -qy mono-devel && \
    apt-get clean -q && rm -rf /var/lib/apt/lists/*

# Install Java.
RUN apt-get update -q && apt-get install -qy default-jre default-jdk && \
    apt-get clean && rm -rf /var/lib/apt/lists/*

# Install Bazel.
RUN apt-get update -q && apt-get install -qy curl gnupg && \
    curl https://bazel.build/bazel-release.pub.gpg | apt-key add - && \
    echo "deb [arch=amd64] https://storage.googleapis.com/bazel-apt stable jdk1.8" \
        > /etc/apt/sources.list.d/bazel.list && \
    apt-get update -qy && apt-get install -qy bazel && \
    apt-get clean -q && rm -rf /var/lib/apt/lists/*

# Install Rust.
RUN curl -o rustup-init --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs && \
    chmod +x rustup-init && ./rustup-init -y --no-modify-path && \
    rm -f ./rustup-init && chmod -R a+w $RUSTUP_HOME $CARGO_HOME && \
    rustup --version && cargo --version && rustc --version

# Install Go.
RUN curl -o go.tar.gz https://dl.google.com/go/go1.14.4.linux-amd64.tar.gz && \
    tar -xf go.tar.gz && \
    mv go /usr/local/ && \
    rm go.tar.gz && \
    mkdir -p /go/src && \
    echo 'GOROOT="/usr/local/go"' >> /etc/environment && \
    echo 'GOPATH="/go"' >> /etc/environment

# Install scripts (python, php, ruby).
RUN apt-get update && DEBIAN_FRONTEND=noninteractive apt-get install -y \
        php-cli php-mysql php-curl php-pear \
        python3 python3-pip ruby && \
    apt-get clean && rm -rf /var/lib/apt/lists/*

# Install other useful tools.
RUN apt-get update && apt-get install -y \
        build-essential devscripts ubuntu-standard software-properties-common \
        screen lxc traceroute gdb \
        vim git subversion mercurial cmake make \
        dos2unix nkf curl xz-utils graphviz imagemagick \
        openssh-server sudo autoconf automake libtool make unzip net-tools && \
    mkdir -p /var/run/sshd && \
    apt-get clean && rm -rf /var/lib/apt/lists/*

# Install protobuf.
RUN apt-get update -q && apt-get install -qy \
        libprotobuf-dev libprotoc-dev protobuf-compiler && \
    apt-get clean && rm -rf /var/lib/apt/lists/*

# Set locale to suppress an sshd warning.
RUN echo 'LANG="en_US.UTF-8"' > /etc/default/locale

# Install additional packages.
RUN apt-get update && apt-get install -y libssl-dev && \
    apt-get clean && rm -rf /var/lib/apt/lists/*

# Install gcloud.
RUN echo "deb [signed-by=/usr/share/keyrings/cloud.google.gpg]" \
        "https://packages.cloud.google.com/apt cloud-sdk main" \
        >> /etc/apt/sources.list.d/google-cloud-sdk.list && \
    curl https://packages.cloud.google.com/apt/doc/apt-key.gpg | \
        apt-key --keyring /usr/share/keyrings/cloud.google.gpg add - && \
    apt-get update -qy && \
    apt-get install -qy google-cloud-sdk google-cloud-sdk-app-engine-python \
        google-cloud-sdk-app-engine-python-extras \
        google-cloud-sdk-app-engine-java google-cloud-sdk-app-engine-go \
        google-cloud-sdk-bigtable-emulator google-cloud-sdk-cbt \
        google-cloud-sdk-cloud-build-local google-cloud-sdk-datalab \
        google-cloud-sdk-datastore-emulator \
        google-cloud-sdk-firestore-emulator google-cloud-sdk-pubsub-emulator \
        kubectl && \
    wget https://dl.google.com/cloudsql/cloud_sql_proxy.linux.amd64 \
        -O /usr/local/bin/cloud_sql_proxy && \
    chmod +x /usr/local/bin/cloud_sql_proxy && \
    apt-get clean && rm -rf /var/lib/apt/lists/*

# Install AWS CLI.
RUN python3 -m pip install awscli

# Install Docker.
RUN curl -fsSL https://download.docker.com/linux/ubuntu/gpg | \
        apt-key add - && \
    add-apt-repository "$(echo \
        "deb [arch=amd64] https://download.docker.com/linux/ubuntu" \
        "$(lsb_release -cs)" stable)" && \
    apt-get update -qqy && \
    apt-get install -qyy docker-ce docker-ce-cli containerd.io && \
    curl -L https://github.com/docker/compose/releases/download/1.26.2/docker-compose-`uname -s`-`uname -m` \
        -o /usr/local/bin/docker-compose && \
    chmod +x /usr/local/bin/docker-compose && \
    apt-get clean && rm -rf /var/lib/apt/lists/*

# Install nodejs.
# RUN apt-get update && apt-get install -y nodejs-dev npm && \
#     apt-get clean && rm -rf /var/lib/apt/lists/*

# Install sshfs.
RUN apt-get update && apt-get install -y jq sshfs && \
    apt-get clean && rm -rf /var/lib/apt/lists/*

# Install protoc-gen-go.
RUN go get github.com/golang/protobuf/protoc-gen-go && \
    go install github.com/golang/protobuf/protoc-gen-go
ENV PATH=$PATH:/go/bin

# Install dependency for lambda-cli.
RUN python3 -m pip install werkzeug json-rpc jsonrpc-requests cachetools

################################################################################
# Configurations
################################################################################

# Gcloud service account.
ADD ./service_account.json /root/.config/service_account.json
ADD ./service_account.p12 /root/.config/service_account.p12
ADD ./service_account.pem /root/.config/service_account.pem
RUN gcloud auth activate-service-account \
    docker@icfpc-primary.iam.gserviceaccount.com \
    --key-file=/root/.config/service_account.json && \
    gcloud config set project icfpc-primary && \
    gcloud config set compute/region asia-northeast1 && \
    gcloud config set compute/zone asia-northeast1-a

# SSH settings.
ADD ./id_ed25519 /root/.ssh/id_ed25519
ADD ./id_ed25519 /root/.ssh/google_compute_engine
ADD ./id_ed25519 /home/unagi/.ssh/id_ed25519
ADD ./id_ed25519 /home/unagi/.ssh/google_compute_engine
RUN chmod 400 \
    /root/.ssh/id_ed25519 \
    /root/.ssh/google_compute_engine \
    /home/unagi/.ssh/id_ed25519 \
    /home/unagi/.ssh/google_compute_engine
ADD ./id_ed25519.pub /root/.ssh/authorized_keys
ADD ./id_ed25519.pub /root/.ssh/id_ed25519.pub
ADD ./id_ed25519.pub /root/.ssh/google_compute_engine.pub
ADD ./id_ed25519.pub /home/unagi/.ssh/authorized_keys
ADD ./id_ed25519.pub /home/unagi/.ssh/id_ed25519.pub
ADD ./id_ed25519.pub /home/unagi/.ssh/google_compute_engine.pub
ADD ./ssh_config /root/.ssh/config
ADD ./ssh_config /home/unagi/.ssh/config
RUN ssh-keyscan github.com >> /root/.ssh/known_hosts
RUN ssh-keyscan github.com >> /home/unagi/.ssh/known_hosts
RUN chown -R unagi:unagi /home/unagi/.ssh

# Git settings.
RUN git config --global user.email \
        '5896564+ninetan@users.noreply.github.com' && \
    git config --global user.name 'Ninetan'

################################################################################
# Experimental
################################################################################

ENV CARGO_TARGET_DIR=/work/build
ENV RUST_BACKTRACE=1

ADD ./init-wrapper /usr/local/bin/init-wrapper
RUN chmod +x /usr/local/bin/init-wrapper

################################################################################
# Repository pull
################################################################################

# Download repository.
RUN git clone git@github.com:imos/icfpc2020.git /repo

# Fill quick survey.
RUN echo "last_answer_survey_time: $(date '+%s')" > \
    /root/.config/gcloud/.last_survey_prompt.yaml

CMD /bin/bash --login
ENTRYPOINT [ "/usr/local/bin/init-wrapper" ]

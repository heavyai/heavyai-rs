# Use omniscidb build image, which includes thrift, curl, perl
FROM omnisci/core-build-ubuntu1804-cuda10.1

RUN curl --proto '=https' --tlsv1.2 -sSf \
    https://sh.rustup.rs -o /tmp/installrust.sh && \
    sh /tmp/installrust.sh -y
ENV PATH+=/home/user/.cargo/bin

# RUN useradd -ms /bin/bash user
# USER user
# WORKDIR /home/user

RUN echo "source /usr/local/mapd-deps/mapd-deps.sh" >> "$HOME/.bashrc"

RUN mkdir /src
WORKDIR /src

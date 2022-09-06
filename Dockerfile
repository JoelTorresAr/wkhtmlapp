FROM rust:1.63.0-buster

# Update default packages
RUN apt-get update

# Get Ubuntu packages
RUN apt-get install -y \
    build-essential \
    curl

# Update new packages
RUN apt-get update

# Get Rust
RUN curl https://sh.rustup.rs -sSf | bash -s -- -y

RUN echo 'source $HOME/.cargo/env' >> $HOME/.bashrc
# Download and install wkhtmltopdf dependencies.
# Alpine 3.11 and higher versions have libstdc++ v9+ in their repositories which breaks the build

ARG  jpeg=libjpeg-dev
ARG  python=python
ARG  ssl=libssl-dev
ENV  CFLAGS=-w CXXFLAGS=-w

RUN apt-get update && apt-get install -y -q --no-install-recommends \
    build-essential \
    libfontconfig1-dev \
    libfreetype6-dev \
    $jpeg \
    libpng-dev \
    $ssl \
    libx11-dev \
    libxext-dev \
    libxrender-dev \
    $python \
    zlib1g-dev \
    wget 
#    gdebi 
#    && rm -rf /var/lib/apt/lists/*

RUN apt-get update && apt-get install -y -q --no-install-recommends \
    xfonts-75dpi \
    xfonts-base \
    libpng16-16 \
    musl-dev

RUN apt-get install fontconfig -y
RUN wget https://github.com/wkhtmltopdf/packaging/releases/download/0.12.6-1/wkhtmltox_0.12.6-1.buster_amd64.deb
RUN dpkg -i wkhtmltox_0.12.6-1.buster_amd64.deb
RUN rm wkhtmltox_0.12.6-1.buster_amd64.deb
RUN apt-get install -f
RUN ln -s /usr/local/bin/wkhtmltopdf /usr/bin
RUN ln -s /usr/local/bin/wkhtmltoimage /usr/bin
RUN ln -s /usr/local/lib/lib* /usr/lib/
#RUN cp /usr/local/lib/lib* /usr/lib/ 

RUN ln -s /usr/lib/x86_64-linux-musl/libc.so /lib/libc.musl-x86_64.so.1

RUN mkdir -p /var/www/html
WORKDIR /var/www/html
RUN mkdir /root/.cargo
RUN touch /root/.cargo/env
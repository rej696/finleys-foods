FROM ubuntu
RUN apt-get update && apt-get upgrade -y
WORKDIR /root/install
ARG DEBIAN_FRONTEND=noninteractive
RUN apt-get install curl rlwrap Java -y
RUN curl -O https://download.clojure.org/install/linux-install-1.10.3.814.sh
RUN chmod +x linux-install-1.10.3.814.sh
RUN bash linux-install-1.10.3.814.sh
RUN apt-get install python3 -y
RUN pip3 install Flask

WORKDIR /root/


FROM ubuntu:latest

WORKDIR /app
RUN apt-get update && apt-get upgrade -y && apt-get install -y
RUN apt-get install build-essential -y

RUN apt-get install libssl-dev -y
RUN apt-get install default-mysql-server -y
RUN apt-get install default-mysql-client -y
RUN apt-get install -y sudo default-libmysqlclient-dev
RUN apt-get install -y sudo libmariadb-dev-compat libmariadb-dev
RUN apt-get install -y sudo vim
ADD target/release/mobile_api ./
ADD .env ./
COPY config ./config/
#ADD logs ./logs/
#
#COPY templates ./templates/
#
#COPY static ./static/

EXPOSE 8383

CMD ./mobile_api

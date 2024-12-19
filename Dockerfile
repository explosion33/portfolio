FROM ubuntu:24.04


# removed because causing errors with time desync and not using any ext. packages
#RUN apt-get update -y

COPY public /home/public
COPY proj_images /home/proj_images
COPY blog_images /home/blog_images
COPY templates /home/templates
COPY Rocket.toml /home/Rocket.toml
COPY target/x86_64-unknown-linux-musl/release/portfolio /home/portfolio

WORKDIR /home

RUN chmod 755 /home/portfolio

ENV PATH="${PATH}:/home"

EXPOSE 80

CMD ["portfolio"]
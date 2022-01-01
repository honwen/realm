FROM chenhw2/alpine:base
LABEL MAINTAINER CHENHW2 <https://github.com/chenhw2>

RUN set -ex \
    && cd /usr/bin/ \
    && curl -skSL $(curl -skSL 'https://api.github.com/repos/honwen/realm/releases/latest' | sed -n '/url.*x86_64-unknown-linux-musl/{s/.*\(https:.*tar.gz\)[^\.].*/\1/p}') | tar zxv \
    && realm -V

USER nobody

ENV ARGS="-L=:6666/9.9.9.9:9953"

EXPOSE 6060/tcp 6060/udp

CMD /usr/bin/realm ${ARGS}

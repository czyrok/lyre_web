FROM alpine:3.22.1

RUN \
  apk update \
  && apk add --no-cache bash

COPY --from=docker:28.3.2-dind-alpine3.22 /usr/local/bin/docker /usr/local/bin/
COPY --from=rclone/rclone:sha-d2d664d /usr/local/bin/rclone /usr/local/bin/

WORKDIR /code

COPY --chmod=700 image_fetcher_entrypoint.sh .

VOLUME /image_dir

ENTRYPOINT ["/code/image_fetcher_entrypoint.sh"]

#!/usr/bin/env bash

current_script_path=$(realpath $(dirname $0))

log() {
  local log_date=$(date -u +"%Y-%m-%dT%H:%M:%SZ")

  echo "[$log_date] $@"
}

die() {
  log >&2 "$@"

  exit 1
}

handle_sigterm() {
  log "Received SIGTERM, exiting..."

  exit 0
}

trap 'handle_sigterm' SIGTERM

if [[ -z "$IMAGE_NAME" ]]; then
  die "Missing 'IMAGE_NAME' session variable, exiting..."
fi

if [[ -z "$IMAGE_REGISTRY_BUCKET_NAME" ]]; then
  die "Missing 'IMAGE_REGISTRY_BUCKET_NAME' session variable, exiting..."
fi

if [[ -z "$IMAGE_REGISTRY_BUCKET_REGION" ]]; then
  die "Missing 'IMAGE_REGISTRY_BUCKET_REGION' session variable, exiting..."
fi

if [[ -z "$IMAGE_REGISTRY_BUCKET_ACCESS_KEY_ID" ]]; then
  die "Missing 'IMAGE_REGISTRY_BUCKET_ACCESS_KEY_ID' session variable, exiting..."
fi

if [[ -z "$IMAGE_REGISTRY_BUCKET_ACCESS_KEY_SECRET" ]]; then
  die "Missing 'IMAGE_REGISTRY_BUCKET_ACCESS_KEY_SECRET' session variable, exiting..."
fi

write_rclone_configuration() {
  local rclone_configuration_path="$current_script_path/rclone.conf"

  echo "[$IMAGE_REGISTRY_BUCKET_NAME]" > "$rclone_configuration_path"
  echo "type = s3" >> "$rclone_configuration_path"
  echo "provider = Other" >> "$rclone_configuration_path"
  echo "acl = private" >> "$rclone_configuration_path"
  echo "endpoint = https://s3.${IMAGE_REGISTRY_BUCKET_REGION,,}.io.cloud.ovh.net" >> "$rclone_configuration_path"
  echo "region = ${IMAGE_REGISTRY_BUCKET_REGION,,}" >> "$rclone_configuration_path"
  echo "location_constraint = ${IMAGE_REGISTRY_BUCKET_REGION,,}" >> "$rclone_configuration_path"
  echo "access_key_id = $IMAGE_REGISTRY_BUCKET_ACCESS_KEY_ID" >> "$rclone_configuration_path"
  echo "secret_access_key = $IMAGE_REGISTRY_BUCKET_ACCESS_KEY_SECRET" >> "$rclone_configuration_path"

  export RCLONE_CONFIG="$current_script_path/rclone.conf"
}

fetch_image() {
  local is_image_found=$(rclone ls "$IMAGE_REGISTRY_BUCKET_NAME:$IMAGE_REGISTRY_BUCKET_NAME/$IMAGE_NAME")

  if [[ -z "$is_image_found" ]]; then
    log "warn: The image '$IMAGE_NAME' doesn't exist on the bucket registry"

    return 0
  fi

  if [[ -f "/image_dir/$IMAGE_NAME" ]]; then
    local_image_checksum=$(md5sum "/image_dir/$IMAGE_NAME" | awk '{print $1}')

    remote_image_checksum=$(rclone md5sum "$IMAGE_REGISTRY_BUCKET_NAME:$IMAGE_REGISTRY_BUCKET_NAME/$IMAGE_NAME" | awk '{print $1}')

    if [ "$local_image_checksum" == "$remote_image_checksum" ]; then
      return 0
    fi

    log "Remote image differs from local one"
  fi

  log "Fetching '$IMAGE_NAME' from bucket registry..."
  rclone copyto "$IMAGE_REGISTRY_BUCKET_NAME:$IMAGE_REGISTRY_BUCKET_NAME/$IMAGE_NAME" "/image_dir/$IMAGE_NAME"

  log "Loading '$IMAGE_NAME' into Docker..."
  docker image load --input "/image_dir/$IMAGE_NAME"
}

write_rclone_configuration

if [ "$RUN_ONCE" = "true" ]; then
  log "warn: Run once mode enabled"

  fetch_image
else
  while true; do
    fetch_image

    sleep 30 # seconds
  done
fi

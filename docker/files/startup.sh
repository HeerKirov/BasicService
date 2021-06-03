#!/bin/sh

envsubst < config.properties.template > config.properties

/release/basic_service runserver
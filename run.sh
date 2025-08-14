set -e # Exit early if any commands fail
exec node app/index.js "$@"
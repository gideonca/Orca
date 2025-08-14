set -e # Exit early if any commands fail
exec node test/client.js "$@"
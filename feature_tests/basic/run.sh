eval $(nsw env)
nsw install v8.11.3
nsw use v8.11.3

if [ "$(node --version)" != "v8.11.3" ]; then
  echo "Node version is not v8.11.3!"
  exit 1
fi
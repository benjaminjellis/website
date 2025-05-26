#!/bin/bash

PORT_TO_KILL=${1:-3000}

echo "Attempting to kill process on TCP port: $PORT_TO_KILL"

PID=$(lsof -t -i tcp:$PORT_TO_KILL)

if [ -z "$PID" ]; then
  echo "No process found listening on TCP port $PORT_TO_KILL."
else
  echo "Found process with PID: $PID on port $PORT_TO_KILL."
  echo "Attempting to forcefully kill PID: $PID..."
  kill -9 "$PID"

  if ! lsof -t -i tcp:$PORT_TO_KILL &>/dev/null; then
    echo "Process with PID $PID on port $PORT_TO_KILL has been killed successfully."
  else
    echo "Warning: Process with PID $PID on port $PORT_TO_KILL might not have been killed."
    echo "You may need to investigate manually."
  fi
fi

echo "Waiting for 10 seconds"
sleep 10
echo "Done waiting"

max_attempts=5

attempt_num=1

success=false

while [ $success = false ] && [ $attempt_num -le $max_attempts ]; do
  cp new_artefacts/website .

  if [ $? -eq 0 ]; then
    success=true
  else
    echo "Attempt $attempt_num failed. Trying again..."
    attempt_num=$((attempt_num + 1))
  fi
done

rm -rf static
mkdir static
cp -r new_artefacts/static .

rm -rf new_artefacts

./website >/dev/null 2>&1 &
exit

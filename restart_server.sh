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

echo "Waiting for 60 seconds"
sleep 60
echo "Done waiting"

# Set the maximum number of attempts
max_attempts=5

# Set a counter for the number of attempts
attempt_num=1

# Set a flag to indicate whether the command was successful
success=false

# Loop until the command is successful or the maximum number of attempts is reached
while [ $success = false ] && [ $attempt_num -le $max_attempts ]; do
  cp new_artefacts/website .

  # Check the exit code of the command
  if [ $? -eq 0 ]; then
    # The command was successful
    success=true
  else
    # The command was not successful
    echo "Attempt $attempt_num failed. Trying again..."
    # Increment the attempt counter
    attempt_num=$((attempt_num + 1))
  fi
done

rm -rf static
mkdir static
cp -r new_artefacts/static .

rm -rf new_artefacts

echo "Running website using nohup"
./website >/dev/null 2>&1 &
echo "done"
exit

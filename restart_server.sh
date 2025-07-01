#!/bin/bash

echo "Attempting to kill 'website' process"

PID=$(pgrep website)

if [ -z "$PID" ]; then
  echo "No process found with name 'website' "
else
  echo "Found process 'website' with PID: $PID"
  echo "Attempting to forcefully kill PID: $PID..."
  kill -9 "$PID"

  if ! pgrep website &>/dev/null; then
    echo "Process 'website' successfully killed"
  else
    echo "Warning: Process 'webite' might not have been killed."
    echo "You may need to investigate manually."
    exit 1
  fi
fi

echo "Waiting for 10 seconds"
sleep 60
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

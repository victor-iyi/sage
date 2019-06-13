#!/bin/bash

# --- RUNNING POSTGRESS ---
# You might see

#     FATAL:  lock file "postmaster.pid" already exists
#     HINT:  Is another postmaster (PID 449) running in data directory "/usr/local/var/postgres"?

# Then try

# kill -9 PID

# Example

# kill -9 449

echo "Starting postgres..."
# Run postgres db.
postgres -D /usr/local/var/postgres

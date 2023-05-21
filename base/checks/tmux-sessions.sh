#!/bin/bash
usersArr=$(awk -F: '$NF!~/\/false$/ && $NF!~/\/nologin$/ && $6~/\/home/{print $1}' /etc/passwd) # get all users with a login shell and home folder
usersArr+=("root") # + root to the array

c=0 # a counter for the number of users with active tmux sessions
for u in ${usersArr[@]}; do
	if runuser -l $u -c 'tmux list-sessions'; then
		c=$((c+1))
		break # no need to continue the loop, if there is any user with a tmux session, the PC is in use and should report 'active' state
	fi
done
if [ $c -ne 0 ]; then
	exit 0
else
	exit 1
fi

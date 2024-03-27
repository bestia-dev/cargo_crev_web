#!/bin/sh

# cargo_crev_web_publish.sh

printf "rsync -avz --delete-after --exclude 'blocklisted_repos.json' /tmp/bestia-dev/cargo_crev_web/uploaded_web_server_folder/ /var/www/webapps/cargo_crev_web/\n"
rsync -avz --delete-after --exclude 'blocklisted_repos.json' /tmp/bestia-dev/cargo_crev_web/uploaded_web_server_folder/ /var/www/webapps/cargo_crev_web/
printf "screen -dr cargo_crev_web_8051 -X stuff $'\003'\n"
screen -dr cargo_crev_web_8051 -X stuff $'\003'
printf "screen -dr cargo_crev_web_8051 -X stuff 'cd /var/www/webapps/cargo_crev_web'\n"
screen -dr cargo_crev_web_8051 -X stuff 'cd /var/www/webapps/cargo_crev_web\n'
printf "screen -dr cargo_crev_web_8051 -X stuff 'chmod +x cargo_crev_web'\n"
screen -dr cargo_crev_web_8051 -X stuff 'chmod +x cargo_crev_web\n'
# don't use sudo here, because .cache is under luciano_bestia user, this user must run the binary
printf "screen -dr cargo_crev_web_8051 -X stuff './cargo_crev_web'\n"
screen -dr cargo_crev_web_8051 -X stuff './cargo_crev_web\n'
printf "completed\n"
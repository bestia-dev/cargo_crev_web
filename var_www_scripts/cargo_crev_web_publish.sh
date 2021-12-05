#!/bin/sh
echo "rsync -avz --delete-after /var/www/transfer_folder/webapps/cargo_crev_web/ /var/www/webapps/cargo_crev_web/"
rsync -avz --delete-after /var/www/transfer_folder/webapps/cargo_crev_web/ /var/www/webapps/cargo_crev_web/
echo "screen -dr cargo_crev_web_8051 -X stuff $'\003'"
screen -dr cargo_crev_web_8051 -X stuff $'\003'
echo "screen -dr cargo_crev_web_8051 -X stuff 'cd /var/www/webapps/cargo_crev_web\n'"
screen -dr cargo_crev_web_8051 -X stuff 'cd /var/www/webapps/cargo_crev_web\n'
echo "screen -dr cargo_crev_web_8051 -X stuff 'chmod +x cargo_crev_web\n'"
screen -dr cargo_crev_web_8051 -X stuff 'chmod +x cargo_crev_web\n'
# don't use sudo here, because .cache is under luciano_bestia user, this user must run the binary
echo "screen -dr cargo_crev_web_8051 -X stuff './cargo_crev_web\n'"
screen -dr cargo_crev_web_8051 -X stuff './cargo_crev_web\n'
echo "completed"
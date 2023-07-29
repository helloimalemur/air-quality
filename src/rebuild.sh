#!/bin/bash
systemctl stop airquality;
cd /srv/http/vhosts/air-quality/;
git stash;
git stash drop;
git pull origin master;
cargo build;
systemctl start airquality;
systemctl status airquality;
#journalctl -f -u airquality;

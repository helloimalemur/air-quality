#!/bin/bash
USER=$(whoami);
SERVICE_USER="myq";
sudo systemctl stop myq;
if [ -f "/usr/share/myq/host.json" ]; then echo "copying /usr/share/myq/host.json to ~/host.json" && sudo cp /usr/share/myq/host.json /home/"$USER"/host.json; fi;
if [ -f "/usr/share/myq/myq.yml" ]; then echo "copying /usr/share/myq/myq.yml to ~/myq.yml" && sudo cp /usr/share/myq/myq.yml /home/"$USER"/myq.yml; fi;
sudo rm -rf /usr/share/myq/;
sudo mkdir /usr/share/myq/;
#sudo useradd myq;
sudo passwd -l myq;
sudo su -c 'pip install pymyq' $SERVICE_USER;
sudo su -c 'pip install aiohttp' $SERVICE_USER;
#cd /usr/share/myq/;
sudo git clone https://github.com/helloimalemur/rust-myq-garage.git /usr/share/myq/rust-myq-garage/;
sudo chown -R "$SERVICE_USER":"$SERVICE_USER" /usr/share/myq/;
sudo chmod -R 0700 /usr/share/myq/;
#cd rust-myq-garage/;
sudo su -c 'cargo build --manifest-path=/usr/share/myq/rust-myq-garage/Cargo.toml' myq;
sudo cp /usr/share/myq/rust-myq-garage/target/debug/rust-myq-garage /usr/share/myq/myq;
sudo cp src/assets/myq.service /etc/systemd/system/myq.service;
sudo systemctl daemon-reload;
sudo cp src/assets/* /usr/share/myq/;
if [ -f "/home/"$USER"/host.json" ]; then echo "copying ~/host.json to /usr/share/myq/host.json"; else sudo rm /usr/share/myq/host.json && sudo cp /home/"$USER"/host.json /usr/share/myq/host.json; fi;
if [ -f "/home/"$USER"/myq.yml" ]; then echo "copying ~/myq.yml to /usr/share/myq/myq.yml"; else sudo rm /usr/share/myq/myq.yml && sudo cp /home/"$USER"/myq.yml /usr/share/myq/myq.yml; fi;
sudo chown -R "$SERVICE_USER":"$SERVICE_USER" /usr/share/myq/;
sudo chmod -R 0700 /usr/share/myq/;
sudo systemctl start myq;
sudo systemctl status myq;
sudo tail -f /usr/share/myq/running.log

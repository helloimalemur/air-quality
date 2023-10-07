#!/bin/bash
USER=$(whoami);
SERVICE_USER="airquality";
sudo systemctl stop airquality;
#if [ -f "/usr/share/airquality/host.json" ]; then echo "copying /usr/share/airquality/host.json to ~/host.json" && sudo cp /usr/share/airquality/host.json /home/"$USER"/host.json; fi;
#if [ -f "/usr/share/airquality/airquality.yml" ]; then echo "copying /usr/share/airquality/airquality.yml to ~/airquality.yml" && sudo cp /usr/share/airquality/airquality.yml /home/"$USER"/airquality.yml; fi;
sudo rm -rf /usr/share/airquality/;
sudo mkdir /usr/share/airquality/;
sudo useradd airquality;
sudo passwd -l airquality;
#sudo su -c 'pip install pyairquality' $SERVICE_USER;
#sudo su -c 'pip install aiohttp' $SERVICE_USER;
cd /usr/share/airquality/;

sudo chown -R "$SERVICE_USER":"$SERVICE_USER" /usr/share/airquality/;
sudo chmod -R 0700 /usr/share/airquality/;

sudo su -c 'cargo build --release' airquality;
sudo cp /usr/share/airquality/target/release/airquality /usr/share/airquality/airquality;
sudo cp src/assets/airquality.service /etc/systemd/system/airquality.service;
sudo systemctl daemon-reload;
sudo cp src/assets/* /usr/share/airquality/;
if [ -f "/home/"$USER"/host.json" ]; then echo "copying ~/host.json to /usr/share/airquality/host.json"; else sudo rm /usr/share/airquality/host.json && sudo cp /home/"$USER"/host.json /usr/share/airquality/host.json; fi;
if [ -f "/home/"$USER"/airquality.yml" ]; then echo "copying ~/airquality.yml to /usr/share/airquality/airquality.yml"; else sudo rm /usr/share/airquality/airquality.yml && sudo cp /home/"$USER"/airquality.yml /usr/share/airquality/airquality.yml; fi;
sudo chown -R "$SERVICE_USER":"$SERVICE_USER" /usr/share/airquality/;
sudo chmod -R 0700 /usr/share/airquality/;
sudo systemctl start airquality;
sudo systemctl status airquality;
sudo tail -f /usr/share/airquality/running.log
npm install react-bootstrap bootstrap

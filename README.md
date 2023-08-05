# air-quality
### WORK IN PROGRESS
#### Notifications and Web Interface to notify subscribers of questionable/hazardous air quality in their area, using data from https://www.iqair.com/
###### Project uses IQair's Community tier API.

[x] Add sub \
[x] Fetch & store air quality reading \
[x] Loop on subs and send alerts \
[x] Alerts for discord \
[ ] email alerts \
[ ] mobile app?

#### todo
    alert using preferred alert type if over threshold
    alerts (discord,email,sms)
    fetch per location - for subs, currently only fetches current location
    web interface


## Create ./config/Settings.toml
```toml
database_url = "mysql://user:password@domain.com:3306/database"
database_name = "database"
api_key = "yourapikey"
iqair_key = ""
port = 8080
```


## database user
    CREATE USER 'dev'@'%' IDENTIFIED WITH sha256_password BY 'password';
    CREATE USER 'dev'@'%' IDENTIFIED BY 'password';
    GRANT ALL PRIVILEGES ON database.* TO 'dev'@'%';
    FLUSH PRIVILEGES;




## database tables
```mysql
CREATE TABLE `readings` (
    `id` int(11) NOT NULL AUTO_INCREMENT,
    `city` varchar(255) NOT NULL,
    `state` varchar(255) NOT NULL,
    `temp` varchar(255) NOT NULL,
    `pressure` varchar(255) NOT NULL,
    `humidity` varchar(255) NOT NULL,
    `wind_speed` varchar(255) NOT NULL,
    `current_pollution_aqius` varchar(255) NOT NULL,
    `main_pollutant` varchar(255) NOT NULL,
    PRIMARY KEY (`id`));


CREATE TABLE `subs` (
    `id` int(11) NOT NULL AUTO_INCREMENT,
    `email` varchar(255) NOT NULL,
    `discord` varchar(255) NOT NULL,
    `additional_details` varchar(255) NOT NULL,
    PRIMARY KEY (`id`));
```

### resources
    https://api-docs.iqair.com/

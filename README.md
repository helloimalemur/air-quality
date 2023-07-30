# air-quality
Notifications and Web Interface to notify subscribers of questionable/hazardous air quality in their area, using data from https://www.iqair.com/


Create ./config/Settings.toml
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



### resources
    https://api-docs.iqair.com/




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

# air-quality
Notifications and Web Interface to notify subscribers of questionable/hazardous air quality in their area, using data from https://www.iqair.com/


Create ./config/Settings.toml
```toml
database_url = "mysql://user:password@domain.com:3306/database"
database_name = "database"
api_key = "yourapikey"
port = 8080
```


## database user
    CREATE USER 'dev'@'%' IDENTIFIED WITH sha256_password BY 'password';
    CREATE USER 'dev'@'%' IDENTIFIED BY 'password';
    GRANT ALL PRIVILEGES ON database.* TO 'dev'@'%';
    FLUSH PRIVILEGES;

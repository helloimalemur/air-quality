# air-quality
### WORK IN PROGRESS
#### Notifications and Web Interface to notify subscribers of questionable/hazardous air quality in their area, using data from https://www.iqair.com/
###### Project uses IQair's Community tier API.

### To Do
[x] Add sub \
[x] Fetch & store air quality reading \
[x] Loop on subs and send alerts \
[x] Alerts for discord \
[ ] web interface \
[ ] email alerts \
[ ] fetch per specified location \
[ ] mobile app?

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

## Example IQAPI response
```json
{
  "status": "success",
  "data": {
    "city": "Port Harcourt",
    "state": "Rivers",
    "country": "Nigeria",
    "location": {
      "type": "Point",
      "coordinates": [
        7.048623,
        4.854166
      ]
    },
    "forecasts": [
      {
        "ts": "2019-08-15T12:00:00.000Z",
        "aqius": 137,
        "aqicn": 69,
        "tp": 23,
        "tp_min": 23,
        "pr": 996,
        "hu": 100,
        "ws": 2,
        "wd": 225,
        "ic": "10d"
      },
      {
        "ts": "2019-08-18T09:00:00.000Z",
        "aqius": 158,
        "aqicn": 94,
        "tp": 24,
        "tp_min": 24,
        "pr": 995,
        "hu": 100,
        "ws": 2,
        "wd": 182,
        "ic": "10d"
      }
    ],
    "current": {
      "weather": {
        "ts": "2019-08-15T09:00:00.000Z",
        "tp": 23,
        "pr": 997,
        "hu": 100,
        "ws": 1,
        "wd": 216,
        "ic": "10d"
      },
      "pollution": {
        "ts": "2019-08-15T10:00:00.000Z",
        "aqius": 83,
        "mainus": "p2",
        "aqicn": 39,
        "maincn": "p2",
        "p2": {
          "conc": 27.2,
          "aqius": 83,
          "aqicn": 39
        }
      }
    },
    "history": {
      "weather": [
        {
          "ts": "2019-08-15T09:00:00.000Z",
          "tp": 23,
          "pr": 997,
          "hu": 100,
          "ws": 1,
          "wd": 216,
          "ic": "10d"
        },
        {
          "ts": "2019-08-15T08:00:00.000Z",
          "tp": 23,
          "pr": 997,
          "hu": 100,
          "ws": 1,
          "wd": 216,
          "ic": "10d"
        },
        {
          "ts": "2019-08-13T09:00:00.000Z",
          "tp": 22,
          "pr": 996,
          "hu": 100,
          "ws": 2,
          "wd": 227,
          "ic": "10d"
        }
      ],
      "pollution": [
        {
          "ts": "2019-08-15T10:00:00.000Z",
          "aqius": 83,
          "mainus": "p2",
          "aqicn": 39,
          "maincn": "p2",
          "p2": {
            "conc": 27.2,
            "aqius": 83,
            "aqicn": 39
          }
        },
        {
          "ts": "2019-08-15T07:00:00.000Z",
          "aqius": 102,
          "mainus": "p2",
          "aqicn": 51,
          "maincn": "p2",
          "p2": {
            "conc": 36,
            "aqius": 102,
            "aqicn": 51
          }
        }
      ]
    }
  }
}
```

### resources
    https://api-docs.iqair.com/

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
CREATE TABLE generic (
  id TEXT,
  status TEXT,
  PRIMARY KEY (id)
);

CREATE TABLE generic_data (
  generic_id TEXT,
  id TEXT,
  city TEXT,
  state TEXT,
  country TEXT,
  PRIMARY KEY (id),
  FOREIGN KEY (generic_id) REFERENCES generic(id)
);

CREATE TABLE generic_data_location (
  generic_data_id TEXT,
  id TEXT,
  type TEXT,
  PRIMARY KEY (id),
  FOREIGN KEY (generic_data_id) REFERENCES generic_data(id)
);

CREATE TABLE generic_data_location_coordinates (
  generic_data_location_id TEXT,
  id TEXT,
  value TEXT,
  PRIMARY KEY (id),
  FOREIGN KEY (generic_data_location_id) REFERENCES generic_data_location(id)
);

CREATE TABLE generic_data_forecasts (
  generic_data_id TEXT,
  id TEXT,
  ts TIMESTAMP,
  aqius INT,
  aqicn INT,
  tp INT,
  tp_min INT,
  pr INT,
  hu INT,
  ws INT,
  wd INT,
  ic TEXT,
  PRIMARY KEY (id),
  FOREIGN KEY (generic_data_id) REFERENCES generic_data(id)
);

CREATE TABLE generic_data_current (
  generic_data_id TEXT,
  id TEXT,
  PRIMARY KEY (id),
  FOREIGN KEY (generic_data_id) REFERENCES generic_data(id)
);

CREATE TABLE generic_data_current_weather (
  generic_data_current_id TEXT,
  id TEXT,
  ts TIMESTAMP,
  tp INT,
  pr INT,
  hu INT,
  ws INT,
  wd INT,
  ic TEXT,
  PRIMARY KEY (id),
  FOREIGN KEY (generic_data_current_id) REFERENCES generic_data_current(id)
);

CREATE TABLE generic_data_current_pollution (
  generic_data_current_id TEXT,
  id TEXT,
  ts TIMESTAMP,
  aqius INT,
  mainus TEXT,
  aqicn INT,
  maincn TEXT,
  PRIMARY KEY (id),
  FOREIGN KEY (generic_data_current_id) REFERENCES generic_data_current(id)
);

CREATE TABLE generic_data_current_pollution_p2 (
  generic_data_current_pollution_id TEXT,
  id TEXT,
  conc INT,
  aqius INT,
  aqicn INT,
  PRIMARY KEY (id),
  FOREIGN KEY (generic_data_current_pollution_id) REFERENCES generic_data_current_pollution(id)
);

CREATE TABLE generic_data_history (
  generic_data_id TEXT,
  id TEXT,
  PRIMARY KEY (id),
  FOREIGN KEY (generic_data_id) REFERENCES generic_data(id)
);

CREATE TABLE generic_data_history_weather (
  generic_data_history_id TEXT,
  id TEXT,
  ts TIMESTAMP,
  tp INT,
  pr INT,
  hu INT,
  ws INT,
  wd INT,
  ic TEXT,
  PRIMARY KEY (id),
  FOREIGN KEY (generic_data_history_id) REFERENCES generic_data_history(id)
);

CREATE TABLE generic_data_history_pollution (
  generic_data_history_id TEXT,
  id TEXT,
  ts TIMESTAMP,
  aqius INT,
  mainus TEXT,
  aqicn INT,
  maincn TEXT,
  PRIMARY KEY (id),
  FOREIGN KEY (generic_data_history_id) REFERENCES generic_data_history(id)
);

CREATE TABLE generic_data_history_pollution_p2 (
  generic_data_history_pollution_id TEXT,
  id TEXT,
  conc INT,
  aqius INT,
  aqicn INT,
  PRIMARY KEY (id),
  FOREIGN KEY (generic_data_history_pollution_id) REFERENCES generic_data_history_pollution(id)
);
```

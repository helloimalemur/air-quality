CREATE TABLE `airqualtiy` (
    `id` int(11) NOT NULL AUTO_INCREMENT,
    `city` varchar(255) NOT NULL,
    `state` varchar(255) NOT NULL,
    `current_weather` varchar(255) NOT NULL,
    `current_pollution_aqius` varchar(255) NOT NULL,
    PRIMARY KEY (`id`));

CREATE TABLE `subs` (
    `id` int(11) NOT NULL AUTO_INCREMENT,
    `email` varchar(255) NOT NULL,
    `discord` varchar(255) NOT NULL,
    `additional_details` varchar(255) NOT NULL,
    PRIMARY KEY (`id`));

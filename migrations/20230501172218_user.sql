CREATE TABLE `user` (
                        `id` int NOT NULL AUTO_INCREMENT,
                        `first_name` varchar(255) DEFAULT NULL,
                        `last_name` varchar(255) DEFAULT NULL,
                        `user_name` varchar(255) NOT NULL,
                        `email` varchar(255) NOT NULL,
                        `password` varchar(255) NOT NULL,
                        `created_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
                        `updated_at` timestamp NULL,
                        `is_active` tinyint(1) NOT NULL DEFAULT '0',
                        PRIMARY KEY (`id`),
                        UNIQUE KEY `user_name` (`user_name`),
                        UNIQUE KEY `email` (`email`)
) ENGINE=InnoDB AUTO_INCREMENT=0 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;
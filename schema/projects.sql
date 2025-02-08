CREATE    TABLE `projects` (
          `slug` VARCHAR(255) NOT NULL PRIMARY KEY,
          `next_slug` VARCHAR(255),
          `position` INTEGER NOT NULL,
          `title` VARCHAR(255) NOT NULL,
          `image_url` VARCHAR(255) NOT NULL,
          `date` DATE NOT NULL,
          `content` TEXT NOT NULL,
          FOREIGN KEY (`next_slug`) REFERENCES `projects` (`slug`)
          );

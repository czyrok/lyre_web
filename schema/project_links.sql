CREATE    TABLE `project_links` (
          `project_slug` VARCHAR(255) NOT NULL,
          `url` VARCHAR(255) NOT NULL,
          `title` VARCHAR(32) NOT NULL,
          ---- https://stackoverflow.com/questions/5299267/how-to-create-enum-type-in-sqlite
          `icon` VARCHAR(255) CHECK (`icon` IN ('GITHUB')) NULL DEFAULT NULL,
          CONSTRAINT `pk_project_link` PRIMARY KEY (`project_slug`, `url`) FOREIGN KEY (`project_slug`) REFERENCES `projects` (`slug`)
          );

CREATE    TABLE `project_tags` (
          `project_slug` VARCHAR(255) NOT NULL,
          `name` VARCHAR(32) NOT NULL,
          CONSTRAINT `pk_project_tag` PRIMARY KEY (`project_slug`, `name`) FOREIGN KEY (`project_slug`) REFERENCES `projects` (`slug`)
          );

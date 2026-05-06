CREATE    TABLE `project_tags` (
          `short_name` VARCHAR(32) NOT NULL PRIMARY KEY,
          `long_name` VARCHAR(32) NULL
          );


CREATE    TABLE `project_tags_projects` (
          `project_slug` VARCHAR(255) NOT NULL,
          `tag_short_name` VARCHAR(32) NOT NULL,
          CONSTRAINT `pk_project_tag` PRIMARY KEY (`project_slug`, `tag_short_name`) FOREIGN KEY (`project_slug`) REFERENCES `projects` (`slug`)
          );

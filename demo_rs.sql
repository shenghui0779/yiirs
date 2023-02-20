#
# SQL Export
# Created by Querious (303012)
# Created: February 10, 2023 at 11:25:49 GMT+8
# Encoding: Unicode (UTF-8)
#


SET @ORIG_FOREIGN_KEY_CHECKS = @@FOREIGN_KEY_CHECKS;
SET FOREIGN_KEY_CHECKS = 0;

SET @ORIG_UNIQUE_CHECKS = @@UNIQUE_CHECKS;
SET UNIQUE_CHECKS = 0;

SET @ORIG_TIME_ZONE = @@TIME_ZONE;
SET TIME_ZONE = '+00:00';

SET @ORIG_SQL_MODE = @@SQL_MODE;
SET SQL_MODE = 'NO_AUTO_VALUE_ON_ZERO';



DROP DATABASE IF EXISTS `demo_rs`;
CREATE DATABASE `demo_rs` DEFAULT CHARACTER SET utf8mb4 DEFAULT COLLATE utf8mb4_bin;
USE `demo_rs`;




DROP TABLE IF EXISTS `project`;
DROP TABLE IF EXISTS `account`;


CREATE TABLE `account` (
  `id` bigint unsigned NOT NULL AUTO_INCREMENT COMMENT '自增ID',
  `username` varchar(16) CHARACTER SET utf8mb4 COLLATE utf8mb4_bin NOT NULL DEFAULT '' COMMENT '用户名称',
  `password` varchar(32) CHARACTER SET utf8mb4 COLLATE utf8mb4_bin NOT NULL DEFAULT '' COMMENT '用户密码',
  `salt` varchar(16) CHARACTER SET utf8mb4 COLLATE utf8mb4_bin NOT NULL DEFAULT '' COMMENT '加密盐',
  `role` tinyint NOT NULL DEFAULT '0' COMMENT '角色：1 - 普通；2 - 管理员',
  `realname` varchar(16) CHARACTER SET utf8mb4 COLLATE utf8mb4_bin NOT NULL DEFAULT '' COMMENT '真实姓名',
  `login_at` bigint NOT NULL DEFAULT '0' COMMENT '最近一次登录时间',
  `login_token` varchar(32) CHARACTER SET utf8mb4 COLLATE utf8mb4_bin NOT NULL DEFAULT '' COMMENT '当前登录的token',
  `created_at` bigint NOT NULL DEFAULT '0' COMMENT '创建时间',
  `updated_at` bigint NOT NULL DEFAULT '0' COMMENT '更新时间',
  PRIMARY KEY (`id`),
  UNIQUE KEY `uniq_username` (`username`) USING BTREE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_bin COMMENT='账号表';

CREATE TABLE `project` (
  `id` bigint unsigned NOT NULL AUTO_INCREMENT COMMENT '自增ID',
  `code` varchar(8) CHARACTER SET utf8mb4 COLLATE utf8mb4_bin NOT NULL DEFAULT '' COMMENT '项目编号',
  `name` varchar(32) CHARACTER SET utf8mb4 COLLATE utf8mb4_bin NOT NULL DEFAULT '' COMMENT '项目名称',
  `remark` varchar(64) CHARACTER SET utf8mb4 COLLATE utf8mb4_bin NOT NULL DEFAULT '' COMMENT '项目备注',
  `account_id` bigint unsigned NOT NULL DEFAULT '0' COMMENT '创建账号ID',
  `created_at` bigint NOT NULL DEFAULT '0' COMMENT '创建时间',
  `updated_at` bigint NOT NULL DEFAULT '0' COMMENT '更新时间',
  PRIMARY KEY (`id`),
  UNIQUE KEY `uniq_code` (`code`) USING BTREE,
  KEY `idx_account` (`account_id`) USING BTREE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_bin COMMENT='项目表';





LOCK TABLES `account` WRITE;
TRUNCATE `account`;
INSERT INTO `account` (`id`, `username`, `password`, `salt`, `role`, `realname`, `login_at`, `login_token`, `created_at`, `updated_at`) VALUES
	(1,'admin','e03dcdf34a257041b36bd77132130fdc','LCV8xdTcqmkhA$ze',2,'Administrator',1675941517,'cc3e49577201323b0010815f2485acd9',1675941476,1675941517);
UNLOCK TABLES;





SET FOREIGN_KEY_CHECKS = @ORIG_FOREIGN_KEY_CHECKS;

SET UNIQUE_CHECKS = @ORIG_UNIQUE_CHECKS;

SET @ORIG_TIME_ZONE = @@TIME_ZONE;
SET TIME_ZONE = @ORIG_TIME_ZONE;

SET SQL_MODE = @ORIG_SQL_MODE;



# Export Finished: February 10, 2023 at 11:25:49 GMT+8


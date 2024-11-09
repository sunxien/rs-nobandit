

create database `nobandit` default charset utf8mb4;

use `nobandit`;

--
create table if not exists `config` (
    `id` bigint unsigned not null primary key auto_increment comment '主键',
    `garage_name` varchar(100) not null comment '车库名称',
    `garage_type` tinyint not null comment '车库类型：0:公共车库，1: 私有车库',
    `free_time` smallint unsigned not null comment '免费时长（单位：小时）',
    `parking_price` smallint unsigned not null comment '停车价格（单位：元/小时）',
    `max_stay_time` smallint unsigned not null comment '最长滞留时间（单位：分钟）',
    unique(`garage_name`)
) engine=innodb charset=utf8mb4 comment '系统配置表';

insert into `config` values (1, '北京市碧水蓝天SPA会馆地下车库', 0, 2, 50, 30);
insert into `config` values (1, '北京市朝阳区广渠金茂府地下车库', 1, 0, 10, 30);

--
create table if not exists `registered` (
    `id` bigint unsigned not null primary key auto_increment comment '主键',
    `owner` varchar(50) not null comment '车主',
    `phone` varchar(50) not null comment '车主联系方式',
    `address` varchar(50) not null comment '车主家庭住址',
    `license_plate` varchar(50) not null comment '车牌',
    unique(`license_plate`),
    key(`license_plate`)
) engine=innodb charset=utf8mb4 comment '车辆登记表';

insert into `registered` (`owner`,`phone`,`address`,`license_plate`)
values ('李国良', '(+86)18203478809', '17幢-2单元-3201室', '京A-76B33');

--
create table if not exists `history` (
    `id` bigint unsigned not null primary key auto_increment comment '主键',
    `license_plate` varchar(50) not null comment '车牌',
    `enter_time` timestamp not null default current_timestamp comment '车辆进场时间',
    `exit_time` timestamp not null default current_timestamp comment '车辆出场时间',
    `status` tinyint not null default 1 comment '1:已进场, 2:待支付, 3:已支付，未出场, 4:已支付，已出场'
) engine=innodb charset=utf8mb4 comment '车辆出入历史表';

-- 模拟进场
insert into `history` (`license_plate`) values ('京F-88531');

-- 进场
-- insert into `history` (`license_plate`) values (?);

-- 计算费用
-- select timestampdiff(SECOND,`enter_time`,`exit_time`) as stay_time from `history` where `license_plate`=?;

-- 出场
-- update `history` set `exit_time`=current_timestamp where `license_plate`=?;

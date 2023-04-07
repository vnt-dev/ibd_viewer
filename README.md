# ibd_viewer
mysql innodb .ibd文件解析器

### 示例
```
CREATE TABLE `test_index_1` (
  `id` bigint NOT NULL AUTO_INCREMENT COMMENT '主键',
  `a` int NOT NULL,
  `str1` varchar(255) DEFAULT NULL COMMENT 'str1',
  `str2` varchar(255) NOT NULL COMMENT 'str2',
  PRIMARY KEY (`id`),
  UNIQUE KEY `str2_key` (`str2`),
  KEY `a_key` (`a`),
  KEY `str1_key` (`str1`)
) ENGINE=InnoDB AUTO_INCREMENT=2 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci
```
#### 查看所有页的页号和类型
![图片](https://user-images.githubusercontent.com/49143209/230388154-dd95f714-c10a-40fe-824d-19fbc5c0a6ee.png)
#### 查看索引根
![图片](https://user-images.githubusercontent.com/49143209/230390654-4fd85882-2afc-4728-860b-bb4ba8ba47cf.png)
#### 查看具体页的数据,可以看到行记录
![图片](https://user-images.githubusercontent.com/49143209/230541089-fbbbc614-a846-4807-87f7-3536c3336880.png)

### 编译

1. 安装rust环境 https://www.rust-lang.org/zh-CN/tools/install
2. 到项目目录下执行 cargo build

### 支持

1. MySQL 8.0
2. 行格式为Compact、Dynamic 和 Compressed
3. B+树节点、FilPageTypeFspHdr

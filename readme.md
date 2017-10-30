```
docker run -p 3306 \
 -e MYSQL_ROOT_PASSWORD=pw  \
 -e CLUSTER_NAME=cluster1  \
 -e XTRABACKUP_PASSWORD=pw  \
 percona/percona-xtradb-cluster 

docker inspect 02a0f4f5d8a18070510b27646aa71c0ce13c1da5e4b778347d14b31ef461c21a | grep IPAddress


mysql -uroot -ppw --port=32768 --host=127.0.0.1
mysql -uroot -ppw --port=32769 --host=127.0.0.1
mysql -uroot -ppw --port=32770 --host=127.0.0.1


 docker run -p 3306 \
 -e MYSQL_ROOT_PASSWORD=pw  \
 -e CLUSTER_NAME=cluster1  \
 -e CLUSTER_JOIN=172.17.0.2  \
 -e XTRABACKUP_PASSWORD=pw  \
 percona/percona-xtradb-cluster 

SET SESSION wsrep_OSU_method=RSU;
create database foo2;
use foo2;
CREATE TABLE test5 (id int(11) NOT NULL AUTO_INCREMENT, PRIMARY KEY (id)) ENGINE=InnoDB;


insert into test1 () values ();
```
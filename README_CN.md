# server and client base on actix_web by IThawk 
* hawk_server  微服务提供方示例
    * server base on axtix_web
* hawk_client  客户端消费示例
    * client base on axtix_web
* hawk_config
    * config center:
        * log               READY
        * server
            * http          READY
            * unix_socket   READY
        * redis
            * ip_port        READY
            * unix_socket    TODO
        * client             TODO
        * chart              TODO
        * proxy              TODO        
* hawk_tools
    * rust tools
        * file_utils
            * read_file
            * write_file
        * os_utils
            * is_windows
        * yml_utils
            * read_yml
            * write_yml
* hawk_chart_center :chart server center

* hawk_proxy : 服务转发代理服务，服务请求到总线                 TODO

* hawk_data : data center, all data connection                 TODO

* hawk_log_center : 
    * 日志统计
    * 日志分析 
    * 提供数据给配置中心，实现配置的动态配置

* hawk_config_center : 
    * 服务总线，更新配置，路由转发到对应的服务地址，提供服务配置查询接口   TODO 
    * 接入服务监控
    * 调用服务转发代理，返回请求数据
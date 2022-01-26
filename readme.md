
# 
高性能的im服务
# 实现概要
1. 使用flatbuffers定义通信数据，不使用http，这样以提高性能
2. 使用udp协议，不使用tcp
3. 使用go、rust实现两个服务端版本
4. 支持window linux mac等常用平台
5. 第二版会增加io uring优化
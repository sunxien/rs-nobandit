# Rust 编写车辆进出车库程序

## 系统库
1. brew install pkg-config
   2. export PKG_CONFIG_PATH=/usr/local/Cellar/pkg-config/0.29.2_3
2. brew install leptonica

## 依赖库
1. SQLite Library https://rustwiki.org/zh-CN/rust-cookbook/database/sqlite.html
2. OCR Library 
3. Camera Library
4. 

## 系统逻辑

### 进场
1. 打开摄像头，拍照，留存照片
2. 使用 OCR 识别照片
   - 2.1 识别车牌：文字和颜色，车辆颜色，例如：京A-76B33（蓝）
   - 2.2 识别车身：颜色，车型，例如：黑色，SUV
3. 是否私有车库
    - 3.1 私有车库，车辆信息已登记，开闸并记录进场时间；否则拒绝；
    - 3.2 公共车库，开闸，记录进场时间；

## 出场
1. 打开摄像头，拍照，留存照片
2. 使用 OCR 识别照片
  - 2.1 识别车牌：文字和颜色，车辆颜色，例如：京A-76B33（蓝）
  - 2.2 识别车身：颜色，车型，例如：黑色，SUV
3. 记录出场时间，计算停车费用，生产支付订单
4. 等待客户支付，后台定时刷新订单状态
    - 4.1 超出15分钟未支付，订单取消，等下次出场重新计算费用；
    - 4.2 每秒刷新一次订单状态，若已支付，要求30分钟内离场；超时未离场，则下次离场重新计算费用；
5. 开闸，放行

# rust core os

## 第一章

```bash
make run LOG=TRACE
```

### 操作系统内核应用

- 普通应用程序去掉标准库（gnu libc），同时自动去除了系统调用;
- 根据CPU架构设置好入口地址，使用汇编调用rust 函数。
- 配置linker，设置入口汇编、栈空间。

### 系統調用

- 內核通過系統調用請求SBI服務
  - 關閉系統
  - 打印字符
- 應用程序通過係統調用請求內核服務
  - 訪問資源

### log 模塊

- rust 外部crate

### panic_handler

去除標準庫後，自己做panic處理。

# norm-adapter-sdk

norm-spec 家族的共享宿主适配器引擎 SDK。引擎侧合并/投影的唯一实现——
协议镜像、单目标与多目标 prompt 上下文、layout 索引——由所有宿主适配器
以精确版本依赖消费。

- 宿主无关：所有机器 API 标识、错误码前缀、prompt 标记均由适配器的
  `AdapterIdentity` 提供，宿主之间不再有其他差异点。
- 确定性：投影保持上游顺序，输出逐字节稳定，超限时失败而非截断。
- 边界清晰：不做 YAML 解析、文件系统遍历、schema 校验、进程生命周期
  或网络 I/O——这些分别属于 norm-spec 与各宿主适配器。

适配器：[pi-norm-spec](https://github.com/CyanoOrg/pi-norm-spec)、
[dsh-norm-spec](https://github.com/CyanoOrg/dsh-norm-spec)。提取决策与
阶段计划记录于 pi-norm-spec D018，本仓以 D001 镜像。

## 状态

`0.1.0` 发布前开发阶段。首次 crates.io 发布评审前保持私有。见
`docs/planning/status.md`。

## 许可

MIT。

# 模块需求 - 设置

## 1. 模块目标

统一管理AI能力接入与系统安全边界，包括模型能力、默认任务权限、可操作目录范围。

## 2. 配置域划分

### 2.1 AI模型能力配置

分三类管理：

- 各AI厂商模型能力（如OpenAI、Anthropic、Google等）
- 本地模型能力（如Ollama）
- API Key方式接入的模型能力

每个模型建议配置字段：

- 提供方
- 模型名称
- 能力标签（对话、工具调用、推理强度、上下文长度）
- 连接参数（endpoint、超时、重试）
- 认证信息（密钥，需加密存储）
- 是否启用
- 健康状态（可连通/不可连通）

### 2.2 默认任务执行权限

- 定义任务默认权限模板
- 新建任务默认继承该模板，可在任务级覆写
- 权限项建议：
  - 文件读写
  - 命令执行
  - 网络访问
  - 外部工具调用

### 2.3 AI可操作目录范围

- 维护目录白名单（允许操作）
- 可选维护目录黑名单（显式拒绝）
- 任务执行时进行路径校验，超出范围直接拒绝

## 3. 功能需求

### FR-SET-01 模型配置管理

- 新增/编辑/删除模型配置
- 测试连通性并返回结果
- 支持设置默认模型与任务专用模型

### FR-SET-02 默认权限配置

- 可配置默认权限模板并版本化
- 模板变更不应直接覆盖历史任务（防止行为突变）

### FR-SET-03 目录范围配置

- 可增删目录规则并即时生效
- 执行前进行路径权限检查并输出可读错误

### FR-SET-04 安全与审计

- 密钥加密存储
- 关键配置变更记录审计日志（谁、何时、改了什么）

## 4. 数据结构（建议）

- `ModelConfig`
  - `configId`
  - `providerType`（vendor/local/api_key）
  - `providerName`
  - `modelName`
  - `endpoint`
  - `apiKeyEncrypted`
  - `capabilities`
  - `isEnabled`
  - `isDefault`
  - `healthStatus`
  - `updatedAt`
- `DefaultPermissionTemplate`
  - `templateId`
  - `templateName`
  - `permissions`
  - `version`
  - `updatedAt`
- `PathScopeRule`
  - `ruleId`
  - `ruleType`（allow/deny）
  - `pathPattern`
  - `enabled`
  - `updatedAt`

## 5. 验收标准

- 三类模型配置可独立维护并可测试连通
- 默认权限可被新任务继承且支持任务级覆写
- 目录范围规则生效，越界操作被阻止并记录
- 关键变更具备审计追踪能力

## 6. 风险与约束

- API Key安全要求高，必须避免明文展示与日志泄露
- 目录规则跨平台差异（Windows/Linux/macOS）需标准化处理


# 模块需求 - 任务管理

## 1. 模块目标

提供任务的集中管理界面，支持创建、编辑、执行、追踪，并与AI对话形成闭环。

新增分层目标：

- 任务分为简单任务和复杂任务
- 复杂任务可由多个任务节点（简单或复杂）进行流程编排
- 编排功能后续版本实现，V0.1先完成数据与状态字段预留

## 2. 页面结构

### 2.1 顶部区域

- 创建任务按钮
- 筛选与搜索（建议V0.1至少保留任务名称搜索）
- 任务类型筛选（简单任务/复杂任务）

### 2.2 任务表格

表格列必须包含：

- 任务名称
- 任务描述
- 任务周期
- 任务权限
- 任务执行次数
- 任务上次执行时间
- 操作

操作列包含：

- 编辑任务信息
- 查看任务执行记录
- 与AI沟通调整任务
- 现在执行

备注：任务类型建议以标签形式展示在“任务名称”单元格中，避免改变V0.1主表列结构。

## 3. 功能需求

### FR-TM-01 创建任务

- 点击“创建任务”后先填写基本信息
- 提交后自动跳转AI对话，并携带基础信息用于任务生成
- AI确认后的结果回写任务列表

### FR-TM-02 编辑任务

- 支持编辑基础字段（名称、描述、周期、权限）
- 编辑时可选择“进入AI辅助调整”，并携带历史对话上下文

### FR-TM-03 任务执行

- 点击“现在执行”可立即触发执行
- 执行后更新：
  - 执行次数 +1
  - 上次执行时间
  - 新增一条执行记录
  - 预留并更新任务级字段：`executionStatus`、`executionResult`

### FR-TM-04 任务记录跳转

- 点击“查看任务执行记录”进入记录模块，并自动定位当前任务

### FR-TM-05 简单任务与复杂任务

- 每个任务必须具备任务复杂度类型：
  - `simple`：独立执行任务
  - `complex`：编排型任务
- `complex`任务可包含多个任务节点，节点可引用`simple`或`complex`任务
- 编排关系至少预留以下结构能力：
  - 顺序执行（A -> B）
  - 条件分支（按上一步执行状态/结果判断）
  - 并行分支（后续版本实现）
- V0.1实现边界：
  - 先支持复杂任务的数据结构建模与配置入口预留
  - 暂不实现复杂任务实际编排执行引擎

## 4. 数据结构（建议）

- `Task`
  - `taskId`
  - `taskName`
  - `taskDescription`
  - `taskType`（one_time/recurring）
  - `taskComplexityType`（simple/complex）
  - `scheduleRule`
  - `permissionPolicy`
  - `executionCount`
  - `lastExecutedAt`
  - `executionStatus`（idle/running/success/failed/canceled）
  - `executionResult`（success/failed/partial/unknown）
  - `flowId`（复杂任务关联，可空）
  - `status`（enabled/disabled/archived）
  - `createdBy` `createdAt` `updatedAt`
- `TaskPermissionPolicy`
  - `policyId`
  - `scopeType`（inherit_default/custom）
  - `allowedActions`
  - `allowedPaths`
- `ComplexTaskFlow`
  - `flowId`
  - `parentTaskId`
  - `nodes`
  - `edges`
  - `version`
  - `updatedAt`
- `ComplexTaskNode`
  - `nodeId`
  - `flowId`
  - `refTaskId`
  - `nodeType`（simple_task/complex_task）
  - `onSuccessNext`
  - `onFailureNext`
  - `conditionExpr`（可空）

## 5. 交互与状态

- 列表需要支持加载态、空态、错误态
- “现在执行”需有执行中状态防重复点击
- 复杂任务在V0.1可预留“编排配置”入口，并标记“后续开放”
- 删除需求暂不纳入V0.1（避免误删与审计缺失）

## 6. 验收标准

- 表格字段完整显示且可分页
- 四个操作按钮均可用并完成对应跳转/动作
- 新建流程实现“表单 -> AI对话 -> 回写列表”的闭环
- 编辑流程可携带AI上下文并成功保存
- 任务实体在执行后可更新`executionStatus`和`executionResult`
- 可创建`simple`与`complex`两类任务（复杂任务先完成结构化保存）

## 7. 风险与约束

- 周期表达式复杂度高，V0.1建议先限制常见模板
- 权限策略需与设置模块保持一致，避免冲突
- 复杂任务可能出现循环依赖与死锁，后续需加入流程校验

# Tauri 本地列表工具开发规格

## 1. 项目目标

开发一个基于 **Tauri 2 + Svelte + TypeScript + Rust + Tokio + Axum + SQLite** 的桌面应用。

第一版只实现 **本地运行模式**。

联网模式暂时只做架构预留，不实现实际网络连接、WebSocket、UPnP、端口映射等功能。

应用主要用于维护多个独立列表。

每个列表内部保存若干 **8 位字母数字码**。

每个码：

- 固定长度为 8 位。
- 仅允许英文字母大小写 `A-Z`、`a-z` 和数字 `0-9`。
- 不允许包含任何符号、空格或其他字符。
- 同一个列表内，在未过期期间不允许重复。
- 不同列表之间允许存在相同 code。
- code 必须始终按字符串处理，不得声明或转换为数值类型。
- 新增后立即开始倒计时。
- 默认有效期为 **1 小时 30 分钟**。
- 到期后自动从活跃列表中删除。
- 用户可以手动执行操作删除该码。
- 不需要提供编辑码内容的功能。

---

# 2. 技术栈

## 2.1 桌面框架

使用：

```text
Tauri 2
```

负责：

- 桌面应用生命周期。
- 前端 WebView。
- 前端与 Rust 后端通信。
- 本地窗口管理。
- 本地设置持久化。

---

## 2.2 前端

使用：

```text
Svelte
TypeScript
```

前端主要负责：

- 页面布局。
- 列表切换。
- 数据展示。
- 倒计时实时显示。
- 粘贴事件监听。
- 用户交互。
- 调用 Tauri command。
- 接收 Rust 后端事件。

不要把数据库逻辑放在前端。

---

## 2.3 Rust 后端

使用：

```text
Rust
Tokio
```

负责：

- SQLite 数据访问。
- 数据增删。
- 数据去重。
- 过期处理。
- 定时任务。
- 应用初始化。
- 设置保存。
- 为未来联网模式预留接口。

---

## 2.4 HTTP / WebSocket

未来使用：

```text
Axum
Tokio
WebSocket
```

但：

> 第一版不要实现 Axum Server、WebSocket、UPnP、NAT、端口映射、互联网连接。

只需要在代码结构中预留 networking 模块。

---

## 2.5 数据库

使用：

```text
SQLite
rusqlite
```

第一版所有列表和数据均保存在本地 SQLite 数据库。

---

# 3. 第一版范围

第一版必须实现：

- 首次运行模式选择页面。
- 默认自动选择“本地运行”。
- 本地运行模式。
- 多列表管理。
- 列表切换。
- 每个列表独立维护自己的数据。
- 8 位字母数字码录入。
- 粘贴自动识别。
- 自动去重。
- 自动入库。
- 重复码自动清空输入区并提示“已存在”。
- 90 分钟倒计时。
- 自动过期删除。
- 手动删除。
- 当前第一个码详情展示。
- 设置页面入口。
- SQLite 持久化。
- 应用重启后恢复未过期数据。
- 已经过期的数据启动时自动清理。

第一版不实现：

- 联网同步。
- WebSocket。
- 用户系统。
- 登录。
- 公网服务器。
- UPnP。
- NAT 穿透。
- IPv4 / IPv6 检测。
- DDNS。
- 云端数据库。
- 多设备数据同步。
- 数据编辑。
- 每日固定清空。

---

# 4. 首次运行流程

第一次启动应用时显示运行模式选择页面。

界面应包含两个模式：

```text
本地运行
联网运行
```

其中：

## 本地运行

第一版可用。

说明：

```text
数据仅保存在当前设备。
不需要网络连接。
```

## 联网运行

第一版暂不可用。

可以显示：

```text
联网运行
即将支持
```

或者 disabled 状态。

---

## 4.1 默认行为

第一次运行时：

```text
默认选中：本地运行
```

允许用户点击：

```text
开始使用
```

进入主界面。

第一版可以直接把：

```text
run_mode = local
```

保存到本地设置。

之后再次启动：

```text
直接进入主界面
```

不再显示首次运行页面。

设置中应预留：

```text
运行模式
```

但第一版只允许：

```text
本地运行
```

---

# 5. 整体界面设计

整体视觉和布局参考典型 Agent / IDE 类应用。

采用：

```text
左侧全局功能栏
+
左侧当前功能的数据列表
+
右侧主功能区域
```

整体结构：

```text
┌──────┬──────────────────┬────────────────────────────┐
│      │                  │                            │
│ 全局 │   当前列表数据    │        数据详情区域         │
│ 功能 │                  │                            │
│      │                  │                            │
│      │                  ├────────────────────────────┤
│      │                  │                            │
│      │                  │        输入区域             │
│      │                  │                            │
│      │                  │                            │
│  ⚙   │                  │                            │
└──────┴──────────────────┴────────────────────────────┘
```

---

# 6. 最左侧：全局功能栏

最左侧使用窄侧栏。

主要功能：

```text
列表 1
列表 2
列表 3
...
```

用户可以点击不同列表进行切换。

底部固定一个：

```text
设置图标
```

例如：

```text
⚙
```

---

## 6.1 列表管理

用户需要能够：

- 创建新列表。
- 切换列表。
- 删除列表。
- 重命名列表。

建议默认创建：

```text
列表 1
```

如果数据库中没有任何列表，则首次进入主界面时自动创建一个默认列表。

---

## 6.2 当前列表状态

当前选中的列表应该有明显选中状态。

例如：

```text
● 列表 1
  列表 2
  列表 3
```

不要使用过多装饰。

界面整体应保持：

- 简洁。
- 高信息密度。
- 接近 Agent / 开发工具。
- 不要做成移动 App 风格。
- 不使用大面积卡片堆叠。

---

# 7. 中间左侧：当前列表活跃数据

当前列表的数据区域显示：

```text
当前列表中所有未过期的 8 位 code
```

例如：

```text
A1b2C3d4
4829ABcd
ZZ91xY7Q
10aB39Cd
```

---

## 7.1 排序规则

默认按照：

```text
最早加入的数据排在最上面
```

即：

```text
created_at ASC
```

因此第一个数据就是当前优先展示的详细数据。

---

## 7.2 数据展示

每行至少显示：

```text
A1b2C3d4
```

可以附带一个简短的剩余时间：

```text
A1b2C3d4      01:18:32
```

建议显示。

code 建议使用等宽字体。

---

## 7.3 当前选中数据

默认：

```text
自动选中第一条数据
```

用户点击其他码时：

```text
右上详情区域切换到该码
```

但是：

如果用户没有手动选择其他码，则列表变化后继续默认显示当前第一条数据。

---

## 7.4 数据为空

如果当前列表为空：

```text
暂无数据
```

右侧详情区域显示空状态。

---

# 8. 右上：数据详细区域

右侧上半部分显示当前选中码的详细情况。

至少包含：

```text
码
过期时间倒计时
操作按钮
```

布局示例：

```text
A1b2C3d4

剩余时间
01:23:41

[ 删除 ]
```

---

## 8.1 码显示

需要突出显示当前码：

```text
A1b2C3d4
```

字号应明显大于普通列表文字。

code 必须以字符串显示并保持原始大小写，不进行大小写归一化。

---

## 8.2 倒计时

每个码固定有效：

```text
90 分钟
```

数据库保存绝对时间：

```text
expires_at
```

前端根据：

```text
expires_at - 当前时间
```

计算倒计时。

显示格式：

```text
HH:MM:SS
```

例如：

```text
01:29:59
00:42:13
00:03:07
```

---

## 8.3 倒计时原则

不要在数据库中每秒更新剩余时间。

数据库只保存：

```text
created_at
expires_at
```

前端自己实时计算。

例如：

```ts
remaining = expiresAt - Date.now()
```

每秒刷新 UI。

---

## 8.4 操作按钮

第一版只需要：

```text
删除
```

点击后：

```text
从数据库删除
↓
从当前列表删除
↓
UI 更新
```

可以增加删除确认，但由于数据本身生命周期只有 90 分钟，第一版建议：

```text
直接删除
```

不要增加不必要的确认步骤。

---

# 9. 右下：输入区域

右侧下半部分是主要输入区域。

提供一个输入框。

用途：

```text
用户复制一个或多个 code
↓
粘贴进入输入框
↓
系统自动识别
↓
校验
↓
同列表内去重
↓
直接入库
```

---

# 10. code 格式

一个合法 code 必须满足：

```regex
^[A-Za-z0-9]{8}$
```

即：

- 长度必须恰好 8 位。
- 允许 `A-Z`。
- 允许 `a-z`。
- 允许 `0-9`。
- 不允许任何符号。
- 不允许空格。
- 不允许中文或其他字符。
- 大小写敏感。

例如：

```text
A1b2C3d4
abcdefgh
ABCDEFGH
12345678
aB12Cd34
```

均合法。

以下非法：

```text
ABC-1234
ABC_1234
A1 B2C3D
ABCDEFG
ABCDEFGHI
```

---

# 11. 粘贴行为

这是核心交互。

监听：

```text
paste
```

事件。

当检测到用户粘贴文本以后：

```text
读取剪贴板内容
↓
解析所有可能的 8 位字母数字码
↓
过滤非法内容
↓
同一次粘贴内容内部去重
↓
检查当前列表数据库中现有活跃数据
↓
新增不存在的 code
↓
开始 90 分钟计时
↓
自动清空输入区域
```

---

## 11.1 支持单码粘贴

例如用户粘贴：

```text
A1b2C3d4
```

合法且当前列表不存在时立即添加。

添加完成后：

```text
自动清空输入区域
```

---

## 11.2 支持多码粘贴

第一版直接支持多码。

例如：

```text
A1b2C3d4
B2c3D4e5
C3d4E5f6
```

或者：

```text
A1b2C3d4 B2c3D4e5 C3d4E5f6
```

或者：

```text
A1b2C3d4,B2c3D4e5,C3d4E5f6
```

都应该能够识别。

推荐解析方式：

1. 对粘贴文本按常见分隔符拆分。
2. 对每个候选值执行完整校验：

```regex
^[A-Za-z0-9]{8}$
```

不要从更长的连续字母数字字符串中截取 8 位子串作为合法 code。

---

# 12. 自动去重

去重范围为：

```text
当前列表
```

不同列表之间允许存在相同 code。

因此：

```text
列表 A:
A1b2C3d4

列表 B:
A1b2C3d4
```

是合法状态。

但是：

```text
列表 A:
A1b2C3d4
A1b2C3d4
```

不允许。

数据库约束：

```text
PRIMARY KEY (list_id, code)
```

或者：

```text
UNIQUE(list_id, code)
```

均可。

第一版推荐直接：

```text
PRIMARY KEY (list_id, code)
```

---

# 13. 重复码处理

假设当前列表已经存在：

```text
A1b2C3d4
剩余 32 分钟
```

用户再次粘贴：

```text
A1b2C3d4
```

处理规则：

```text
不新增
不重置倒计时
不修改 expires_at
自动清空输入区域
```

UI 中短暂提示：

```text
已存在
```

要求：

- 不弹模态窗口。
- 提示自动消失。
- 建议显示 1~2 秒。
- 不阻塞继续输入。
- 不改变当前已有数据的过期时间。

如果一次粘贴包含多个 code，其中部分重复、部分新数据：

```text
新数据正常入库
重复数据忽略
输入区域仍然在处理结束后自动清空
```

可以提示：

```text
已添加 2 个，1 个已存在
```

---

# 14. 新增 code

新 code 成功加入时：

```text
created_at = 当前时间
expires_at = 当前时间 + 90 分钟
```

例如：

```text
created_at:
2026-09-25 13:00:00

expires_at:
2026-09-25 14:30:00
```

code 必须原样保存。

例如：

```text
AbCd1234
```

不得自动改为：

```text
ABCD1234
```

或：

```text
abcd1234
```

---

# 15. 过期逻辑

code 到期后自动删除。

Rust 后端负责真实数据库清理。

第一版建议：

```text
每 1 秒或每 5 秒检查一次
```

由于数据量极小，无需过度优化。

SQL：

```sql
DELETE FROM items
WHERE expires_at <= ?;
```

更好的实现方式：

```text
查询已过期数据
↓
删除
↓
向前端发送事件
↓
前端同步移除
```

---

# 16. 应用重启

应用退出后重新打开：

```text
读取数据库
↓
删除已经过期的数据
↓
加载仍然有效的数据
↓
根据原 expires_at 继续倒计时
```

绝对不能因为程序重启而重新开始 90 分钟计时。

例如：

```text
13:00 添加
14:00 关闭程序
14:10 再打开
```

应该显示：

```text
剩余 20 分钟
```

而不是重新显示：

```text
90 分钟
```

---

# 17. 数据库设计

建议 SQLite schema：

```sql
CREATE TABLE lists (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    created_at INTEGER NOT NULL
);
```

items：

```sql
CREATE TABLE items (
    list_id INTEGER NOT NULL,
    code TEXT NOT NULL,
    created_at INTEGER NOT NULL,
    expires_at INTEGER NOT NULL,

    PRIMARY KEY (list_id, code),

    FOREIGN KEY (list_id)
        REFERENCES lists(id)
        ON DELETE CASCADE
);
```

注意：

```text
code 必须使用 TEXT
```

不得因为某些 code 恰好全部由数字组成而使用：

```text
INTEGER
BIGINT
NUMBER
```

因为合法 code 允许英文字母，并且即使是：

```text
00001234
```

也必须保留前导零。

推荐时间全部保存：

```text
Unix timestamp milliseconds
```

方便前端直接：

```ts
new Date(timestamp)
```

---

# 18. 设置表

可以建立：

```sql
CREATE TABLE settings (
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL
);
```

第一版至少保存：

```text
first_run_completed
run_mode
last_selected_list_id
```

例如：

```text
first_run_completed = true
run_mode = local
last_selected_list_id = 3
```

---

# 19. 前后端通信

使用 Tauri commands。

例如：

```rust
#[tauri::command]
fn get_lists()

#[tauri::command]
fn create_list(name: String)

#[tauri::command]
fn rename_list(id: i64, name: String)

#[tauri::command]
fn delete_list(id: i64)

#[tauri::command]
fn get_items(list_id: i64)

#[tauri::command]
fn add_codes(list_id: i64, codes: Vec<String>)

#[tauri::command]
fn delete_code(list_id: i64, code: String)
```

特别注意：

```rust
code: String
codes: Vec<String>
```

不得使用数值类型。

---

# 20. 后端事件

Rust 后端可以使用 Tauri event 通知前端。

例如：

```text
item-added
item-deleted
item-expired
list-updated
```

过期任务：

```text
Rust
↓
删除 SQLite
↓
emit("item-expired")
↓
Svelte 更新界面
```

这样不需要前端轮询数据库。

---

# 21. 前端状态管理

第一版不需要引入复杂状态管理框架。

可以直接使用 Svelte store。

例如：

```text
currentList
lists
items
selectedCode
```

其中：

```ts
selectedCode: string | null
```

---

# 22. 输入框交互

输入框建议行为：

### 粘贴

```text
检测 paste
↓
解析合法 code
↓
立即提交
↓
处理新增/重复
↓
清空输入框
```

不需要用户再点击：

```text
添加
```

---

### 重复粘贴

如果粘贴的是当前列表中已经存在的 code：

```text
不新增
不重新计时
清空输入框
显示短暂提示“已存在”
```

---

### 手工输入

虽然主要用途是粘贴，但可以允许手工输入。

当输入达到 8 位，并且满足：

```regex
^[A-Za-z0-9]{8}$
```

时，可以：

```text
按 Enter
```

提交。

第一版不建议输入满 8 位就自动提交，因为用户手工输入过程中误触更容易出现问题。

提交处理完成后无论：

```text
新增成功
或
已经存在
```

都清空输入框。

非法输入时可保留输入内容，方便用户自行修改。

---

# 23. 粘贴反馈

例如用户一次粘贴：

```text
A1b2C3d4
B2c3D4e5
A1b2C3d4
ABC-1234
```

当前列表已经有：

```text
B2c3D4e5
```

最终：

```text
新增：1
重复：2
非法：1
```

UI 可以在输入框附近短暂显示：

```text
已添加 1 个，2 个已存在
```

输入区域在处理结束后自动清空。

不需要详细列出错误。

---

# 24. 当前第一条数据逻辑

右上区域默认显示当前列表中的第一条活跃数据。

默认规则：

```text
items 按 created_at ASC
↓
items[0]
↓
右上详情
```

如果用户主动点击其他 code：

```text
selectedCode = 用户选择
```

如果该 selectedCode 被删除或过期：

```text
自动回到当前第一条
```

---

# 25. 布局比例

推荐大致比例：

```text
全局侧栏：
56~72px

数据列表区：
220~300px

右侧主区域：
剩余全部宽度
```

右侧：

```text
上部详情：
约 55%

下部输入：
约 45%
```

不要求严格固定比例，应支持窗口缩放。

---

# 26. 视觉风格

目标：

```text
桌面 Agent
IDE
管理工具
```

避免：

```text
手机 App 风
大量圆角卡片
大面积彩色背景
营销网页风格
```

建议：

- 暗色/浅色可以先跟随系统。
- 边框低对比度。
- 简洁字体层级。
- code 使用等宽字体。
- 倒计时使用等宽数字。
- 当前选中列表和 code 有明显但克制的高亮。
- 操作按钮集中在详情区。

---

# 27. 设置页面

左下角设置图标：

```text
⚙
```

点击以后切换到设置页面。

第一版至少展示：

```text
运行模式
本地运行

默认有效期
90 分钟
```

默认有效期第一版可以只读，不允许修改。

另外展示：

```text
数据库路径
```

方便调试。

可以提供：

```text
打开数据目录
```

---

# 28. 联网功能预留

虽然第一版不实现网络，但是代码结构必须避免未来重构业务层。

建议定义：

```rust
enum RunMode {
    Local,
    NetworkHost,
    NetworkClient,
}
```

第一版实际只有：

```rust
RunMode::Local
```

可用。

---

# 29. Repository / Service 分层

不要让 UI 或网络层直接操作 SQLite。

建议：

```text
UI
↓
Service
↓
Repository
↓
SQLite
```

例如：

```text
ItemService
├─ add_codes()
├─ delete_code()
├─ expire_items()
└─ get_active_items()

ItemRepository
├─ insert()
├─ delete()
├─ find_active()
└─ delete_expired()
```

未来联网时：

```text
Local UI
        ↓
    ItemService
        ↓
    Repository
```

和：

```text
WebSocket
        ↓
    ItemService
        ↓
    Repository
```

可以复用完全相同的业务逻辑。

---

# 30. 推荐 Rust 项目结构

```text
src-tauri/
├─ src/
│  ├─ main.rs
│  ├─ lib.rs
│  │
│  ├─ commands/
│  │  ├─ mod.rs
│  │  ├─ lists.rs
│  │  ├─ items.rs
│  │  └─ settings.rs
│  │
│  ├─ db/
│  │  ├─ mod.rs
│  │  ├─ migrations.rs
│  │  ├─ lists.rs
│  │  └─ items.rs
│  │
│  ├─ services/
│  │  ├─ mod.rs
│  │  ├─ list_service.rs
│  │  └─ item_service.rs
│  │
│  ├─ scheduler/
│  │  ├─ mod.rs
│  │  └─ expiry.rs
│  │
│  ├─ networking/
│  │  └─ mod.rs
│  │
│  ├─ models/
│  │  ├─ mod.rs
│  │  ├─ list.rs
│  │  └─ item.rs
│  │
│  └─ state.rs
│
└─ Cargo.toml
```

---

# 31. 推荐前端结构

```text
src/
├─ App.svelte
│
├─ routes/
│  ├─ FirstRun.svelte
│  ├─ Main.svelte
│  └─ Settings.svelte
│
├─ components/
│  ├─ GlobalSidebar.svelte
│  ├─ ListSidebar.svelte
│  ├─ ItemDetail.svelte
│  ├─ CodeInput.svelte
│  └─ Countdown.svelte
│
├─ stores/
│  ├─ lists.ts
│  ├─ items.ts
│  └─ settings.ts
│
├─ services/
│  └─ tauri.ts
│
└─ types/
   └─ index.ts
```

---

# 32. 数据模型

TypeScript：

```ts
export interface ListInfo {
  id: number;
  name: string;
  createdAt: number;
}
```

```ts
export interface Item {
  listId: number;
  code: string;
  createdAt: number;
  expiresAt: number;
}
```

注意：

```text
code 必须为 string
```

不能使用：

```ts
number
bigint
```

原因：

- code 允许英文字母。
- code 大小写敏感。
- 纯数字形式的合法 code 也可能包含前导零。

Rust：

```rust
pub struct Item {
    pub list_id: i64,
    pub code: String,
    pub created_at: i64,
    pub expires_at: i64,
}
```

---

# 33. 过期处理

后端建议运行一个 Tokio background task：

```text
应用启动
↓
spawn expiry_worker
```

逻辑：

```text
loop
    当前时间
    ↓
    查找 expires_at <= now
    ↓
    删除
    ↓
    通知前端
    ↓
    sleep 1 second
```

由于预计数据量很小：

```text
每秒查询一次 SQLite
```

完全可以接受。

也可以优化为：

```text
查询最近 expires_at
↓
sleep_until()
```

但第一版不需要。

优先保证代码简单可靠。

---

# 34. 程序关闭行为

正常退出：

```text
不修改任何 expires_at
```

数据库保留原状态。

再次打开：

```text
清理已经过期的数据
↓
恢复剩余数据
```

---

# 35. 数据安全

第一版只运行本地模式。

因此：

- 不需要加密数据库。
- 不需要用户认证。
- 不需要密码。
- 不需要 Token。
- 不需要网络权限管理。

但数据库写入必须使用事务保证一致性。

---

# 36. 错误处理

原则：

不要因为单条非法输入导致整个程序报错。

例如：

```text
输入：
A1b2C3d4
ABC-1234
Z9y8X7w6
```

应该：

```text
添加：
A1b2C3d4
Z9y8X7w6

忽略：
ABC-1234
```

---

# 37. 日志

Rust 侧使用：

```text
tracing
```

至少记录：

```text
应用启动
数据库初始化
列表创建/删除
code 新增
code 删除
code 过期
数据库错误
后台任务错误
```

不要记录大量每秒倒计时日志。

---

# 38. 第一版用户流程

## 首次启动

```text
启动
↓
运行模式选择
↓
默认本地运行
↓
开始使用
↓
主界面
```

---

## 正常使用

```text
选择列表
↓
复制一个或多个 8 位 code
↓
粘贴到输入区域
↓
自动解析
↓
当前列表内自动去重
↓
自动入库
↓
输入区域自动清空
↓
立即开始 90 分钟倒计时
↓
左侧出现数据
↓
右上显示第一条数据详情
```

---

## 重复输入

```text
粘贴当前列表已有 code
↓
不重复入库
↓
不重置倒计时
↓
输入区域自动清空
↓
短暂提示“已存在”
```

---

## 数据过期

```text
倒计时归零
↓
Rust 删除 SQLite 数据
↓
通知前端
↓
左侧自动消失
↓
右上切换下一条
```

---

## 手动删除

```text
选择 code
↓
点击删除
↓
立即删除
↓
自动显示下一条
```

---

# 39. 第一版验收标准

以下全部满足才算第一版完成。

### 启动

- [ ] 第一次启动显示运行模式选择。
- [ ] 默认选择本地运行。
- [ ] 联网模式显示但不可用。
- [ ] 第二次启动直接进入应用。

### 列表

- [ ] 可以创建多个列表。
- [ ] 可以切换列表。
- [ ] 可以重命名列表。
- [ ] 可以删除列表。
- [ ] 每个列表数据独立。
- [ ] 不同列表允许存在相同 code。

### code

- [ ] code 固定为 8 位。
- [ ] 仅允许英文字母大小写和数字。
- [ ] 不允许符号。
- [ ] code 大小写敏感。
- [ ] code 全程使用字符串类型。
- [ ] 支持粘贴一个 code。
- [ ] 支持一次粘贴多个 code。
- [ ] 自动解析。
- [ ] 当前列表内自动去重。
- [ ] 重复 code 不会重新计时。
- [ ] 重复 code 输入后自动清空输入框。
- [ ] 重复 code 显示短暂“已存在”提示。
- [ ] 新 code 自动设置 90 分钟过期时间。
- [ ] 数据写入 SQLite。

### 展示

- [ ] 左侧展示当前列表所有活跃 code。
- [ ] 默认按创建时间升序排列。
- [ ] 默认选中第一条。
- [ ] 可以点击其他 code 查看详情。
- [ ] 详情显示 code。
- [ ] 详情显示实时倒计时。
- [ ] 详情提供删除按钮。

### 过期

- [ ] 倒计时基于绝对 expires_at。
- [ ] 到期自动删除。
- [ ] UI 自动更新。
- [ ] 应用重启不会重置倒计时。
- [ ] 启动时自动清理已经过期的数据。

### 设置

- [ ] 左下角存在设置按钮。
- [ ] 可以进入设置页面。
- [ ] 显示当前运行模式。
- [ ] 显示默认有效期。
- [ ] 显示数据库路径。

### 第一版明确不包含

- [ ] 不实现每日固定清空。
- [ ] 不实现联网功能。
- [ ] 不实现 WebSocket。
- [ ] 不实现 UPnP / NAT / 公网地址检测。

---

# 40. 第二阶段联网功能预留

第二阶段预计实现：

```text
主机模式
客户端模式
```

主机：

```text
Tauri
↓
Axum WebSocket Server
↓
SQLite
```

客户端：

```text
Tauri
↓
WebSocket
↓
主机
```

主机未来负责：

- WebSocket Server。
- 多客户端连接。
- 增删广播。
- SQLite authoritative state。
- 过期广播。
- UPnP 临时端口映射。
- 公网 IPv4 获取。

但：

> 第一版绝对不要为了未来联网功能增加当前实现复杂度。

只要求业务逻辑与数据库层足够独立，以便以后复用。

---

# 41. 实现原则

开发过程中优先级：

```text
正确性
>
稳定性
>
代码清晰
>
可扩展
>
性能优化
```

不要过早优化。

预计列表数据量和用户量都很小，因此：

- 不需要 Redis。
- 不需要 PostgreSQL。
- 不需要复杂缓存。
- 不需要 ORM。
- 不需要微服务。
- 不需要消息队列。
- 不需要复杂状态管理库。

保持单体 Tauri 应用即可。

---

# 42. 最终目标结构

```text
┌──────────────────────────────────────────┐
│                 Tauri                    │
│                                          │
│  ┌──────────── Svelte UI ─────────────┐ │
│  │                                    │ │
│  │ 全局侧栏                           │ │
│  │ 列表数据                           │ │
│  │ 数据详情                           │ │
│  │ 输入区域                           │ │
│  │ 设置                               │ │
│  │                                    │ │
│  └────────────────────────────────────┘ │
│                    │                     │
│              Tauri Commands              │
│                    │                     │
│  ┌──────────── Rust Backend ──────────┐ │
│  │                                    │ │
│  │ ItemService                        │ │
│  │ ListService                        │ │
│  │ SQLite                             │ │
│  │ Tokio Scheduler                    │ │
│  │ Expiry Worker                      │ │
│  │                                    │ │
│  │ networking/                        │ │
│  │   └─ 暂不实现                      │ │
│  │                                    │ │
│  └────────────────────────────────────┘ │
│                                          │
└──────────────────────────────────────────┘
```

第一版应首先完成一个：

> **稳定、简单、纯本地、支持多列表、支持粘贴自动入库、同列表去重、90 分钟自动过期的桌面工具。**

联网能力放到后续版本实现。

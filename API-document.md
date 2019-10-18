# API文档
文档描述该应用程序包含的所有对外暴露的API，作为使用说明书。

API的定义是与配置文件相关的。在这里，所有`web.api.prefix`下属的URL，前缀均用`/api`标示；
所有`static.cover.prefix`下属的URL，前缀均用`/static/cover`标示。
## 公共事项
### 用户认证
系统只提供Bearer Token认证。在发送请求时，编写请求头`Authorization`: `Bearer {token}`。token使用API`/api/token/`获得。  
用户认证成功时，会继续后续的业务请求。  
用户认证失败时，有以下几种情况：
1. 401 Unauthorized
    * 没有获得Authorization请求头，请求头无法被正确解析，或请求头不是合法的Bearer格式
    * token不存在，已经过期，或已被销毁
2. 403 Forbidden - 不止需要登录，还需要更高的权限，比如管理员权限
### Request反序列化
除个别API之外，几乎所有的API的Request Body都是`application/json`格式。  
如果body内容不符合基本的json规范，那么首先会返回`400 Bad Request`状态码。  
如果API描述中对Request Body的内容要求没有被满足，那么也会返回`400 Bad Request`状态码。
### 内部服务器错误

## 普通用户API

### /api/token/ POST
用户进行token认证，拿到一个新的token。  
token认证需要用户名和密码。除此之外，还可以设置token的有效时长。  
* 如果不设置任何与时长有关的参数，那么持续时长将是系统的默认时长；  
* 如果将`effective_unlimit`设置为`true`，那么将使用系统规定的能使用的最长时长，如果最长时长是无限，那么可以设置无限长的token；
* 如果设置`effective`参数，那么将请求这个长度的持续时长，但如果这个时长超过了最大时长，还是会被设定为最大时长。  

token将在持续时长结束后过期，并被销毁。如果想延长一个已经持有的token的持续时间，请使用`/api/token/{token}/ PUT`API。

- **Request Body**
    - `username`: string - 用户名
    - `password`: string - 密码
    - `effective`: (optional) long|null - token有效时长，单位毫秒
    - `effective_unlimit`: (optional) bool - 请求最大长度的token
    
- **Response Body** [201 Created]
    - `key`: string - token
    - `user_id`: int - 该用户的id
    - `expire_time`: datetime|null - token过期的时间，null表示不会被动过期的token
    - `create_time`: datetime - token创建的时间
    - `update_time`: datetime - 上次更新token的时间
    
- **Response Error**
    1. 401 Unauthorized
        * `Password wrong` - 密码错误
        * `User not exist` - 该用户不存在
        * `User not enabled` - 该用户已经被停用

### /api/token/{token}/ GET
查询用户持有的token的状态。
- **Path** 
    - `token`: 要查阅的token

- **Response Body** [200 Ok]
    与`/api/token/ POST`的body相同。

- **Response Error**
    1. 404 Not Found - 未找到token，或该token已经失效。

### /api/token/{token}/ PUT
对用户持有的token的持续时间进行更新。
- **Path** 
    - `token`: 要查阅的token
    
- **Request Body**
    - `effective`: long - token有效时长，具体规则参考`/api/token/ POST`的说明
    
- **Response Body** [200 Ok]
    与`/api/token/ POST`的body相同。

- **Response Error**
    1. 404 Not Found - 未找到token，或该token已经失效。

### /api/register/ POST
注册一个新用户。  
注册用户需要几项基本信息。此外，系统控制着注册途径的开放。有三类状态：
* 开放注册：允许自由注册。
* 仅允许注册码注册：注册必须消耗正确的注册码，否则不允许注册。
* 关闭注册：任何人都不允许注册，相当于关闭此API。

- **Request Body**
    - `username`: string - 用户名，不允许重复用户名
    - `password`: string - 密码
    - `name`: string - 用户显示的名称
    - `code`: (optional)string - 注册码
    
- **Response Body** [201 Created]
    (empty)
    
- **Response Error**
    1. 403 Forbidden - `Register closed` - 注册被关闭
    2. 400 Bad Request
        - `Disabled registration code`: 使用了无法使用的注册码，可能是注册码错误、过期、已被使用
        - `Need registration code`: 系统处于仅注册码模式，必须提供注册码
        - `Username exist`: 该用户名已经存在
        - `field {field} cannot be empty`: 必填字段有留空
        

### /api/user/ GET
用户查看自己的用户信息。

- **Verify**: login

- **Response Body** [200 Ok]
    - `id`: int - user id
    - `username`: string - 用户名
    - `name`: string - 用户显示名
    - `cover`: string|null - 用户头像的文件名
    - `is_staff`: bool - 是否是系统管理员
    - `last_login`: datetime|null - 上次登录时间，如果为null表示用户还没有登录过
    - `last_login_ip`: string|null - 上次登录的IP，如果为null表示可能还没有登录过，或上次登录的IP无法被识别
    - `create_time`: datetime - 用户创建时间
    - `create_path`: string - 用户创建途径：
        1. `System` - 该用户由系统创建，只有初始化系统时创建的初始用户会通过此途径创建
        2. `Admin` - 该用户由系统管理员在管理后台创建
        3. `Code` - 该用户使用注册码注册
        4. `Public` - 该用户通过开放注册注册

### /api/user/ POST|PUT
修改用户的部分信息。

- **Verify**: login

- **Request Body**
    - `name`: string - 用户显示名
    
- **Response Body** [200 Ok]
    与`GET /api/user/`结果相同。

### /api/user/password/ POST
用户修改密码。  
修改尽管需要认证，但同时仍然需要旧密码。

- **Verify**: login

- **Request Body**
    - `old_password`: string - 旧密码
    - `new_password`: string - 新密码
    
- **Response Body** [200 Ok]
    `success`

- **Response Error**
    1. 401 Unauthorized - `Password wrong` - 旧密码错误

### /api/user/cover/ POST
上传用户头像。  
上传的图像应当是主流图像格式，例如jpg、png。上传完成后，返回头像文件名，并且用户的`cover`字段会更改为此名。  
头像图像会被裁剪为正方形，并被缩放到一个较小的尺寸，以减小文件大小。  
浏览器要访问该图像，参考`/static/cover/{cover} GET`API。

- **Verify**: login

- **Request Body**
    要上传的文件的二进制内容。
    
- **Response Body** [200 Ok]
    - `cover`: 图像文件名

- **Response Error**
    1. 400 Bad Request - 文件转换出现问题，文件类型或内容不正确

### /static/cover/{cover} GET
(静态HTTP请求)请求一份头像图像文件。

- **Path**
    - `cover`: 头像文件名。

- **Response Body** [200 Ok]
    图像文件的二进制内容。

## 管理API

### /api/admin/setting/ GET
获得现在的系统设置内容。内容包括：
1. 注册模式：切换注册的限制。
    - `Open`: 开放注册
    - `Code`: 只允许注册码注册
    - `Close`: 关闭注册
2. token持续时间限制。
    - 最长token时间：系统中最长能申请多长持续时间的token。如果设置为null，则允许无限持续时间的token。
    - 默认token持续时间：在用户不主动设定持续时间时，为token设定这么长的持续时间。不能为null，系统不允许默认给予无限时长的token。

- **Verify**: admin
  
- **Response Body** [200 Ok]
    - `register_mode`: string - 注册模式开关
    - `effective_max`: long|null - 最长token时间
    - `effective_default`: long - 默认token时间

### /api/admin/setting/ POST|PUT
更改系统设置。  
这个API是PUT UPDATE，将一次更新全部的设置项目。  
有关设置项目的内容，参考`/api/admin/setting/ GET`API。

- **Verify**: admin

- **Request Body**
    - `register_mode`: string - 注册模式开关
    - `effective_max`: long|null - 最长token时间
    - `effective_default`: long - 默认token时间

- **Response Body** [200 Ok]
    参考`/api/admin/setting/ GET`API。 

### /api/admin/registration-code/ GET
获得全部注册码的列表。

- **Verify**: admin

- **Response Body** [200 Ok]
    - list[]
        - `id`: int - code id
        - `code`: string - 注册码。这个码是系统生成全局唯一的
        - `enable`: bool - 该注册码可用
        - `deadline`: datetime|null - 过期时间。超过该时间，注册码就会失效
        - `used_time`: datetime|null - 如果注册码已被使用，这表示被使用的时间
        - `used_user`: string|null - 如果注册码已被使用，这表示使用它的用户的`username`
        - `create_time`: datetime - 注册码被创建的时间

### /api/admin/registration-code/ POST
创建一条新的注册码。  
创建注册码几乎不需要用户的任何输入。

- **Verify**: admin

- **Request Body**
    - `deadline`: (optional)datetime - 设定注册码的过期时间

- **Response Body** [201 Created]
    - `id`: int - code id
    - `code`: string - 注册码。这个码是系统生成全局唯一的
    - `enable`: bool - 该注册码可用
    - `deadline`: datetime|null - 过期时间。超过该时间，注册码就会失效
    - `used_time`: datetime|null - 如果注册码已被使用，这表示被使用的时间
    - `used_user`: string|null - 如果注册码已被使用，这表示使用它的用户的`username`
    - `create_time`: datetime - 注册码被创建的时间

### /api/admin/registration-code/{code-id}/ GET
获得指定注册码的信息。

- **Verify**: admin

- **Path**
    - `code-id`: code id

- **Response Body** [200 Ok]
    内容参考`/api/admin/registration-code/ POST`API。

- **Response Error**
    1. 404 Not Found - 没有找到该code id指定的注册码

### /api/admin/registration-code/{code-id}/ PUT
变更指定的注册码。能变更的内容有两项，`enable`和`deadline`。  
实际上，已经是enable状态的注册码将被禁用并归档，完全无法更改，因此，可行的更新只有两种：
1. 更改`deadline`。
2. 更改`enable`为false，禁用该注册码。

- **Verify**: admin

- **Path**
    - `code-id`: code id

- **Request Body**
    - `deadline`: (optional)datetime
    - `enable`: (optional)bool

- **Response Body** [200 Ok]
    内容参考`/api/admin/registration-code/ POST`API。

- **Response Error**
    1. 404 Not Found - 没有找到该code id指定的注册码

### /api/admin/user/ GET
获得系统中全部用户的列表。

- **Verify**: admin
  
- **Response Body** [200 Ok]
    - list[]
        - `id`: int - user id
        - `username`: string - 用户名
        - `name`: string - 用户显示名
        - `cover`: string|null - 用户头像的文件名
        - `is_staff`: bool - 是否是系统管理员
        - `last_login`: datetime|null - 上次登录时间，如果为null表示用户还没有登录过
        - `last_login_ip`: string|null - 上次登录的IP，如果为null表示可能还没有登录过，或上次登录的IP无法被识别
        - `create_time`: datetime - 用户创建时间
        - `create_path`: string - 用户创建途径
            1. `System` - 该用户由系统创建，只有初始化系统时创建的初始用户会通过此途径创建
            2. `Admin` - 该用户由系统管理员在管理后台创建
            3. `Code` - 该用户使用注册码注册
            4. `Public` - 该用户通过开放注册注册
        - `enable`: bool - 该用户处于可用状态。被禁用的用户将无法登录。

### /api/admin/user/ POST
创建一个新用户。

- **Verify**: admin

- **Request Body**
    - `username`: string
    - `password`: string
    - `name`: string
    - `is_staff`: bool

- **Response Body** [201 Created]
    - `id`: int - user id
    - `username`: string - 用户名
    - `name`: string - 用户显示名
    - `cover`: string|null - 用户头像的文件名
    - `is_staff`: bool - 是否是系统管理员
    - `last_login`: datetime|null - 上次登录时间，如果为null表示用户还没有登录过
    - `last_login_ip`: string|null - 上次登录的IP，如果为null表示可能还没有登录过，或上次登录的IP无法被识别
    - `create_time`: datetime - 用户创建时间
    - `create_path`: string - 用户创建途径
        1. `System` - 该用户由系统创建，只有初始化系统时创建的初始用户会通过此途径创建
        2. `Admin` - 该用户由系统管理员在管理后台创建
        3. `Code` - 该用户使用注册码注册
        4. `Public` - 该用户通过开放注册注册
    - `enable`: bool - 该用户处于可用状态。被禁用的用户将无法登录。

- **Response Error**
    1. 400 Bad Request
        - `Username exist`: 该用户名已经存在
        - `field {field} cannot be empty`: 必填字段有留空

### /api/admin/user/{user}/ GET
获得指定的用户的详细信息。

- **Verify**: admin

- **Path**
    - `user`: user id
    
- **Response Body** [200 Ok]
    参考`/api/admin/user/ POST`API。

- **Response Error**
    1. 404 Not Found - 找不到指定的用户

### /api/admin/user/{user}/ PUT
调整指定用户。  
该API能调整的事项只有停用状态。其他杂项不在管理员要管的事务范围内。

- **Verify**: admin

- **Path**
    - `user`: user id

- **Request Body**
    - `enable`: bool
    
- **Response Body** [200 Ok]
    参考`/api/admin/user/ POST`API。

- **Response Error**
    1. 404 Not Found - 找不到指定的用户

### /api/admin/user/{user}/ DELETE
删除指定用户。  
与封禁不同，用户被删除的操作不可逆，不过仍然可以通过操作数据库逆转。  
即使删除了用户，用户的username仍然不能被再次使用。

- **Verify**: admin

- **Path**
    - `user`: user id
    
- **Response Body** [204 No Content]
    (empty)

- **Response Error**
    1. 404 Not Found - 找不到指定的用户

### /api/admin/user/{user}/password/ PUT
更改指定用户的密码。  
这个API不需要提供旧密码。这是管理员的职能之一。

- **Verify**: admin

- **Path**
    - `user`: user id

- **Request Body**
    - `new_password`: string
    
- **Response Body** [200 Ok]
    (empty)

- **Response Error**
    1. 404 Not Found - 找不到指定的用户

### /api/admin/app/ GET
获得系统全部app的列表。

- **Verify**: admin

- **Response Body** [200 Ok]
    - list[]
        - `id`: int - app id
        - `name`: string - app名称
        - `description`: string - 描述
        - `public`: bool - 是否是公共可见的app。公共可见的app能够出现在用户查询的app列表里。非可见的app不会出现在列表，但是还是能够通过其他途径被用户使用。
        - `enable`: bool - 该app可用。不可用的app将在用户列表不可见，且不能通过app查询接口被调用
        - `create_time`: datetime - 创建时间
        - `update_time`: datetime - 上次更新的时间

### /api/admin/app/ POST
创建新的app。

- **Verify**: admin

- **Request Body**
    - `name`: string - app名称
    - `description`: string - 描述
    - `public`: bool - 是否是公共可见的app。公共可见的app能够出现在用户查询的app列表里。非可见的app不会出现在列表，但是还是能够通过其他途径被用户使用。

- **Response Body** [201 Created]
    - `id`: int - app id
    - `name`: string - app名称
    - `description`: string - 描述
    - `public`: bool - 是否是公共可见的app。公共可见的app能够出现在用户查询的app列表里。非可见的app不会出现在列表，但是还是能够通过其他途径被用户使用。
    - `enable`: bool - 该app可用。不可用的app将在用户列表不可见，且不能通过app查询接口被调用
    - `create_time`: datetime - 创建时间
    - `update_time`: datetime - 上次更新的时间

### /api/admin/app/{app-id}/ GET
获得指定的app的信息。 

- **Verify**: admin

- **Path**
    - `app-id`: app id

- **Response Body**
    参考`/api/admin/app/ POST`API。

- **Response Error**
    1. 404 Not Found - 找不到指定的app

### /api/admin/app/{app-id}/ PUT
变更指定app的信息。

- **Verify**: admin

- **Path**
    - `app-id`: app id

- **Request Body**
    参考`/api/admin/app/ POST`API。

- **Response Body**
    参考`/api/admin/app/ POST`API。

- **Response Error**
    1. 404 Not Found - 找不到指定的app

### /api/admin/app/{app-id}/ DELETE
删除指定的app。

- **Verify**: admin

- **Path**
    - `app-id`: app id

- **Response Body** [204 No Content]
    (empty)

- **Response Error**
    1. 404 Not Found - 找不到指定的app

### /api/admin/app/{app-id}/secret/ GET
获得指定的app的认证接口密码。  
app所代表的应用程序通过本系统查询token时，不走用户接口，而是走内部接口。这个密码将是用来验证app合法的密码。该密码由系统生成。 

- **Verify**: admin

- **Path**
    - `app-id`: app id

- **Response Body**
    - `secret`: string - 密码

- **Response Error**
    1. 404 Not Found - 找不到指定的app

### /api/admin/app/{app-id}/secret/ PUT
重新生成app的认证密码。

- **Verify**: admin

- **Path**
    - `app-id`: app id

- **Response Body**
    - `secret`: string - 密码

- **Response Error**
    1. 404 Not Found - 找不到指定的app

## 应用程序接入API

### /api/ METHOD

- **Verify**: login

- **Path**
    - ``: 

- **Request Body**
    - ``:
    
- **Response Body** [200 Ok]
    - ``:

- **Response Error**
    1. 500 ISE - 

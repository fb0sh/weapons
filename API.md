# Weapons API 文档

基础 URL: `http://localhost:8080`

## 目录

- [用户管理](#用户管理)
- [分类管理](#分类管理)
- [标签管理](#标签管理)

---

## 用户管理

### 注册用户

创建新的用户账户。

**接口地址:** `POST /users/register`

**请求体:**
```json
{
  "username": "字符串",
  "email": "字符串 (可选)",
  "password": "字符串"
}
```

**响应:** `200 OK` - `"User registered successfully"`

**cURL 示例:**
```bash
curl -X POST http://localhost:8080/users/register \
  -H "Content-Type: application/json" \
  -d '{
    "username": "testuser",
    "email": "test@example.com",
    "password": "password123"
  }'
```

---

### 用户登录

验证身份并获取 JWT 令牌。

**接口地址:** `POST /users/login`

**请求体:**
```json
{
  "username": "字符串",
  "password": "字符串"
}
```

**响应:** `200 OK`
```json
{
  "id": "字符串",
  "username": "字符串",
  "email": "字符串 (可选)",
  "created_at": "ISO8601 日期时间",
  "updated_at": "ISO8601 日期时间",
  "token": "JWT 令牌字符串"
}
```

**cURL 示例:**
```bash
curl -X POST http://localhost:8080/users/login \
  -H "Content-Type: application/json" \
  -d '{
    "username": "testuser",
    "password": "password123"
  }'
```

---

### 删除用户

删除用户账户（需要管理员权限）。

**接口地址:** `DELETE /users/{id}`

**请求头:**
- `Authorization: Bearer {token}` (必需，必须是管理员)

**响应:** `200 OK` - `"User deleted successfully"`

**cURL 示例:**
```bash
curl -X DELETE http://localhost:8080/users/user-id-here \
  -H "Authorization: Bearer YOUR_JWT_TOKEN"
```

---

## 分类管理

### 获取所有分类

获取所有分类列表。

**接口地址:** `GET /categories`

**请求头:**
- `Authorization: Bearer {token}` (必需)

**响应:** `200 OK`
```json
[
  {
    "id": "字符串",
    "name": "字符串"
  }
]
```

**cURL 示例:**
```bash
curl -X GET http://localhost:8080/categories \
  -H "Authorization: Bearer YOUR_JWT_TOKEN"
```

---

### 创建分类

创建新的分类。

**接口地址:** `POST /categories`

**请求头:**
- `Authorization: Bearer {token}` (必需)

**请求体:**
```json
{
  "name": "字符串"
}
```

**响应:** `200 OK`
```json
{
  "id": "字符串",
  "name": "字符串"
}
```

**cURL 示例:**
```bash
curl -X POST http://localhost:8080/categories \
  -H "Authorization: Bearer YOUR_JWT_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "电子产品"
  }'
```

---

### 更新分类

更新现有分类。

**接口地址:** `PUT /categories/{category_id}`

**请求头:**
- `Authorization: Bearer {token}` (必需)

**请求体:**
```json
{
  "name": "字符串 (可选)"
}
```

**响应:** `200 OK`
```json
{
  "id": "字符串",
  "name": "字符串"
}
```

**cURL 示例:**
```bash
curl -X PUT http://localhost:8080/categories/category-id-here \
  -H "Authorization: Bearer YOUR_JWT_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "更新后的分类名称"
  }'
```

---

### 删除分类

删除指定分类。

**接口地址:** `DELETE /categories/{category_id}`

**请求头:**
- `Authorization: Bearer {token}` (必需)

**响应:** `200 OK` - `null`

**cURL 示例:**
```bash
curl -X DELETE http://localhost:8080/categories/category-id-here \
  -H "Authorization: Bearer YOUR_JWT_TOKEN"
```

---

## 标签管理

### 获取所有标签

获取所有标签列表。

**接口地址:** `GET /tags`

**请求头:**
- `Authorization: Bearer {token}` (必需)

**响应:** `200 OK`
```json
[
  {
    "id": "字符串",
    "name": "字符串"
  }
]
```

**cURL 示例:**
```bash
curl -X GET http://localhost:8080/tags \
  -H "Authorization: Bearer YOUR_JWT_TOKEN"
```

---

### 创建标签

创建新的标签。

**接口地址:** `POST /tags`

**请求头:**
- `Authorization: Bearer {token}` (必需)

**请求体:**
```json
{
  "name": "字符串"
}
```

**响应:** `200 OK`
```json
{
  "id": "字符串",
  "name": "字符串"
}
```

**cURL 示例:**
```bash
curl -X POST http://localhost:8080/tags \
  -H "Authorization: Bearer YOUR_JWT_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "重要"
  }'
```

---

### 更新标签

更新现有标签。

**接口地址:** `PUT /tags/{tag_id}`

**请求头:**
- `Authorization: Bearer {token}` (必需)

**请求体:**
```json
{
  "name": "字符串 (可选)"
}
```

**响应:** `200 OK`
```json
{
  "id": "字符串",
  "name": "字符串"
}
```

**cURL 示例:**
```bash
curl -X PUT http://localhost:8080/tags/tag-id-here \
  -H "Authorization: Bearer YOUR_JWT_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "紧急"
  }'
```

---

### 删除标签

删除指定标签。

**接口地址:** `DELETE /tags/{tag_id}`

**请求头:**
- `Authorization: Bearer {token}` (必需)

**响应:** `200 OK` - `null`

**cURL 示例:**
```bash
curl -X DELETE http://localhost:8080/tags/tag-id-here \
  -H "Authorization: Bearer YOUR_JWT_TOKEN"
```

---

## 身份认证

大多数接口需要 JWT 身份认证。认证流程如下：

1. 使用 `/users/register` 接口注册用户
2. 使用 `/users/login` 接口登录获取 JWT 令牌
3. 在请求头的 `Authorization` 字段中包含令牌，格式为：`Bearer {token}`

示例:
```bash
export TOKEN="your-jwt-token-here"
curl -H "Authorization: Bearer $TOKEN" http://localhost:8080/categories
```

## 错误响应

所有接口可能返回以下错误状态码：

- `400 Bad Request` - 请求数据无效
- `401 Unauthorized` - 缺少或无效的身份认证
- `403 Forbidden` - 权限不足
- `404 Not Found` - 资源未找到
- `500 Internal Server Error` - 服务器错误

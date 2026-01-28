#!/bin/bash

# Items API 测试脚本
# 包含登录和对 item 的增删改查

BASE_URL="http://localhost:8080"

# 颜色输出
GREEN='\033[0;32m'
BLUE='\033[0;34m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${BLUE}========================================${NC}"
echo -e "${BLUE}      Items API 测试脚本${NC}"
echo -e "${BLUE}========================================${NC}"
echo ""

# 测试用户信息
USERNAME="testuser"
PASSWORD="password123"

# 1. 注册用户（如果用户已存在会失败，但不影响后续操作）
echo -e "${BLUE}[1/8] 注册用户...${NC}"
REGISTER_RESPONSE=$(curl -s -X POST ${BASE_URL}/users/register \
  -H "Content-Type: application/json" \
  -d "{
    \"username\": \"${USERNAME}\",
    \"email\": \"test@example.com\",
    \"password\": \"${PASSWORD}\"
  }")

echo "响应: $REGISTER_RESPONSE"
echo ""

# 2. 登录获取 token
echo -e "${BLUE}[2/8] 用户登录...${NC}"
LOGIN_RESPONSE=$(curl -s -X POST ${BASE_URL}/users/login \
  -H "Content-Type: application/json" \
  -d "{
    \"username\": \"${USERNAME}\",
    \"password\": \"${PASSWORD}\"
  }")

echo "响应: $LOGIN_RESPONSE"

# 提取 token
TOKEN=$(echo $LOGIN_RESPONSE | grep -o '"token":"[^"]*"' | cut -d'"' -f4)

if [ -z "$TOKEN" ]; then
  echo -e "${RED}登录失败，无法获取 token${NC}"
  exit 1
fi

echo -e "${GREEN}Token: $TOKEN${NC}"
echo ""

# 3. 查询所有 items（初始状态）
echo -e "${BLUE}[3/8] 查询所有 items（初始状态）...${NC}"
GET_ALL_RESPONSE=$(curl -s -X GET ${BASE_URL}/items \
  -H "Authorization: Bearer $TOKEN")

echo "响应: $GET_ALL_RESPONSE"
echo ""

# 4. 创建 item
echo -e "${BLUE}[4/8] 创建 item...${NC}"
CREATE_RESPONSE=$(curl -s -X POST ${BASE_URL}/items \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "title": "测试文章",
    "content": "这是一篇测试文章的内容",
    "category_id": "categories:note",
    "tags": ["tags:rust", "tags:web"],
    "format": "markdown",
    "status": "draft",
    "read_scope": "private",
    "authors": ["测试作者"]
  }')

echo "响应: $CREATE_RESPONSE"

# 提取 item ID
ITEM_ID=$(echo $CREATE_RESPONSE | grep -o '"id":"[^"]*"' | cut -d'"' -f4)

if [ -z "$ITEM_ID" ]; then
  echo -e "${RED}创建 item 失败，无法获取 item ID${NC}"
  exit 1
fi

echo -e "${GREEN}Item ID: $ITEM_ID${NC}"
echo ""

# 5. 查询单个 item
echo -e "${BLUE}[5/8] 查询单个 item...${NC}"
GET_ONE_RESPONSE=$(curl -s -X GET ${BASE_URL}/items/${ITEM_ID} \
  -H "Authorization: Bearer $TOKEN")

echo "响应: $GET_ONE_RESPONSE"
echo ""

# 6. 查询所有 items（创建后）
echo -e "${BLUE}[6/8] 查询所有 items（创建后）...${NC}"
GET_ALL_RESPONSE=$(curl -s -X GET ${BASE_URL}/items \
  -H "Authorization: Bearer $TOKEN")

echo "响应: $GET_ALL_RESPONSE"
echo ""

# 7. 更新 item
echo -e "${BLUE}[7/8] 更新 item...${NC}"
UPDATE_RESPONSE=$(curl -s -X PUT ${BASE_URL}/items/${ITEM_ID} \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "title": "更新后的测试文章",
    "content": "这是更新后的文章内容",
    "status": "done"
  }')

echo "响应: $UPDATE_RESPONSE"
echo ""

# 8. 删除 item
echo -e "${BLUE}[8/8] 删除 item...${NC}"
DELETE_RESPONSE=$(curl -s -X DELETE ${BASE_URL}/items/${ITEM_ID} \
  -H "Authorization: Bearer $TOKEN")

echo "响应: $DELETE_RESPONSE"
echo ""

# 验证删除
echo -e "${YELLOW}验证删除结果...${NC}"
VERIFY_RESPONSE=$(curl -s -X GET ${BASE_URL}/items \
  -H "Authorization: Bearer $TOKEN")

echo "当前 items 列表: $VERIFY_RESPONSE"
echo ""

echo -e "${GREEN}========================================${NC}"
echo -e "${GREEN}      测试完成！${NC}"
echo -e "${GREEN}========================================${NC}"

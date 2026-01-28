#!/bin/bash

# API 测试脚本
# 从注册、登录到查询的完整流程

BASE_URL="http://localhost:8080"

# 颜色输出
GREEN='\033[0;32m'
BLUE='\033[0;34m'
RED='\033[0;31m'
NC='\033[0m' # No Color

echo -e "${BLUE}========================================${NC}"
echo -e "${BLUE}      Weapons API 测试脚本${NC}"
echo -e "${BLUE}========================================${NC}"
echo ""

# 1. 注册用户
echo -e "${BLUE}[1/4] 注册用户...${NC}"
REGISTER_RESPONSE=$(curl -s -X POST ${BASE_URL}/users/register \
  -H "Content-Type: application/json" \
  -d '{
    "username": "testuser",
    "email": "test@example.com",
    "password": "password123"
  }')

echo "响应: $REGISTER_RESPONSE"
echo ""

# 2. 登录获取 token
echo -e "${BLUE}[2/4] 用户登录...${NC}"
LOGIN_RESPONSE=$(curl -s -X POST ${BASE_URL}/users/login \
  -H "Content-Type: application/json" \
  -d '{
    "username": "testuser",
    "password": "password123"
  }')

echo "响应: $LOGIN_RESPONSE"

# 提取 token
TOKEN=$(echo $LOGIN_RESPONSE | grep -o '"token":"[^"]*"' | cut -d'"' -f4)

if [ -z "$TOKEN" ]; then
  echo -e "${RED}登录失败，无法获取 token${NC}"
  exit 1
fi

echo -e "${GREEN}Token: $TOKEN${NC}"
echo ""

# 3. 查询分类
echo -e "${BLUE}[3/4] 查询所有分类...${NC}"
CATEGORIES_RESPONSE=$(curl -s -X GET ${BASE_URL}/categories \
  -H "Authorization: Bearer $TOKEN")

echo "响应: $CATEGORIES_RESPONSE"
echo ""

# 4. 查询标签
echo -e "${BLUE}[4/4] 查询所有标签...${NC}"
TAGS_RESPONSE=$(curl -s -X GET ${BASE_URL}/tags \
  -H "Authorization: Bearer $TOKEN")

echo "响应: $TAGS_RESPONSE"
echo ""

echo -e "${GREEN}========================================${NC}"
echo -e "${GREEN}      测试完成！${NC}"
echo -e "${GREEN}========================================${NC}"

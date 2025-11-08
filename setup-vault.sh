#!/bin/bash

# Запуск сервисов
docker-compose -f docker-compose-vault.yaml up -d 

# Ожидание запуска Vault
sleep 10

# Инициализация (только при первом запуске)
docker exec vault vault operator init -key-shares=1 -key-threshold=1 > vault_keys.txt

# Извлечение ключа и токена
UNSEAL_KEY=$(grep "Unseal Key 1" vault_keys.txt | cut -d' ' -f4)
ROOT_TOKEN=$(grep "Initial Root Token" vault_keys.txt | cut -d' ' -f4)

# Разблокировка
docker exec vault vault operator unseal $UNSEAL_KEY

echo "Vault initialized. Root token: $ROOT_TOKEN"
echo "VAULT_TOKEN=$ROOT_TOKEN" >> .env
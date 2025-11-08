# Хранилище (локальные файлы)
storage "file" {
  path = "/vault/file"
}

# Адрес API для доступа снаружи
api_addr = "http://vault:8200"

# Адрес кластера (не обязателен в dev, но полезен для имитации прод)
cluster_addr = "https://vault:8201"

# UI и логирование
ui = true
disable_mlock = true
log_level = "debug"

# Listener (HTTP)
listener "tcp" {
  address = "0.0.0.0:8200"
  tls_disable = 1
}

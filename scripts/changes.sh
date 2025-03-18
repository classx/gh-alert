#!/bin/bash

# Настройка
REPOS=(
  "owner1/repo1:path/to/file1.txt,path/to/file2.json"
  "owner2/repo2:path/to/file3.txt,path/to/file4.json,another/file.txt"
  # Добавьте больше репозиториев и их файлы здесь, разделяя файлы запятыми
)
STATE_FILE="state.txt"
NOTIFICATION_COMMAND="notify-send" # Или используйте другую команду уведомления

# Функция для получения текущего состояния файлов
get_current_state() {
  local state=""
  for repo_files in "${REPOS[@]}"; do
    local repo=$(echo "$repo_files" | cut -d':' -f1)
    local files=$(echo "$repo_files" | cut -d':' -f2)
    local file_array=($(echo "$files" | tr ',' '\n'))

    for file in "${file_array[@]}"; do
      local url="https://raw.githubusercontent.com/$repo/master/$file"
      local hash=$(curl -s "$url" | sha256sum | awk '{print $1}')
      state+="$repo:$file:$hash\n"
    done
  done
  echo "$state"
}

# Функция для проверки изменений
check_for_changes() {
  local current_state=$(get_current_state)
  local previous_state=$(cat "$STATE_FILE" 2>/dev/null)

  if [[ -z "$previous_state" ]]; then
    echo "$current_state" > "$STATE_FILE"
    return
  fi

  if [[ "$current_state" != "$previous_state" ]]; then
    $NOTIFICATION_COMMAND "Изменения обнаружены в отслеживаемых файлах!"
    echo "$current_state" > "$STATE_FILE"
  fi
}

# Основной цикл
check_for_changes
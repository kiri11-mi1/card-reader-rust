# Используем официальный образ Ubuntu
FROM ubuntu:20.04

# Обновление и установка необходимых пакетов
RUN apt-get update && apt-get install -y \
    build-essential \
    usbutils \
    linux-headers-generic \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Установка Rust с использованием rustup
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# Установка переменной окружения
ENV PATH="/root/.cargo/bin:${PATH}"

# Создание рабочей директории
WORKDIR /usr/src/card-reader

# Копирование исходного кода в рабочую директорию
COPY . .

# Сборка программы
RUN /root/.cargo/bin/cargo build --release

# Указываем команду для запуска контейнера
CMD ["bash"]

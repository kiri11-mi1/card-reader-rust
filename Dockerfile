# Этап сборки
FROM rust:latest AS builder

# Создание рабочей директории
WORKDIR /usr/src/card-reader-rust

# Копирование исходного кода в рабочую директорию
COPY ./src ./src
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# Сборка программы
RUN cargo build --release

# Финальный этап
FROM rust:latest

# Копирование скомпилированного бинарного файла из этапа сборки
COPY --from=builder /usr/src/card-reader-rust/target/release/card-reader-rust .

# Указываем команду для запуска контейнера
CMD ["bash"]

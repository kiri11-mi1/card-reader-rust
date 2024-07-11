# Этап сборки
FROM rust:slim-buster AS builder

# Создание рабочей директории
RUN USER=root cargo new --bin card-reader-rust
WORKDIR /card-reader-rust

# Копирование зависимостей
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# Сборка зависимостей для кэширования
RUN cargo build --release
RUN rm src/*.rs

# Копирую исходники
COPY ./src ./src

# Сборка проекта в режиме релиз
RUN rm ./target/release/deps/card_reader_rust*
RUN cargo build --release

# Финальный этап
FROM debian:latest

# Копирование скомпилированного бинарного файла из этапа сборки
COPY --from=builder /card-reader-rust/target/release/card-reader-rust .

# Указываем команду для запуска контейнера
CMD ["bash"]

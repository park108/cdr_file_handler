# cdr_file_handler
CDR(Call Detail Record)를 처리하기 위한 고성능 파일 파서 테스트

## 기술 스택
- Rust 2024

## 환경 구성
### 1. rustup 설치
```shell
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
### 2. 터미널 재실행
### 3. Rust 설치
```shell
$ rustup default stable
```
### 4. cargo 버전 확인
(예시)
```shell
$ cargo --version
cargo 1.87.0 (99624be96 2025-05-06)
```

# 처리 절차

## watcher
- 특정 디렉토리에서 생성되는 파일을 감지한다.
  - Notify Watcher를 설정하여 전달된 채널(tx)에 이벤트를 전송
  - 이벤트가 실제 처리해야 할 파일 변경인지 검사

## parser
- 지정된 CDR 파일을 파싱하고 각 레코드를 처리한다.
  - 파일을 열고, 사전 정의된 Fixed Length CDR의 구조에 따라 파싱한다.

## event_producer (TODO)
- 파싱된 개별 레코드를 이벤트 브로커에 발행한다.


# 테스트

## 실행
- cargo run -- {path}
(예시)
```shell
$ cargo run -- tests/test_data
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.04s
     Running `target/debug/cdr_file_handler tests/test_data`
📁 디렉토리 감시 시작: tests/test_data
```

## 결과
```shell
📌 변경 감지됨: /Users/park108/Dev/rust/cdr_file_handler/tests/test_data/cdr_300k_2.txt
✅ 처리 완료: 2025-05-29 14:47:21, 300000 레코드, 소요 시간: 232.79ms
⏳ 이벤트 없음. 대기 중...
📌 변경 감지됨: /Users/park108/Dev/rust/cdr_file_handler/tests/test_data/cdr_100k_2.txt
✅ 처리 완료: 2025-05-29 14:47:36, 100000 레코드, 소요 시간: 84.47ms
📌 변경 감지됨: /Users/park108/Dev/rust/cdr_file_handler/tests/test_data/cdr_100k_3.txt
✅ 처리 완료: 2025-05-29 14:47:39, 100000 레코드, 소요 시간: 96.14ms
```
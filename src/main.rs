mod cdr;
mod parser;
mod watcher;

use std::env;
use std::path::{Path};
use std::sync::mpsc::channel;
use std::time::{Duration};
use watcher::{setup_watcher, is_relevant_event};
use parser::parse_file;

/// 🎬 CDR File Handler: 디렉토리 내 CDR 파일 생성/수정 감지 및 파싱
/// 📝 주요 기능:
///   1. `notify` 크레이트로 디렉토리 감시
///   2. CDR 레코드 파싱 (고정 길이)
///   3. 이벤트 브로커에 파싱한 레코드 발행
fn main() -> notify::Result<()> {

    // 🛠 CLI 인자 처리: 감시할 디렉토리 또는 파일 경로
    let args: Vec<String> = env::args().collect();
    let watch_path = if args.len() > 1 {
        &args[1]
    }
    else {
        eprintln!("⚠️ 사용법: {} <감시_경로>", args[0]);
        std::process::exit(1);
    };

    // 🔗 채널 생성: notify 이벤트 수신용
    let (tx, rx) = channel();

    // 👀 Watcher 초기화
    let path = Path::new(watch_path);
    let _watcher = setup_watcher(tx, path)?;

    // 🔄 이벤트 루프: 지정된 시간(10초)마다 대기
    loop {
        match rx.recv_timeout(Duration::from_secs(10)) {

            // 🎉 이벤트 수신 성공
            Ok(Ok(event)) => {
                for changed_path in event.paths {

                    // ✅ 필터: 파일 생성(Create) 이벤트만 처리
                    if is_relevant_event(&event.kind) && changed_path.is_file() {

                        println!("📌 변경 감지됨: {}", changed_path.display());

                        parse_file(&changed_path);
                    }
                }
            }

            // ⚠️ 감시 중 오류 발생
            Ok(Err(e)) => eprintln!("⚠️ 감시 오류: {:?}", e),

            // ⏳ 시간 초과: 이벤트 없음
            Err(_) => println!("⏳ 이벤트 없음. 대기 중..."),
        }
    }
}
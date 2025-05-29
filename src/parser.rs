use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use chrono::Local;
use crate::cdr::CdrRecord;

/// 📄 지정된 CDR 파일을 파싱하고 각 레코드를 처리한다.
///
/// # 매개변수
/// - `path`: 파싱할 파일의 경로
pub fn parse_file(path: &Path) {

    let start_time = std::time::Instant::now(); // ⏱ 시작 시각 기록
    let mut processed_count: usize = 0; // 🔢 처리된 레코드 카운터 초기화

    // 🔓 파일 열기 시도
    let file = match File::open(path) {
        Ok(f) => f, // 📂 파일 열기 성공
        Err(e) => {
            eprintln!("❌ 파일 열기 실패: {}", e);
            return;
        }
    };

    // 📚 버퍼를 사용하여 효율적으로 파일 읽기
    let reader = BufReader::new(file);

    // 🌀 파일의 각 줄을 순회하며 파싱
    for (idx, line) in reader.lines().enumerate() {

        // 📥 라인 읽기 및 에러 처리
        let line = line.unwrap_or_default();

        // 🔍 각 라인 파싱 시도
        if let Some(_record) = CdrRecord::parse(&line) {

            // 📄 파싱 성공
            processed_count += 1;

            // 🚀 TODO: 이벤트 브로커 발행 로직 추가
        }
        else {

            // ❗️ 파싱 실패: 경고 로그
            eprintln!("⚠️ [{}] 파싱 실패: {}", idx + 1, line);
        }
    }

    // ⏳ 종료 시각 측정 및 결과 로깅
    let elapsed = start_time.elapsed();

    println!(
        "✅ 처리 완료: {}, {} 레코드, 소요 시간: {:.2?}",
        Local::now().format("%Y-%m-%d %H:%M:%S"),
        processed_count,
        elapsed
    );
}
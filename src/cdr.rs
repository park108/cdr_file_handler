/// 🔍 CDR 레코드를 고정 길이 텍스트 라인에서 파싱하는 구조체 및 메서드
///
/// 필드 위치 (byte index 기반):
/// 0..2   : 교환기유형 (2)          e.g. "CH"
/// 2..5   : 서비스유형 (3)          e.g. "ETC"
/// 5..16  : 발신번호 (11)           e.g. "01062461080"
/// 16..30 : 발신 시작 일시 (14)      e.g. "20250528094700"
/// 30..37 : 사용량 (7)              e.g. "1284741"
impl CdrRecord {

    /// 📥 한 줄(line)을 받아 CdrRecord로 변환
    ///
    /// - `line` 끝의 개행문자(`\r`, `\n`)를 제거한 후
    /// - 최소 길이(37) 검사
    /// - 각 필드를 슬라이스하여 `String`, `u32`로 매핑
    ///
    /// # 반환
    /// - `Some(CdrRecord)`: 파싱 성공
    /// - `None`: 길이 부족 또는 숫자 파싱 실패
    pub(crate) fn parse(line: &str) -> Option<Self> {

        // 🪄 개행문자 제거하여 정확한 바이트 인덱스 매치
        let line = line.trim_end_matches(&['\r', '\n'][..]);

        // 📏 최소 길이 검사 (37 bytes)
        if line.len() < 37 {
            eprintln!("⚠️ 줄 길이 부족: {}, 내용: '{}'", line.len(), line);
            return None;
        }

        // 🧩 각 필드를 분리하여 구조체 생성
        Some(Self {
            switch_type: line[0..2].to_string(),             // 0..2: 교환기유형
            service_type: line[2..5].to_string(),            // 2..5: 서비스유형
            caller_number: line[5..16].to_string(),          // 5..16: 발신번호
            call_start_datetime: line[16..30].to_string(),   // 16..30: 발신 시작 일시
            usage: line[30..37].parse().unwrap_or(0), // 30..37: 사용량
        })
    }
}

/// 🗄️ CDR 레코드 정보를 담는 구조체
///
/// - `switch_type`: 교환기유형
/// - `service_type`: 서비스유형
/// - `caller_number`: 발신번호
/// - `call_start_datetime`: 발신 시작 일시 (yyyyMMddHHmmss)
/// - `usage`: 사용량
#[allow(dead_code)]
#[derive(Debug)]
pub(crate) struct CdrRecord {
    switch_type: String,
    service_type: String,
    caller_number: String,
    call_start_datetime: String,
    usage: u32,
}
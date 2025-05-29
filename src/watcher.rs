use notify::{RecommendedWatcher, RecursiveMode, Config, Watcher, EventKind};
use std::sync::mpsc::Sender;
use std::path::Path;
use notify::event::{CreateKind, ModifyKind, RenameMode};


/// 🔧 Watcher 설정 및 이벤트 필터링 유틸
///
/// - 특정 디렉토리에서 생성되는 파일을 감지한다.

/// 📥 Notify Watcher를 설정하여 전달된 채널(tx)에 이벤트를 전송
///
/// # 매개변수
/// - `tx`: notify 이벤트를 전송할 MPSC 송신자
/// - `path`: 감시할 파일 또는 디렉토리 경로
///
/// # 반환
/// - `Ok(RecommendedWatcher)`: 설정 완료된 Watcher
/// - `Err(e)`: Watcher 생성 또는 감시 시작 실패
pub fn setup_watcher(
    tx: Sender<notify::Result<notify::Event>>,
    path: &Path
) -> notify::Result<RecommendedWatcher> {

    // 👀 RecommendedWatcher 생성 (기본 Config 사용)
    let mut watcher = RecommendedWatcher::new(tx, Config::default())?;

    // 📁 지정된 경로 감시 시작 (NonRecursive: 단일 디렉토리)
    watcher.watch(path, RecursiveMode::NonRecursive)?;
    println!("📁 디렉토리 감시 시작: {}", path.display());

    Ok(watcher)
}

/// 🔍 이벤트가 실제 처리해야 할 파일 변경인지 검사
///
/// - 파일 생성(CreateKind::File)
/// - 다른 위치에서 디렉토리로 이동(RenameMode::To)
///
/// # 매개변수
/// - `kind`: notify 이벤트 종류
///
/// # 반환
/// - `true`: 처리 대상 이벤트
/// - `false`: 무시할 이벤트
pub fn is_relevant_event(kind: &EventKind) -> bool {
    matches!(kind,
        EventKind::Create(CreateKind::File) // ▶️ 새 파일이 생성된 경우
        | EventKind::Modify(ModifyKind::Name(RenameMode::To)) // 🔀 파일이 Rename(To)되어 들어온 경우
    )
}
slint::include_modules!();

use slint::{ModelRc, VecModel, SharedString};
use std::rc::Rc;
use tokio;

use crate::builds_info_checker::{BuildSummary, get_all_builds_data, check_builds};

impl From<BuildSummary> for BuildTileData {
    fn from(build: BuildSummary) -> Self {
        BuildTileData {
            name: SharedString::from(build.name),
            status: SharedString::from(build.status),
            duration: SharedString::from(format!("{:.2} sek", build.duration)),
            time: SharedString::from(build.start_time.format("%Y-%m-%d %H:%M").to_string()),
        }
    }
}

pub async fn show() -> Result<(), Box<dyn std::error::Error>> {
    let initial_data = get_all_builds_data().await?;
    let ui = MainWindow::new()?;

    // Inicjalizacja UI
    let tiles_model = Rc::new(VecModel::from(
        initial_data.into_iter()
            .map(BuildTileData::from)
            .collect::<Vec<BuildTileData>>()
    ));
    ui.set_tiles(ModelRc::from(tiles_model));

    let ui_weak = ui.as_weak();

    // Używamy zwykłego spawn z Tokio
    tokio::spawn(async move {
        let update_callback = move |new_data: Vec<BuildSummary>| {
            let ui_weak_clone = ui_weak.clone();
            let build_tiles_data: Vec<BuildTileData> = new_data.into_iter()
                .map(BuildTileData::from)
                .collect();

            // Przekazujemy dane do wątku UI
            slint::invoke_from_event_loop(move || {
                if let Some(ui) = ui_weak_clone.upgrade() {
                    let vec_model = Rc::new(VecModel::from(build_tiles_data));
                    ui.set_tiles(ModelRc::from(vec_model));
                }
            }).unwrap_or_else(|e| {
                eprintln!("Błąd aktualizacji UI: {}", e);
            });
        };

        if let Err(e) = check_builds(update_callback).await {
            eprintln!("Błąd podczas sprawdzania buildów: {}", e);
        }
    });

    // Uruchamiamy główną pętlę UI
    ui.run()?;

    Ok(())
}
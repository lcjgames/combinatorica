#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AppState {
    Loading,
    MainMenu,
    FleetEditor,
    ShipEditor,
    Battle,
}

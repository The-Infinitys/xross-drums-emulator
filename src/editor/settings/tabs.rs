#[derive(PartialEq, Eq, Clone, Copy)]
pub enum SelectedTab {
    Kick,
    Snare,
    HiHat,
    Crash,
    Ride,
    TomH,
    TomL,
    Floor,
    Master,
}

pub const DRUM_TABS: [(&str, SelectedTab); 8] = [
    ("Kick", SelectedTab::Kick),
    ("Snare", SelectedTab::Snare),
    ("Hi-Hat", SelectedTab::HiHat),
    ("Crash", SelectedTab::Crash),
    ("Ride", SelectedTab::Ride),
    ("Tom H", SelectedTab::TomH),
    ("Tom L", SelectedTab::TomL),
    ("Floor", SelectedTab::Floor),
];

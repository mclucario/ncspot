use std::sync::{Arc, RwLock};

use cursive::view::ViewWrapper;
use cursive::Cursive;

use crate::command::Command;
use crate::commands::CommandResult;
use crate::library::Library;
use crate::playlist::Playlist;
use crate::queue::Queue;
use crate::track::Track;
use crate::traits::ViewExt;
use crate::ui::listview::ListView;

pub struct PlaylistView {
    playlist: Playlist,
    list: ListView<Track>,
}

impl PlaylistView {
    pub fn new(queue: Arc<Queue>, library: Arc<Library>, playlist: &Playlist) -> Self {
        let mut playlist = playlist.clone();

        playlist.load_tracks(queue.get_spotify());

        let tracks = if let Some(t) = playlist.tracks.as_ref() {
            t.clone()
        } else {
            Vec::new()
        };

        let list = ListView::new(Arc::new(RwLock::new(tracks)), queue, library);

        Self { playlist, list }
    }
}

impl ViewWrapper for PlaylistView {
    wrap_impl!(self.list: ListView<Track>);
}

impl ViewExt for PlaylistView {
    fn title(&self) -> String {
        self.playlist.name.clone()
    }

    fn on_command(&mut self, s: &mut Cursive, cmd: &Command) -> Result<CommandResult, String> {
        self.list.on_command(s, cmd)
    }
}

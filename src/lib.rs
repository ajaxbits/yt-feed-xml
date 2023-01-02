mod channel;
mod playlist;
mod user;
mod video;
mod xml_feed;

pub use channel::Channel;
pub use playlist::Playlist;
pub use user::User;

#[cfg(test)]
mod tests {
    #[test]
    fn test_setup() {
        color_eyre::install().expect("color_eyre::install can only be called once");
    }
}

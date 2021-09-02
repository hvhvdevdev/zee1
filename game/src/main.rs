fn main() {
    let video = zee1_engine::video::sdl::SdlVideo::new();
    zee1_engine::Service::start(&video);
}

use godot::{classes::Engine, prelude::*};
use godot_tokio::AsyncRuntime;

struct QuicConnections;

#[gdextension]
unsafe impl ExtensionLibrary for QuicConnections {
    fn on_level_init(level: InitLevel) {
        match level {
            InitLevel::Scene => {
                let mut engine = Engine::singleton();

                // This is where we register our async runtime singleton.
                engine.register_singleton(AsyncRuntime::SINGLETON, &AsyncRuntime::new_alloc());
            }
            _ => (),
        }
    }

    fn on_level_deinit(level: InitLevel) {
        match level {
            InitLevel::Scene => {
                let mut engine = Engine::singleton();

                // Here is where we free our async runtime singleton from memory.
                if let Some(async_singleton) = engine.get_singleton(AsyncRuntime::SINGLETON) {
                    engine.unregister_singleton(AsyncRuntime::SINGLETON);
                    async_singleton.free();
                } else {
                    godot_warn!(
                        "Failed to find & free singleton -> {}",
                        AsyncRuntime::SINGLETON
                    );
                }
            }
            _ => (),
        }
    }
}

#[derive(GodotClass)]
#[class(base=RefCounted)]
struct QuicServer {
    base: Base<RefCounted>,
}

#[godot_api]
impl IRefCounted for QuicServer {
    fn init(base: Base<RefCounted>) -> Self {
        Self { base }
    }
}

#[godot_api]
impl QuicServer {}

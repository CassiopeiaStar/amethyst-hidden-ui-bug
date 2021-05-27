use amethyst::{
    assets::{LoaderBundle,DefaultLoader,Loader},
    prelude::*,
    renderer::{
        plugins::{RenderToWindow},
        rendy::hal::command::ClearColor,
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
    input::InputBundle,
    ui::{UiBundle,RenderUi,UiTransform,Anchor,UiImage,UiText,LineMode},
    winit::event::{Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    core::Hidden
};


fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;
    let display_config_path = app_root.join("config/display.ron");
    let assets_dir = app_root.join("assets/");

    let mut dispatcher = DispatcherBuilder::default();
    dispatcher
        .add_bundle(LoaderBundle)
        .add_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?.with_clear(ClearColor {
                        float32: [0.0, 0.0, 0.0, 1.0],
                    }),
                )
                .with_plugin(RenderUi::default()),
        )
        .add_bundle(InputBundle::new())
        .add_bundle(UiBundle::<u32>::new())
        .flush();

    let game = Application::new(assets_dir, GameState, dispatcher)?;
    game.run();
    Ok(())
}

struct GameState;

impl SimpleState for GameState {
    fn on_start(&mut self, mut data: StateData<'_,GameData>) {
        //create an eample ui element without the 'Hidden' component
        init_shown_ui_element(&mut data);

        //create some text explaining the test
        init_text_ui(&mut data);
    }
    fn handle_event(&mut self, mut data: StateData<'_, GameData>, event: StateEvent) -> SimpleTrans {
        if let StateEvent::Window(event) = &event {
            match event {
                Event::WindowEvent { event, .. } => {
                    match event {
                        WindowEvent::KeyboardInput {
                            input:
                                KeyboardInput {
                                    virtual_keycode: Some(VirtualKeyCode::Escape),
                                    ..
                                },
                            ..
                        } | WindowEvent::CloseRequested => Trans::Quit,

                        WindowEvent::KeyboardInput{
                            input:
                            KeyboardInput {
                                virtual_keycode: Some(VirtualKeyCode::Space),
                                ..
                            },
                            ..
                        } => {
                            init_hidden_ui_element(&mut data);
                            Trans::None
                        }

                        _ => Trans::None,
                    }
                }
                _ => Trans::None,
            }
        } else {
            Trans::None
        }
    }
}

//explanation text
fn init_text_ui(data: &mut StateData<'_,GameData>){
    let loader = data.resources.get::<DefaultLoader>().unwrap();
    let font = loader.load("font/square.ttf");
    data.world.push((
        UiTransform::new(
            "text_ui".to_string(),
            Anchor::TopMiddle,
            Anchor::TopMiddle,
            0.,-10.,0.,300.,100.,
        ),
        UiText::new(
            Some(font),
            "Press Space to add a new entity with a 'Hidden' component".to_string(),
            [0.2,0.2,1.,1.0],
            20.,
            LineMode::Wrap,
            Anchor::Middle
        )
    ));
}

//this is just to show that the same entity is created just fine without Hidden
fn init_shown_ui_element(data: &mut StateData<'_,GameData>){
    data.world.push((
        UiTransform::new(
            "shown_ui".to_string(),
            Anchor::Middle,
            Anchor::Middle,
            -100.,0.,0.,100.,100.,
        ),
        UiImage::SolidColor([0.,1.,0.,1.])
    ));
}

//Panics!
fn init_hidden_ui_element(data: &mut StateData<'_,GameData>){
    data.world.push((
        UiTransform::new(
            "hidden_ui".to_string(),
            Anchor::Middle,
            Anchor::Middle,
            0.,0.,0.,100.,100.,
        ),
        UiImage::SolidColor([1.,0.,0.,1.]),
        Hidden
    ));
}

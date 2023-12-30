use iced::{widget::Component, Element, Sandbox, Length};

type Message = ();

struct App;

impl Sandbox for App {
    type Message = ();

    fn new() -> Self {
        App
    }

    fn title(&self) -> String {
        "Picklist in component bug".to_owned()
    }

    fn update(&mut self, _message: Self::Message) {}

    fn view(&self) -> Element<'_, Self::Message> {
        iced::widget::container(
            iced::widget::row![
                iced::widget::component(MyComponent),
                my_pick_list().map(|_| ()),
            ]
        )
        .center_x()
        .center_y()
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }
}

struct MyComponent;

fn my_pick_list() -> iced::Element<'static, &'static str> {
    iced::widget::pick_list(&["A", "B", "C"][..], None, |x| x).into()
}

impl Component<Message, iced::Renderer> for MyComponent {
    type State = ();
    type Event = ();

    fn update(&mut self, _state: &mut Self::State, _event: Self::Event) -> Option<()> {
        None
    }

    fn view(&self, _state: &Self::State) -> Element<'_, Self::Event> {
        my_pick_list().map(|_| ())
    }
}

fn main() {
    App::run(Default::default()).unwrap();
}

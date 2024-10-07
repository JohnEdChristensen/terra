use iced::widget::canvas::Stroke;
use iced::widget::container::bordered_box;
use iced::widget::{button, column};
use iced::{
    advanced::mouse,
    widget::{canvas, container, Column},
    Color, Element, Rectangle, Renderer, Theme,
};
use iced::{Border, Length, Size, Vector};

const WIDTH: f32 = 300.;
const HEIGHT: f32 = 400.;

pub fn main() -> iced::Result {
    iced::application("A counter", update, view)
        .theme(|_state| Theme::Ferra)
        .window_size(Size::new(WIDTH, HEIGHT))
        .run()
}

#[derive(Debug, Clone)]
enum Message {
    Increment,
}

#[derive(Default)]
struct State {
    val: i32,
}

fn update(state: &mut State, message: Message) {
    match message {
        Message::Increment => state.val += 1,
    }
}

fn view(state: &State) -> Column<Message> {
    let viewport: Element<Message> = canvas(Circle { radius: 50.0 })
        .width(Length::Fixed(WIDTH))
        .height(Length::Fixed(WIDTH))
        .into();

    column![
        container(viewport).style(|theme| bordered_box(theme)
            .background(theme.palette().background)
            .border(Border {
                width: 4.,
                color: theme.extended_palette().success.weak.color,
                ..Default::default()
            })),
        iced::widget::text(state.val.to_string()),
        button("+").on_press(Message::Increment)
    ]
}

// First, we define the data we need for drawing
#[derive(Debug)]
struct Circle {
    radius: f32,
}

// Then, we implement the `Program` trait
impl<Message> canvas::Program<Message> for Circle {
    // No internal state
    type State = ();

    fn draw(
        &self,
        _state: &(),
        renderer: &Renderer,
        theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        // We prepare a new `Frame`
        let mut frame = canvas::Frame::new(renderer, bounds.size());
        frame.scale(0.9);
        frame.translate(Vector::new(WIDTH / 20., WIDTH / 20.).into());

        let stroke = Stroke::default()
            .with_color(Color {
                r: 0.6,
                g: 0.6,
                b: 0.7,
                a: 1.,
            })
            .with_width(2.);

        let center = frame.center().clone();
        let xs = 0..=WIDTH as i32 / 30;
        let ys = 0..=WIDTH as i32 / 30;

        let y_lines = ys
            .map(|y| {
                canvas::Path::line(
                    (0., (y * 30) as f32).into(),
                    (WIDTH, (y * 30) as f32).into(),
                )
            })
            .collect::<Vec<_>>();
        let x_lines: Vec<_> = xs
            .map(|x| {
                canvas::Path::line(
                    ((x * 30) as f32, 0.).into(),
                    ((x * 30) as f32, WIDTH).into(),
                )
            })
            .collect();

        let line = canvas::Path::line(frame.center(), (100., 100.).into());
        x_lines.into_iter().for_each(|l| frame.stroke(&l, stroke));
        y_lines.into_iter().for_each(|l| frame.stroke(&l, stroke));

        //// We create a `Path` representing a simple circle
        //let circle = canvas::Path::circle(frame.center(), self.radius);
        //
        //// And fill it with some color
        //frame.fill(&circle, theme.palette().success);

        // Then, we produce the geometry
        vec![frame.into_geometry()]
    }
}

use iced::{
    advanced::{
        graphics::core::event,
        layout, renderer, text,
        widget::{self, tree, Tree, Widget},
        Clipboard, Layout, Shell,
    },
    alignment, mouse, touch, window, Border, Color, Element, Event, Length, Pixels, Rectangle,
    Size, Theme,
};

pub struct BookToggler<'a, Message, Theme = iced::Theme, Renderer = iced::Renderer>
where
    Theme: Catalog,
    Renderer: text::Renderer,
{
    is_toggled: bool,
    on_toggle: Option<Box<dyn Fn((bool, String)) -> Message + 'a>>,
    label: Option<text::Fragment<'a>>,
    width: Length,
    size: f32,
    text_size: Option<Pixels>,
    text_line_height: text::LineHeight,
    text_alignment: alignment::Horizontal,
    text_shaping: text::Shaping,
    text_wrapping: text::Wrapping,
    spacing: f32,
    font: Option<Renderer::Font>,
    class: Theme::Class<'a>,
    last_status: Option<Status>,
}

impl<'a, Message, Theme, Renderer> BookToggler<'a, Message, Theme, Renderer>
where
    Theme: Catalog,
    Renderer: text::Renderer,
{
    pub const DEFAULT_SIZE: f32 = 16.0;

    pub fn new(is_toggled: bool) -> Self {
        BookToggler {
            is_toggled,
            on_toggle: None,
            label: None,
            width: Length::Shrink,
            size: Self::DEFAULT_SIZE,
            text_size: None,
            text_line_height: text::LineHeight::default(),
            text_alignment: alignment::Horizontal::Left,
            text_shaping: text::Shaping::default(),
            text_wrapping: text::Wrapping::default(),
            spacing: Self::DEFAULT_SIZE / 2.0,
            font: None,
            class: Theme::default(),
            last_status: None,
        }
    }

    pub fn label(mut self, label: impl text::IntoFragment<'a>) -> Self {
        self.label = Some(label.into_fragment());
        self
    }

    pub fn on_toggle(mut self, on_toggle: impl Fn((bool, String)) -> Message + 'a) -> Self {
        self.on_toggle = Some(Box::new(on_toggle));
        self
    }

    pub fn on_toggle_maybe(
        mut self,
        on_toggle: Option<impl Fn((bool, String)) -> Message + 'a>,
    ) -> Self {
        self.on_toggle = on_toggle.map(|on_toggle| Box::new(on_toggle) as _);
        self
    }

    pub fn size(mut self, size: impl Into<Pixels>) -> Self {
        self.size = size.into().0;
        self
    }

    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    pub fn text_size(mut self, text_size: impl Into<Pixels>) -> Self {
        self.text_size = Some(text_size.into());
        self
    }

    pub fn text_line_height(mut self, line_height: impl Into<text::LineHeight>) -> Self {
        self.text_line_height = line_height.into();
        self
    }

    pub fn text_alignment(mut self, alignment: alignment::Horizontal) -> Self {
        self.text_alignment = alignment;
        self
    }

    pub fn text_shaping(mut self, shaping: text::Shaping) -> Self {
        self.text_shaping = shaping;
        self
    }

    pub fn text_wrapping(mut self, wrapping: text::Wrapping) -> Self {
        self.text_wrapping = wrapping;
        self
    }

    pub fn spacing(mut self, spacing: impl Into<Pixels>) -> Self {
        self.spacing = spacing.into().0;
        self
    }

    pub fn font(mut self, font: impl Into<Renderer::Font>) -> Self {
        self.font = Some(font.into());
        self
    }

    #[must_use]
    pub fn style(mut self, style: impl Fn(&Theme, Status) -> Style + 'a) -> Self
    where
        Theme::Class<'a>: From<StyleFn<'a, Theme>>,
    {
        self.class = (Box::new(style) as StyleFn<'a, Theme>).into();
        self
    }
}

impl<'a, Message, Theme, Renderer> Widget<Message, Theme, Renderer>
    for BookToggler<'a, Message, Theme, Renderer>
where
    Theme: Catalog,
    Renderer: text::Renderer,
{
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<widget::text::State<Renderer::Paragraph>>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(widget::text::State::<Renderer::Paragraph>::default())
    }

    fn size(&self) -> Size<Length> {
        Size {
            width: self.width,
            height: Length::Shrink,
        }
    }

    fn layout(
        &self,
        tree: &mut Tree,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let limits = limits.width(self.width);

        layout::next_to_each_other(
            &limits,
            self.spacing,
            |_| layout::Node::new(Size::new(2.0 * self.size, self.size)),
            |limits| {
                if let Some(label) = self.label.as_deref() {
                    let state = tree
                        .state
                        .downcast_mut::<widget::text::State<Renderer::Paragraph>>();

                    widget::text::layout(
                        state,
                        renderer,
                        limits,
                        self.width,
                        Length::Shrink,
                        label,
                        self.text_line_height,
                        self.text_size,
                        self.font,
                        self.text_alignment,
                        alignment::Vertical::Top,
                        self.text_shaping,
                        self.text_wrapping,
                    )
                } else {
                    layout::Node::new(Size::ZERO)
                }
            },
        )
    }

    fn on_event(
        &mut self,
        _state: &mut Tree,
        event: Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _renderer: &Renderer,
        _clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        _viewport: &Rectangle,
    ) -> event::Status {
        let Some(on_toggle) = &self.on_toggle else {
            return event::Status::Ignored;
        };

        let event_status = match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerPressed { .. }) => {
                let mouse_over = cursor.is_over(layout.bounds());

                if mouse_over {
                    shell.publish(on_toggle((
                        !self.is_toggled,
                        self.label.clone().unwrap().to_string(),
                    )));
                    event::Status::Captured
                } else {
                    event::Status::Ignored
                }
            }
            _ => event::Status::Ignored,
        };

        let current_status = if self.on_toggle.is_none() {
            Status::Disabled
        } else if cursor.is_over(layout.bounds()) {
            Status::Hovered {
                is_toggled: self.is_toggled,
            }
        } else {
            Status::Active {
                is_toggled: self.is_toggled,
            }
        };

        if let Event::Window(window::Event::RedrawRequested(_now)) = event {
            self.last_status = Some(current_status);
        } else if self
            .last_status
            .is_some_and(|status| status != current_status)
        {
            shell.request_redraw(window::RedrawRequest::NextFrame);
        }

        event_status
    }

    fn mouse_interaction(
        &self,
        _state: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _viewport: &Rectangle,
        _renderer: &Renderer,
    ) -> mouse::Interaction {
        if cursor.is_over(layout.bounds()) {
            if self.on_toggle.is_some() {
                mouse::Interaction::Pointer
            } else {
                mouse::Interaction::NotAllowed
            }
        } else {
            mouse::Interaction::default()
        }
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        _cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        const BORDER_RADIUS_RATIO: f32 = 32.0 / 13.0;

        const SPACE_RATIO: f32 = 0.05;

        let mut children = layout.children();
        let toggler_layout = children.next().unwrap();

        if self.label.is_some() {
            let label_layout = children.next().unwrap();
            let state: &widget::text::State<Renderer::Paragraph> = tree.state.downcast_ref();

            crate::text::draw(
                renderer,
                style,
                label_layout,
                state.0.raw(),
                crate::text::Style::default(),
                viewport,
            );
        }

        let bounds = toggler_layout.bounds();
        let style = theme.style(&self.class, self.last_status.unwrap_or(Status::Disabled));

        let border_radius = bounds.height / BORDER_RADIUS_RATIO;
        let space = SPACE_RATIO * bounds.height;

        let toggler_background_bounds = Rectangle {
            x: bounds.x + space,
            y: bounds.y + space,
            width: bounds.width - (2.0 * space),
            height: bounds.height - (2.0 * space),
        };

        renderer.fill_quad(
            renderer::Quad {
                bounds: toggler_background_bounds,
                border: Border {
                    radius: border_radius.into(),
                    width: style.background_border_width,
                    color: style.background_border_color,
                },
                ..renderer::Quad::default()
            },
            style.background,
        );

        let toggler_foreground_bounds = Rectangle {
            x: bounds.x
                + if self.is_toggled {
                    bounds.width - 2.0 * space - (bounds.height - (4.0 * space))
                } else {
                    2.0 * space
                },
            y: bounds.y + (2.0 * space),
            width: bounds.height - (4.0 * space),
            height: bounds.height - (4.0 * space),
        };

        renderer.fill_quad(
            renderer::Quad {
                bounds: toggler_foreground_bounds,
                border: Border {
                    radius: border_radius.into(),
                    width: style.foreground_border_width,
                    color: style.foreground_border_color,
                },
                ..renderer::Quad::default()
            },
            style.foreground,
        );
    }
}

impl<'a, Message, Theme, Renderer> From<BookToggler<'a, Message, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Theme: Catalog + 'a,
    Renderer: text::Renderer + 'a,
{
    fn from(
        toggler: BookToggler<'a, Message, Theme, Renderer>,
    ) -> Element<'a, Message, Theme, Renderer> {
        Element::new(toggler)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    Active { is_toggled: bool },
    Hovered { is_toggled: bool },
    Disabled,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Style {
    pub background: Color,
    pub background_border_width: f32,
    pub background_border_color: Color,
    pub foreground: Color,
    pub foreground_border_width: f32,
    pub foreground_border_color: Color,
}

pub trait Catalog: Sized {
    type Class<'a>;

    fn default<'a>() -> Self::Class<'a>;

    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style;
}

pub type StyleFn<'a, Theme> = Box<dyn Fn(&Theme, Status) -> Style + 'a>;

impl Catalog for Theme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(default)
    }

    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style {
        class(self, status)
    }
}

pub fn default(theme: &Theme, status: Status) -> Style {
    let palette = theme.extended_palette();

    let background = match status {
        Status::Active { is_toggled } | Status::Hovered { is_toggled } => {
            if is_toggled {
                palette.primary.strong.color
            } else {
                palette.background.strong.color
            }
        }
        Status::Disabled => palette.background.weak.color,
    };

    let foreground = match status {
        Status::Active { is_toggled } => {
            if is_toggled {
                palette.primary.strong.text
            } else {
                palette.background.base.color
            }
        }
        Status::Hovered { is_toggled } => {
            if is_toggled {
                Color {
                    a: 0.5,
                    ..palette.primary.strong.text
                }
            } else {
                palette.background.weak.color
            }
        }
        Status::Disabled => palette.background.base.color,
    };

    Style {
        background,
        foreground,
        foreground_border_width: 0.0,
        foreground_border_color: Color::TRANSPARENT,
        background_border_width: 0.0,
        background_border_color: Color::TRANSPARENT,
    }
}

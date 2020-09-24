//! Show toggle controls using togglers.
use std::hash::Hash;

use crate::{
    event, layout, mouse, row, text, Align, Clipboard, Element, Event, Hasher,
    HorizontalAlignment, Layout, Length, Point, Rectangle, Row, Text,
    VerticalAlignment, Widget,
};

/// A toggler widget
///
/// # Example
///
/// ```
/// # type Toggler<Message> = iced_native::Toggler<Message, iced_native::renderer::Null>;
/// #
/// pub enum Message {
///     TogglerToggled(bool),
/// }
///
/// let is_active = true;
///
/// Toggler::new(is_active, "Toggle me!", |b| Message::TogglerToggled(b));
/// ```
///
#[allow(missing_debug_implementations)]
pub struct Toggler<Message, Renderer: self::Renderer + text::Renderer> {
    is_active: bool,
    on_toggle: Box<dyn Fn(bool) -> Message>,
    label: String,
    width: Length,
    size: u16,
    text_size: Option<u16>,
    text_align: Option<HorizontalAlignment>,
    spacing: u16,
    font: Renderer::Font,
    style: Renderer::Style,
}

impl<Message, Renderer: self::Renderer + text::Renderer>
    Toggler<Message, Renderer>
{
    /// Creates a new [`Toggler`].
    ///
    /// It expects:
    ///   * a boolean describing whether the [`Toggler`] is checked or not
    ///   * the label of the [`Toggler`]
    ///   * a function that will be called when the [`Toggler`] is toggled. It
    ///     will receive the new state of the [`Toggler`] and must produce a
    ///     `Message`.
    ///
    /// [`Toggler`]: struct.Toggler.html
    pub fn new<F>(is_active: bool, label: impl Into<String>, f: F) -> Self
    where
        F: 'static + Fn(bool) -> Message,
    {
        Toggler {
            is_active,
            on_toggle: Box::new(f),
            label: label.into(),
            width: Length::Fill,
            size: <Renderer as self::Renderer>::DEFAULT_SIZE,
            text_size: None,
            text_align: None,
            spacing: 0,
            font: Renderer::Font::default(),
            style: Renderer::Style::default(),
        }
    }

    /// Sets the size of the [`Toggler`].
    ///
    /// [`Toggler`]: struct.Toggler.html
    pub fn size(mut self, size: u16) -> Self {
        self.size = size;
        self
    }

    /// Sets the width of the [`Toggler`].
    ///
    /// [`Toggler`]: struct.Toggler.html
    pub fn width(mut self, width: Length) -> Self {
        self.width = width;
        self
    }

    /// Sets the text size o the [`Toggler`].
    ///
    /// [`Toggler`]: struct.Toggler.html
    pub fn text_size(mut self, text_size: u16) -> Self {
        self.text_size = Some(text_size);
        self
    }

    /// Sets the alignment of the text of the [`Toggler`]
    ///
    /// [`Toggler`]: struct.Toggler.html
    pub fn text_align(mut self, align: HorizontalAlignment) -> Self {
        self.text_align = Some(align);
        self
    }

    /// Sets the spacing between the [`Toggler`] and the text.
    ///
    /// [`Toggler`]: struct.Toggler.html
    pub fn spacing(mut self, spacing: u16) -> Self {
        self.spacing = spacing;
        self
    }

    /// Sets the [`Font`] of the text of the [`Toggler`]
    ///
    /// [`Toggler`]: struct.Toggler.html
    /// [`Font`]: ../../struct.Font.html
    pub fn font(mut self, font: Renderer::Font) -> Self {
        self.font = font;
        self
    }

    /// Sets the style of the [`Toggler`].
    ///
    /// [`Toggler`]: struct.Toggler.html
    pub fn style(mut self, style: impl Into<Renderer::Style>) -> Self {
        self.style = style.into();
        self
    }
}

impl<Message, Renderer> Widget<Message, Renderer> for Toggler<Message, Renderer>
where
    Renderer: self::Renderer + text::Renderer + row::Renderer,
{
    fn width(&self) -> Length {
        self.width
    }

    fn height(&self) -> Length {
        Length::Shrink
    }

    fn layout(
        &self,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        Row::<(), Renderer>::new()
            .width(self.width)
            .spacing(self.spacing)
            .align_items(Align::Center)
            .push(
                Text::new(&self.label)
                    .horizontal_alignment(
                        self.text_align.unwrap_or(HorizontalAlignment::Left),
                    )
                    .font(self.font)
                    .width(self.width)
                    .size(self.text_size.unwrap_or(renderer.default_size())),
            )
            .push(
                Row::new()
                    .width(Length::Units(2 * self.size))
                    .height(Length::Units(self.size)),
            )
            .layout(renderer, limits)
    }

    fn on_event(
        &mut self,
        event: Event,
        layout: Layout<'_>,
        cursor_position: Point,
        _renderer: &Renderer,
        _clipboard: &mut dyn Clipboard,
        messages: &mut Vec<Message>,
    ) -> event::Status {
        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                let mouse_over = layout.bounds().contains(cursor_position);

                if mouse_over {
                    messages.push((self.on_toggle)(!self.is_active));

                    event::Status::Captured
                } else {
                    event::Status::Ignored
                }
            }
            _ => event::Status::Ignored,
        }
    }

    fn draw(
        &self,
        renderer: &mut Renderer,
        defaults: &Renderer::Defaults,
        layout: Layout<'_>,
        cursor_position: Point,
        _viewport: &Rectangle,
    ) -> Renderer::Output {
        let bounds = layout.bounds();
        let mut children = layout.children();

        let label_layout = children.next().unwrap();
        let toggler_layout = children.next().unwrap();
        let toggler_bounds = toggler_layout.bounds();

        let label = text::Renderer::draw(
            renderer,
            defaults,
            label_layout.bounds(),
            &self.label,
            self.text_size.unwrap_or(renderer.default_size()),
            self.font,
            None,
            self.text_align.unwrap_or(HorizontalAlignment::Left),
            VerticalAlignment::Center,
        );

        let is_mouse_over = bounds.contains(cursor_position);

        self::Renderer::draw(
            renderer,
            toggler_bounds,
            self.is_active,
            is_mouse_over,
            label,
            &self.style,
        )
    }

    fn hash_layout(&self, state: &mut Hasher) {
        struct Marker;
        std::any::TypeId::of::<Marker>().hash(state);

        self.label.hash(state)
    }
}

/// The renderer of a [`Toggler`].
///
/// Your [renderer] will need to implement this trait before being
/// able to use a [`Toggler`] in your user interface.
///
/// [`Toggler`]: struct.Toggler.html
/// [renderer]: ../../renderer/index.html
pub trait Renderer: crate::Renderer {
    /// The style supported by this renderer.
    type Style: Default;

    /// The default size of a [`Toggler`].
    ///
    /// [`Toggler`]: struct.Toggler.html
    const DEFAULT_SIZE: u16;

    /// Draws a [`Toggler`].
    ///
    /// It receives:
    ///   * the bounds of the [`Toggler`]
    ///   * whether the [`Toggler`] is activated or not
    ///   * whether the mouse is over the [`Toggler`] or not
    ///   * the drawn label of the [`Toggler`]
    ///   * the style of the [`Toggler`]
    ///
    /// [`Toggler`]: struct.Toggler.html
    fn draw(
        &mut self,
        bounds: Rectangle,
        is_active: bool,
        is_mouse_over: bool,
        label: Self::Output,
        style: &Self::Style,
    ) -> Self::Output;
}

impl<'a, Message, Renderer> From<Toggler<Message, Renderer>>
    for Element<'a, Message, Renderer>
where
    Renderer: 'a + self::Renderer + text::Renderer + row::Renderer,
    Message: 'a,
{
    fn from(
        toggler: Toggler<Message, Renderer>,
    ) -> Element<'a, Message, Renderer> {
        Element::new(toggler)
    }
}

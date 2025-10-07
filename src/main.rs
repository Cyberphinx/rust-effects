use std::{cell::RefCell, io, rc::Rc, time::Duration};

use ratatui::{
    layout::Alignment,
    style::{Color, Style, Stylize},
    widgets::{Block, BorderType, Paragraph},
    Frame, Terminal,
};

use ratzilla::{
    event::{KeyCode, KeyEvent},
    DomBackend, WebRenderer,
};

use tachyonfx::{fx, CellFilter, EffectManager, Interpolation, Shader};

fn main() -> io::Result<()> {
    let backend = DomBackend::new()?;
    let terminal = Terminal::new(backend)?;

    let state = Rc::new(App::default());

    let event_state = Rc::clone(&state);
    terminal.on_key_event(move |key_event| {
        event_state.handle_events(key_event);
    });

    let render_state = Rc::clone(&state);
    terminal.draw_web(move |frame| {
        render_state.render(frame);
    });

    Ok(())
}

#[derive(Default)]
struct App {
    counter: RefCell<u8>,
    // EffectManager requires a generic parameter - use () if you don't need custom state
    effects: RefCell<EffectManager<()>>,
}

impl App {
    fn render(&self, frame: &mut Frame) {
        let counter = self.counter.borrow();
        let block = Block::bordered()
            .title("rust-effects")
            .title_alignment(Alignment::Center)
            .border_type(BorderType::Rounded);

        let text = format!(
            "This is a Ratzilla template.\n\
             Press left and right to increment and decrement the counter respectively.\n\
             Press 'f' to trigger fire effect.\n\
             Counter: {counter}",
        );

        let paragraph = Paragraph::new(text)
            .block(block)
            .fg(Color::White)
            .bg(Color::Black)
            .centered();

        frame.render_widget(paragraph, frame.area());

        // Get area BEFORE the mutable borrow
        let area = frame.area();

        // Process effects
        let mut effects = self.effects.borrow_mut();
        effects.process_effects(
            Duration::from_millis(16).into(), // Convert to tachyonfx::Duration
            frame.buffer_mut(),
            area,
        );
    }

    fn handle_events(&self, key_event: KeyEvent) {
        let mut counter = self.counter.borrow_mut();
        match key_event.code {
            KeyCode::Left => *counter = counter.saturating_sub(1),
            KeyCode::Right => *counter = counter.saturating_add(1),
            KeyCode::Char('f') => {
                drop(counter); // Release borrow before triggering effect
                self.trigger_fire_effect();
            }
            _ => {}
        }
    }

    fn trigger_fire_effect(&self) {
        use ratatui::layout::Rect;

        let content_area = Rect::new(12, 7, 80, 17);
        let screen_bg = Color::from_u32(0x1D2021);
        let content_bg = Color::from_u32(0x32302F);

        let style = Style::default().fg(content_bg).bg(screen_bg);

        let boot_timer = (300, Interpolation::CircIn);
        let timer = (900, Interpolation::QuadIn);

        // Fire effect using the example structure
        let fire_effect = fx::prolong_start(
            300,
            fx::sequence(&[
                // Startup animation
                fx::fade_from(screen_bg, screen_bg, boot_timer),
                // Main fire animation
                fx::parallel(&[
                    fx::fade_from(screen_bg, screen_bg, 300),
                    fx::fade_from(screen_bg, screen_bg, timer).with_filter(CellFilter::Text),
                ]),
            ]),
        );

        let mut effects = self.effects.borrow_mut();
        effects.add_effect(fire_effect);
    }
}

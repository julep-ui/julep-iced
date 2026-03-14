//! Track keyboard events and handle keyboard navigation.
pub use iced_core::keyboard::*;

use crate::UserInterface;
use crate::core;
use crate::core::widget::operation;

/// Runs a chained operation to completion.
fn run_operation<Message, Theme, Renderer>(
    ui: &mut UserInterface<'_, Message, Theme, Renderer>,
    renderer: &Renderer,
    mut op: Box<dyn operation::Operation>,
) where
    Renderer: core::Renderer,
{
    loop {
        ui.operate(renderer, op.as_mut());

        match op.finish() {
            operation::Outcome::Chain(next) => {
                op = next;
            }
            _ => break,
        }
    }
}

/// Moves focus and scrolls the newly focused widget into view.
fn focus_and_scroll<Message, Theme, Renderer>(
    shift: bool,
    ui: &mut UserInterface<'_, Message, Theme, Renderer>,
    renderer: &Renderer,
) where
    Renderer: core::Renderer,
{
    let op: Box<dyn operation::Operation> = if shift {
        Box::new(operation::focusable::focus_previous::<()>())
    } else {
        Box::new(operation::focusable::focus_next::<()>())
    };

    run_operation(ui, renderer, op);

    run_operation(
        ui,
        renderer,
        Box::new(operation::focusable::scroll_focused_into_view::<()>()),
    );
}

/// Handle Ctrl+Tab / Ctrl+Shift+Tab for unconditional focus navigation.
///
/// This always moves focus regardless of whether any widget captured
/// the event. It is the emergency exit from any focus trap.
///
/// Returns `true` if the event was consumed.
pub fn handle_ctrl_tab<Message, Theme, Renderer>(
    event: &core::Event,
    ui: &mut UserInterface<'_, Message, Theme, Renderer>,
    renderer: &Renderer,
) -> bool
where
    Renderer: core::Renderer,
{
    if let core::Event::Keyboard(core::keyboard::Event::KeyPressed {
        key: core::keyboard::Key::Named(core::keyboard::key::Named::Tab),
        modifiers,
        ..
    }) = event
        && modifiers.control()
    {
        focus_and_scroll(modifiers.shift(), ui, renderer);
        return true;
    }

    false
}

/// Handle uncaptured Tab / Shift+Tab for focus navigation.
///
/// Moves focus to the next (or previous) focusable widget and scrolls
/// it into view. Only runs when the event was not captured by any widget.
///
/// Returns `true` if the event was consumed.
pub fn handle_tab<Message, Theme, Renderer>(
    event: &core::Event,
    status: core::event::Status,
    ui: &mut UserInterface<'_, Message, Theme, Renderer>,
    renderer: &Renderer,
) -> bool
where
    Renderer: core::Renderer,
{
    if status != core::event::Status::Ignored {
        return false;
    }

    if let core::Event::Keyboard(core::keyboard::Event::KeyPressed {
        key: core::keyboard::Key::Named(core::keyboard::key::Named::Tab),
        modifiers,
        ..
    }) = event
    {
        focus_and_scroll(modifiers.shift(), ui, renderer);
        return true;
    }

    false
}

/// Handle uncaptured scroll keys for focused ancestor scrolling.
///
/// Maps keyboard scroll keys (Page Up/Down, arrows, Home/End, and
/// their Shift variants for horizontal scrolling) to scroll actions
/// on the focused widget's nearest scrollable ancestor.
///
/// Returns `true` if the event was consumed.
pub fn handle_scroll_keys<Message, Theme, Renderer>(
    event: &core::Event,
    status: core::event::Status,
    ui: &mut UserInterface<'_, Message, Theme, Renderer>,
    renderer: &Renderer,
) -> bool
where
    Renderer: core::Renderer,
{
    if status != core::event::Status::Ignored {
        return false;
    }

    if let core::Event::Keyboard(core::keyboard::Event::KeyPressed {
        key: core::keyboard::Key::Named(named),
        modifiers,
        ..
    }) = event
    {
        use operation::focusable::ScrollAction;

        let action = match named {
            core::keyboard::key::Named::PageDown if modifiers.shift() => {
                Some(ScrollAction::PageRight)
            }
            core::keyboard::key::Named::PageDown => Some(ScrollAction::PageDown),
            core::keyboard::key::Named::PageUp if modifiers.shift() => Some(ScrollAction::PageLeft),
            core::keyboard::key::Named::PageUp => Some(ScrollAction::PageUp),
            core::keyboard::key::Named::ArrowDown if modifiers.shift() => {
                Some(ScrollAction::LineRight)
            }
            core::keyboard::key::Named::ArrowDown => Some(ScrollAction::LineDown),
            core::keyboard::key::Named::ArrowUp if modifiers.shift() => {
                Some(ScrollAction::LineLeft)
            }
            core::keyboard::key::Named::ArrowUp => Some(ScrollAction::LineUp),
            core::keyboard::key::Named::ArrowRight => Some(ScrollAction::LineRight),
            core::keyboard::key::Named::ArrowLeft => Some(ScrollAction::LineLeft),
            core::keyboard::key::Named::Home if modifiers.shift() => Some(ScrollAction::ShiftHome),
            core::keyboard::key::Named::Home => Some(ScrollAction::Home),
            core::keyboard::key::Named::End if modifiers.shift() => Some(ScrollAction::ShiftEnd),
            core::keyboard::key::Named::End => Some(ScrollAction::End),
            _ => None,
        };

        if let Some(action) = action {
            run_operation(
                ui,
                renderer,
                Box::new(operation::focusable::scroll_focused_ancestor::<()>(action)),
            );
            return true;
        }
    }

    false
}

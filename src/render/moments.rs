//! What happens in play beside the thing it concerns: the release prompt
//! rides by the paddle, a caught power's name rises from it, and news
//! (a save failure, an extra life) appears in the band, never over the
//! field.
use super::*;
use crate::ui::Prompt;
use ark::{NOTICE_TICKS, field::PADDLE_Y};

/// How a held ball is released, beside the paddle: right of it, or left
/// when the right wall is near.
pub(super) fn release(v: &Scene, game: &Game) {
    if !game.balls().iter().any(|b| b.active && b.held) {
        return;
    }
    let words = (TextId::ActionRelease, Role::Caption.into());
    let w = chips::prompt_width(v, Prompt::Serve, words, sheet::CHIP_SMALL);
    let paddle = game.paddle();
    let right = paddle.x + paddle.width / 2.0 + S16;
    let x = if right + w <= RIGHT - S16 {
        right
    } else {
        paddle.x - paddle.width / 2.0 - S16 - w
    };
    let mid = PADDLE_Y + PADDLE_HEIGHT / 2.0;
    let baseline = v.snap(mid + v.cap(Role::Caption) / 2.0);
    chips::prompt(
        v,
        Prompt::Serve,
        words,
        (v.snap(x), baseline),
        sheet::CHIP_SMALL,
        CYAN,
    );
}

/// A caught power's name rises 10 units above the paddle in its hue,
/// fading out over a second.
pub(super) fn power(v: &Scene, game: &Game) {
    let effects = game.effects();
    let Some(power) = effects.notice.filter(|_| effects.notice_ticks > 0) else {
        return;
    };
    let elapsed = (NOTICE_TICKS - effects.notice_ticks) as f32 * DT;
    if elapsed >= 1.0 {
        return;
    }
    let y = v.snap(PADDLE_Y - 24.0 - 10.0 * elapsed);
    let colour = opacity(power_color(power), 1.0 - elapsed);
    let slot = Slot::centered(game.paddle().x, 360.0, y);
    v.say(TextId::PowerName(power), &[], Role::Label, slot, colour);
}

/// News in the band's centre while it lasts: a stroked warning sign and
/// one ink caption.
pub(super) fn status(v: &Scene, mid: f32, id: TextId) {
    let room = 420.0;
    let text = v.width_of(id, &[], Role::Caption).min(room);
    let w = 16.0 + 10.0 + text;
    let x = v.snap(WIDTH / 2.0 - w / 2.0);
    let t = v.thick(1.4);
    let (a, b, c) = (
        V2::new(x + 8.0, mid - 6.5),
        V2::new(x + 15.0, mid + 6.0),
        V2::new(x + 1.0, mid + 6.0),
    );
    for (from, to) in [(a, b), (b, c), (c, a)] {
        v.line(from, to, t, DIM);
    }
    v.line(
        V2::new(x + 8.0, mid - 2.0),
        V2::new(x + 8.0, mid + 1.6),
        t,
        DIM,
    );
    v.circle(V2::new(x + 8.0, mid + 3.6), 0.9, DIM);
    let baseline = v.snap(mid + v.cap(Role::Caption) / 2.0);
    v.say(
        id,
        &[],
        Role::Caption,
        Slot::left(x + 26.0, room, baseline),
        INK,
    );
}

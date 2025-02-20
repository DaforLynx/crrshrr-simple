use assets::fonts;
// use nih_plug::prelude::{util, Editor};
use nih_plug::prelude::Editor;
use nih_plug_vizia::assets::fonts::*;
use nih_plug_vizia::vizia::prelude::*;
use nih_plug_vizia::widgets::*;
use nih_plug_vizia::*;
use std::sync::Arc;

use crate::CrrshrrParams;

#[derive(Lens)]
struct Data {
    params: Arc<CrrshrrParams>,
}

impl Model for Data {}

pub(crate) fn default_state() -> Arc<ViziaState> {
    ViziaState::new(|| (200, 350))
}

pub(crate) fn create(
    params: Arc<CrrshrrParams>,
    editor_state: Arc<ViziaState>,
) -> Option<Box<dyn Editor>> {
    create_vizia_editor(editor_state, ViziaTheming::Custom, move |cx, _| {
        assets::register_noto_sans_light(cx);
        assets::register_noto_sans_thin(cx);

        Data {
            params: params.clone(),
        }
        .build(cx);

        ResizeHandle::new(cx);

        VStack::new(cx, |cx| {
            Label::new(cx, "CRRSHRR")
                .font_size(30.0)
                .height(Pixels(30.0))
                .color(Color::white())
                .top(Pixels(4.0));
            Label::new(cx, "SIMPLE")
                .font_size(15.0)
                .height(Pixels(15.0))
                .color(Color::grey())
                .top(Pixels(2.0))
                .child_top(Stretch(1.0))
                .child_bottom(Pixels(0.0));

            Label::new(cx, "BITS").color(Color::white());
            ParamSlider::new(cx, Data::params, |params| &params.bits)
                .background_color(Color::white());

            Label::new(cx, "RATE")
                .color(Color::white())
                .top(Pixels(6.0));
            ParamSlider::new(cx, Data::params, |params| &params.rate)
                .background_color(Color::white());

            Label::new(cx, "CRUNCHY")
                .color(Color::white())
                .top(Pixels(6.0));
            ParamButton::new(cx, Data::params, |params| &params.crunchy)
                .with_label("")
                .outline_color(Color::white())
                .outline_width(Pixels(2.0))
                .top(Pixels(2.0))
                .width(Pixels(70.0));

            Label::new(cx, "NOISE")
                .color(Color::white())
                .top(Pixels(6.0));
            ParamSlider::new(cx, Data::params, |params| &params.noise)
                .background_color(Color::white());

            Label::new(cx, "GATE")
                .color(Color::white())
                .top(Pixels(6.0));
            ParamButton::new(cx, Data::params, |params| &params.noise_gate)
                .with_label("")
                .outline_color(Color::white())
                .outline_width(Pixels(2.0))
                .top(Pixels(2.0))
                .width(Pixels(70.0));
        })
        .row_between(Pixels(0.0))
        .child_left(Stretch(1.0))
        .child_right(Stretch(1.0))
        .background_color(Color::rgb(0x00, 0x00, 0x00));
    })
}

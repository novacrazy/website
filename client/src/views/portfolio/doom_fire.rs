use std::time::Duration;

use wasm_bindgen::prelude::*;

use yew::prelude::*;
use yew::services::{
    interval::{IntervalService, IntervalTask},
    render::{RenderService, RenderTask},
};
use yewtil::{NeqAssign, Pure, PureComponent};

use rand::{Rng, SeedableRng};
use rand_xoshiro::Xoshiro128Plus;

use crate::geometry::Vector2;

pub static PALETTE: [u32; 37] = [
    0x070707, 0x1F0707, 0x2F0F07, 0x470F07, 0x571707, 0x671F07, 0x771F07, 0x8F2707, 0x9F2F07, 0xAF3F07, 0xBF4707, 0xC74707, 0xDF4F07, 0xDF5707,
    0xDF5707, 0xD75F07, 0xD75F07, 0xD7670F, 0xCF6F0F, 0xCF770F, 0xCF7F0F, 0xCF8717, 0xC78717, 0xC78F17, 0xC7971F, 0xBF9F1F, 0xBF9F1F, 0xBFA727,
    0xBFA727, 0xBFAF2F, 0xB7AF2F, 0xB7B72F, 0xB7B737, 0xCFCF6F, 0xDFDF9F, 0xEFEFC7, 0xFFFFFF,
];

pub struct DoomFire {
    pub link: ComponentLink<Self>,
    pub props: DoomFireProps,

    pub pixels: Vec<usize>,

    pub canvas: NodeRef,

    pub tick: IntervalTask,
    pub draw: RenderTask,

    pub rng: Xoshiro128Plus,

    pub is_drawing: bool,
    pub pos: Vector2,
    pub last_pos: Vector2,
}

#[derive(Clone, Copy, Properties, PartialEq)]
pub struct DoomFireProps {
    pub width: usize,
    pub height: usize,
}

pub enum DoomFireMsg {
    MouseDown(web_sys::MouseEvent),
    MouseUp,
    MouseMove(web_sys::MouseEvent),
    Tick,
    Draw,
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = Date, js_name = now)]
    fn now() -> f64;
}

impl Component for DoomFire {
    type Message = DoomFireMsg;
    type Properties = DoomFireProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut pixels = vec![0; props.width * props.height];

        // Set bottom line to white
        for i in 0..props.width {
            pixels[(props.height - 1) * props.width + i] = 36;
        }

        let tick = IntervalService::new().spawn(Duration::from_secs_f32(1.0 / 30.0), link.callback(|_| DoomFireMsg::Tick));
        let draw = RenderService::new().request_animation_frame(link.callback(|_| DoomFireMsg::Draw));

        DoomFire {
            link,
            props,
            pixels,
            canvas: NodeRef::default(),
            tick,
            draw,
            rng: Xoshiro128Plus::seed_from_u64(now() as u64),
            is_drawing: false,
            pos: Vector2::ZERO,
            last_pos: Vector2::ZERO,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let get_mouse_pos = |event: web_sys::MouseEvent| {
            let canvas = self.canvas.cast::<web_sys::HtmlCanvasElement>().unwrap();
            let element: &web_sys::Element = canvas.as_ref();

            let rect = element.get_bounding_client_rect();

            Vector2::new(
                (event.client_x() as f32 - rect.x() as f32).min(self.props.width as f32),
                (event.client_y() as f32 - rect.y() as f32).min(self.props.height as f32),
            )
        };

        match msg {
            DoomFireMsg::MouseDown(event) => {
                self.pos = get_mouse_pos(event);
                self.last_pos = self.pos; // avoids connecting from last drawn position to here

                self.is_drawing = true;
            }
            DoomFireMsg::MouseUp => self.is_drawing = false,
            DoomFireMsg::MouseMove(event) if self.is_drawing => {
                self.pos = get_mouse_pos(event);
            }
            DoomFireMsg::Tick => unsafe {
                if self.is_drawing {
                    const FALLOFF: f32 = 1.0 / 8.0;
                    const SIZE: f32 = 15.0;
                    let norm = 36f32.powf(FALLOFF);

                    // Use a simple 2D line SDF to splat fire
                    let a = self.last_pos;
                    let b = self.pos;

                    let ba = b - a;
                    let ba_dot_ba = ba.dot(ba);

                    for y in 0..self.props.height {
                        for x in 0..self.props.width {
                            let pa = Vector2::new(x as f32, y as f32) - a;
                            let h = (pa.dot(ba) / ba_dot_ba).min(1.0).max(0.0);
                            let d = (pa - ba * h).norm();

                            if d < SIZE {
                                let idx = y * self.props.width + x;

                                let fire = ((36.0 - d).powf(FALLOFF) / norm * 35.0) as usize + 1;

                                self.pixels[idx] = self.pixels[idx].max(fire);
                            }
                        }
                    }

                    self.last_pos = self.pos;
                }

                for x in 0..self.props.width {
                    for y in 1..self.props.height {
                        let idx = y * self.props.width + x;

                        let pixel = *self.pixels.get_unchecked(idx);

                        if pixel == 0 {
                            *self.pixels.get_unchecked_mut(idx - self.props.width) = 0;
                        } else {
                            let rnd_idx = self.rng.gen_range(0, 4);
                            let dst = idx - rnd_idx + 1;
                            *self.pixels.get_unchecked_mut(dst - self.props.width) = pixel - (rnd_idx & 1);
                        }
                    }
                }

                // queue up draw frame only after anything has changed
                self.draw = RenderService::new().request_animation_frame(self.link.callback(|_| DoomFireMsg::Draw));
            },
            DoomFireMsg::Draw => unsafe {
                use wasm_bindgen::{Clamped, JsCast};

                let canvas = self.canvas.cast::<web_sys::HtmlCanvasElement>().unwrap();

                if let Ok(Some(ctx)) = canvas.get_context("2d") {
                    let ctx = ctx.dyn_into::<web_sys::CanvasRenderingContext2d>().unwrap();

                    let width = canvas.width() as usize;
                    let height = canvas.height() as usize;

                    let color = ctx.get_image_data(0.0, 0.0, width as f64, height as f64).unwrap();
                    let mut data = color.data();

                    for y in 0..height {
                        for x in 0..width {
                            let color = *self.pixels.get_unchecked(y * width + x);
                            let rgb = *PALETTE.get_unchecked(color);
                            let r = (rgb >> 16) as u8;
                            let g = (rgb >> 8) as u8;
                            let b = rgb as u8;

                            let offset = (width * y + x) * 4;

                            *data.0.get_unchecked_mut(offset + 0) = r;
                            *data.0.get_unchecked_mut(offset + 1) = g;
                            *data.0.get_unchecked_mut(offset + 2) = b;
                            // simple curve for alpha
                            *data.0.get_unchecked_mut(offset + 3) = ((color as f32).sqrt() / 6.0 * 255.0) as u8;
                        }
                    }

                    ctx.put_image_data(
                        &web_sys::ImageData::new_with_u8_clamped_array_and_sh(Clamped(&mut data.0), width as u32, height as u32).unwrap(),
                        0.0,
                        0.0,
                    )
                    .unwrap();
                }
            },
            _ => {}
        }

        false
    }

    fn change(&mut self, new: Self::Properties) -> ShouldRender {
        self.props.neq_assign(new)
    }

    fn view(&self) -> Html {
        html! {
            <canvas
                ref={self.canvas.clone()}
                width={self.props.width}
                height={self.props.height}
                style="border: 1px solid black;"
                onmousedown={self.link.callback(DoomFireMsg::MouseDown)}
                onmouseup={self.link.callback(|_| DoomFireMsg::MouseUp)}
                onmousemove={self.link.callback(DoomFireMsg::MouseMove)}
            />
        }
    }
}

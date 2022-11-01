/* Wolkenwelten - Copyright (C) 2022 - Benjamin Vincent Schulenburg
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */
extern crate wolkenwelten_client;
extern crate wolkenwelten_game;
extern crate wolkenwelten_scripting;

use winit::event::{
    DeviceEvent, ElementState, Event, KeyboardInput, MouseScrollDelta, VirtualKeyCode, WindowEvent,
};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{CursorGrabMode, Fullscreen, Window, WindowBuilder};

use winit::dpi::PhysicalPosition;
use wolkenwelten_client::{
    input_tick, prepare_frame, render_frame, ClientState, InputEvent, Key, RENDER_DISTANCE,
    VIEW_STEPS,
};
use wolkenwelten_scripting::Runtime;
use wolkenwelten_sound::SfxList;

use wolkenwelten_game::{GameEvent, GameState};

pub struct AppState {
    pub game_state: GameState,
    pub render_state: ClientState,
    pub event_loop: EventLoop<()>,
    pub runtime: Runtime,
    pub sfx: SfxList,
}

pub fn grab_cursor(window: &Window) {
    window.set_cursor_visible(false);
    let e = window.set_cursor_grab(CursorGrabMode::Locked);
    if e.is_ok() {
        return;
    }
    let e = window.set_cursor_grab(CursorGrabMode::Confined);
    if let Err(e) = e {
        println!("Error when grabbing Cursor: {:?}", e);
    }
}

pub fn ungrab_cursor(window: &Window) {
    window.set_cursor_visible(true);
    let _ = window.set_cursor_grab(CursorGrabMode::None);
}

pub fn init() -> (EventLoop<()>, glium::Display) {
    let title = format!("WolkenWelten - {}", env!("CARGO_PKG_VERSION"));
    let event_loop = EventLoop::new();
    let wb = WindowBuilder::new()
        .with_title(title)
        .with_decorations(false)
        .with_maximized(true);

    let cb = glium::glutin::ContextBuilder::new()
        .with_vsync(true)
        .with_double_buffer(Some(true));

    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    {
        let ctx = display.gl_window();
        let window = ctx.window();
        window.focus_window();
        let fs = Fullscreen::Borderless(window.current_monitor());
        window.set_fullscreen(Some(fs));
        grab_cursor(window);
    }

    (event_loop, display)
}

pub fn run_event_loop(state: AppState) {
    let mut render_state = state.render_state;
    let mut game_state = state.game_state;
    let event_loop = state.event_loop;
    let mut runtime = state.runtime;

    event_loop.run(move |event, _, control_flow| match event {
        Event::LoopDestroyed => (),
        Event::DeviceEvent {
            event: DeviceEvent::MouseMotion { delta },
            ..
        } => {
            game_state.player.rot.x += delta.0 as f32 * 0.05;
            game_state.player.rot.y += delta.1 as f32 * 0.05;

            #[cfg(not(target_os = "macos"))]
            {
                let (x, y) = render_state.window_size();
                let center = PhysicalPosition::new(x / 2, y / 2);

                let ctx = render_state.display.gl_window();
                let window = ctx.window();
                let _ = window.set_cursor_position(center);
            }
        }

        Event::WindowEvent {
            event: WindowEvent::MouseInput { button, state, .. },
            ..
        } => {
            render_state
                .input
                .set_mouse_button(button, state == ElementState::Pressed);
        }

        Event::WindowEvent {
            event: WindowEvent::MouseWheel { delta, .. },
            ..
        } => match delta {
            MouseScrollDelta::LineDelta(_, y) => {
                game_state.player.switch_block_selection(y.round() as i32)
            }
            MouseScrollDelta::PixelDelta(PhysicalPosition { x: _x, y }) => {
                game_state.player.switch_block_selection(y.round() as i32)
            }
        },

        Event::WindowEvent {
            event: WindowEvent::Focused(b),
            ..
        } => {
            if b {
                grab_cursor(render_state.display.gl_window().window());
            } else {
                ungrab_cursor(render_state.display.gl_window().window());
            }
        }

        Event::DeviceEvent {
            event: DeviceEvent::Key(input),
            ..
        }
        | Event::WindowEvent {
            event: WindowEvent::KeyboardInput { input, .. },
            ..
        } => match input {
            KeyboardInput {
                virtual_keycode: Some(VirtualKeyCode::Escape),
                ..
            } => *control_flow = ControlFlow::Exit,

            KeyboardInput {
                state: ElementState::Pressed,
                virtual_keycode: Some(VirtualKeyCode::T),
                ..
            } => runtime.eval("print('You pressed the T key!');"),

            KeyboardInput {
                state: ElementState::Pressed,
                virtual_keycode: Some(VirtualKeyCode::N),
                ..
            } => game_state.player.set_no_clip(true),
            KeyboardInput {
                state: ElementState::Pressed,
                virtual_keycode: Some(VirtualKeyCode::M),
                ..
            } => game_state.player.set_no_clip(false),

            KeyboardInput {
                state: ElementState::Pressed,
                virtual_keycode: Some(VirtualKeyCode::O),
                ..
            } => render_state.set_wireframe(true),
            KeyboardInput {
                state: ElementState::Pressed,
                virtual_keycode: Some(VirtualKeyCode::P),
                ..
            } => render_state.set_wireframe(false),

            KeyboardInput {
                state,
                virtual_keycode: Some(VirtualKeyCode::W),
                ..
            } => render_state
                .input
                .key_up_down(Key::Up, state == ElementState::Pressed),

            KeyboardInput {
                state,
                virtual_keycode: Some(VirtualKeyCode::S),
                ..
            } => render_state
                .input
                .key_up_down(Key::Down, state == ElementState::Pressed),

            KeyboardInput {
                state,
                virtual_keycode: Some(VirtualKeyCode::A),
                ..
            } => render_state
                .input
                .key_up_down(Key::Left, state == ElementState::Pressed),

            KeyboardInput {
                state,
                virtual_keycode: Some(VirtualKeyCode::D),
                ..
            } => render_state
                .input
                .key_up_down(Key::Right, state == ElementState::Pressed),

            KeyboardInput {
                state,
                virtual_keycode: Some(VirtualKeyCode::Space),
                ..
            } => render_state
                .input
                .key_up_down(Key::Jump, state == ElementState::Pressed),

            KeyboardInput {
                state,
                virtual_keycode: Some(VirtualKeyCode::C),
                ..
            } => render_state
                .input
                .key_up_down(Key::Crouch, state == ElementState::Pressed),

            KeyboardInput {
                state,
                virtual_keycode: Some(VirtualKeyCode::LShift),
                ..
            } => render_state
                .input
                .key_up_down(Key::Sprint, state == ElementState::Pressed),

            _ => (),
        },
        Event::WindowEvent {
            event: WindowEvent::Resized(physical_size),
            ..
        } => {
            render_state.display.gl_window().resize(physical_size);
            render_state.set_window_size((physical_size.width, physical_size.height));
        }
        Event::RedrawRequested(_) => {
            runtime.tick(game_state.get_millis());
            let mut frame = render_state.display.draw();
            prepare_frame(&mut render_state, &game_state);
            render_frame(&mut frame, &render_state, &game_state);
            frame.finish().unwrap();
            //windowed_context.swap_buffers().unwrap();
        }
        Event::MainEventsCleared => {
            let events = input_tick(&mut game_state, &render_state);
            events.iter().for_each(|e| match e {
                InputEvent::PlayerJump() => state.sfx.play(&state.sfx.jump, 0.2),
                InputEvent::PlayerShoot() => state.sfx.play(&state.sfx.hook_fire, 0.4),
                InputEvent::PlayerBlockMine(_) => state.sfx.play(&state.sfx.tock, 0.3),
                InputEvent::PlayerBlockPlace(_) => state.sfx.play(&state.sfx.pock, 0.3),
            });
            render_state.input.mouse_flush();

            let render_distance = RENDER_DISTANCE * RENDER_DISTANCE;
            let events = game_state.tick(render_distance);
            events.iter().for_each(|e| match e {
                GameEvent::CharacterStomp(_pos) => state.sfx.play(&state.sfx.stomp, 0.3),
                GameEvent::EntityCollision(pos) => {
                    game_state.world.add_explosion(pos, 5.0);
                    state.sfx.play(&state.sfx.bomb, 0.3);
                }
            });
            game_state.prepare_world(VIEW_STEPS, render_distance);
            render_state.display.gl_window().window().request_redraw();
        }
        _ => {}
    });
}

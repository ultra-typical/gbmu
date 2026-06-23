use super::*;
use crate::gui::KeyInput;
use crate::ppu::colors_palette::Color;

// ---------------------------------------------------------------------------
// Shared helpers
// ---------------------------------------------------------------------------

fn setup() -> (Box<dyn GameCT>, Box<dyn InterfaceCT>) {
    create_communication_tools()
}

// ---------------------------------------------------------------------------
// create_communication_tools
// ---------------------------------------------------------------------------

#[test]
fn create_communication_tools_returns_a_working_pair() {
    let (mut gct, ict) = setup();
    let mut input = KeyInput::default();
    assert!(gct.update_input(&mut input).is_ok());
    assert!(ict.get_fps().is_ok());
}

// ---------------------------------------------------------------------------
// GameCT::update_input
// ---------------------------------------------------------------------------

mod update_input {
    use super::*;

    #[test]
    fn without_prior_send_input_is_unchanged() {
        let (mut gct, _ict) = setup();
        let mut input = KeyInput::default();
        if let Err(_) = gct.update_input(&mut input) { panic!("update in error") }
        assert_eq!(input, KeyInput::default());
    }

    #[test]
    fn after_interface_send_input_is_updated() {
        let (mut gct, ict) = setup();
        let mut input = KeyInput::default();
        input.a_pushed = true;
        if let Err(_) = ict.send_input(input) { panic!("update in error") }
        let mut received = KeyInput::default();
        if let Err(_) = gct.update_input(&mut received) { panic!("update in error") }
        assert_eq!(input, received);
    }

    #[test]
    fn returns_ok_when_no_change_available() {
        let (mut gct, _ict) = setup();
        let mut input = KeyInput::default();
        assert_eq!(Ok(()), gct.update_input(&mut input));
    }
}

// ---------------------------------------------------------------------------
// GameCT::put_pixel_to_frame
// ---------------------------------------------------------------------------

mod put_pixel_to_frame {
    use super::*;

    #[test]
    fn writes_color_at_the_given_offset() {
        let (mut gct, mut ict) = setup();
        let c = Color::LightGray;
        gct.put_pixel_to_frame(0, c);
        let mut image = [0; FRAME_SIZE_IN_U8];
        ict.get_new_image(&mut image).unwrap();
        assert_eq!(&image[0..3], c.to_rgb());
    }

    #[test]
    fn marks_image_as_changed() {
        let (mut gct, mut ict) = setup();
        let c = Color::LightGray;
        gct.put_pixel_to_frame(0, c);
        let mut image = [0; FRAME_SIZE_IN_U8];
        let Ok(Some(_)) = ict.get_new_image(&mut image) else {
            panic!("image must be set");
        };
    }

    #[test]
    fn two_pixels_at_different_offsets_are_independent() {
        let (mut gct, mut ict) = setup();
        gct.put_pixel_to_frame(0, Color::Black);
        gct.put_pixel_to_frame(1, Color::White);
        let mut buffer = [0; FRAME_SIZE_IN_U8];
        ict.get_new_image(&mut buffer).unwrap();
        assert_eq!(&buffer[0..3], Color::Black.to_rgb());
        assert_eq!(&buffer[3..6], Color::White.to_rgb());
    }

    #[test]
    fn last_valid_offset_does_not_panic() {
        let (mut gct, _ict) = setup();
        let c = Color::LightGray;
        gct.put_pixel_to_frame(FRAME_SIZE - 1, c);
    }
}

// ---------------------------------------------------------------------------
// GameCT::update_fps / InterfaceCT::get_fps
// ---------------------------------------------------------------------------

mod fps {
    use super::*;

    #[test]
    fn initial_value_is_zero() {
        let (_gct, ict) = setup();
        assert_eq!(ict.get_fps(), Ok(0));
    }

    #[test]
    fn update_fps_is_visible_via_get_fps() {
        let (mut gct, ict) = setup();
        gct.update_fps(60).unwrap();
        assert_eq!(ict.get_fps(), Ok(60));
    }

    #[test]
    fn last_written_value_overwrites_the_previous_one() {
        let (mut gct, ict) = setup();
        gct.update_fps(30).unwrap();
        gct.update_fps(60).unwrap();
        assert_eq!(ict.get_fps(), Ok(60));
    }
}

// ---------------------------------------------------------------------------
// GameCT::send_cpu_state / InterfaceCT::get_cpu_state
// ---------------------------------------------------------------------------

mod cpu_state {
    use super::*;

    #[test]
    fn only_changes_on_update_if_has_changed() {
        let (_, mut ict) = setup();
        let mut cpu_state = CpuState::default();
        cpu_state.a = 1;
        cpu_state.hl = 2;
        let temp = cpu_state.clone();

        let _ = ict.get_cpu_state(&mut cpu_state);

        assert_eq!(cpu_state, temp);
    }

    #[test]
    fn after_send_cpu_state_get_returns_the_sent_state() {
        let (mut gct, mut ict) = setup();
        let mut cpu_state = CpuState::default();
        cpu_state.a = 1;
        cpu_state.hl = 2;
        gct.send_cpu_state(&cpu_state);
        let mut temp = CpuState::default();
        let _ = ict.get_cpu_state(&mut temp);

        assert_eq!(cpu_state, temp);
    }
}

// ---------------------------------------------------------------------------
// GameCT::send_next_instructions / InterfaceCT::get_next_instructions
// ---------------------------------------------------------------------------

mod instructions {
    use super::*;

    #[test]
    fn without_send_the_list_is_unchanged() {
        let (_gct, mut ict) = setup();
        let mut list = InstructionList(vec![(0xAF, "test2".to_string()), (0x01, "test2".to_string())]);
        ict.get_next_instructions(&mut list).unwrap();
        assert_eq!(&*list, &[(0xAF, "test2".to_string()), (0x01, "test2".to_string())]);
    }

    #[test]
    fn after_send_the_list_contains_the_sent_opcodes() {
        let (mut gct, mut ict) = setup();
        gct.send_next_instructions(InstructionList(vec![(0xAF, "test2".to_string()), (0x01, "test2".to_string())]));
        let mut list = InstructionList::default();
        ict.get_next_instructions(&mut list).unwrap();
        assert_eq!(&*list, &[(0xAF, "test2".to_string()), (0x01, "test2".to_string())]);
    }

    #[test]
    fn get_next_instructions_clears_the_list_before_filling_it() {
        let (mut gct, mut ict) = setup();
        gct.send_next_instructions(InstructionList(vec![(0xAF, "test2".to_string())]));
        let mut list = InstructionList(vec![(0x00, "test2".to_string()), (0x00, "test2".to_string()), (0x00, "test2".to_string())]);
        ict.get_next_instructions(&mut list).unwrap();
        assert_eq!(&*list, &[(0xAF, "test2".to_string())]);
    }
}

// ---------------------------------------------------------------------------
// GameCT::send_watched_adresses / InterfaceCT::get_watched_adresses
// ---------------------------------------------------------------------------

mod watched_adresses {
    use super::*;

    #[test]
    fn without_send_adresses_are_unchanged() {
        let (_gct, mut ict) = setup();
        let mut addresses = WatchedAdresses(vec![(0xFF00, 0xCF)]);
        ict.get_watched_adresses(&mut addresses).unwrap();
        assert_eq!(&*addresses, &[(0xFF00, 0xCF)]);
    }

    #[test]
    fn after_send_adresses_and_values_are_correct() {
        let (mut gct, mut ict) = setup();
        gct.send_watched_adresses(WatchedAdresses(vec![(0xFF00, 0xCF)]));
        let mut addresses = WatchedAdresses::default();
        ict.get_watched_adresses(&mut addresses).unwrap();
        assert_eq!(&*addresses, &[(0xFF00_u16, 0xCF_u8)]);
    }
}

// ---------------------------------------------------------------------------
// GameCT::poll_requests
// ---------------------------------------------------------------------------

mod poll_requests {
    use super::*;

    #[test]
    fn returns_empty_vec_when_no_request_was_sent() {
        let (mut gct, _) = setup();
        assert!(gct.poll_requests().is_empty());
    }

    #[test]
    fn returns_requests_in_fifo_order() {
        let (mut gct, ict) = setup();
        ict.watch_adress(0xFF00).unwrap();
        ict.set_instruction_list_len(5).unwrap();
        let requests = gct.poll_requests();
        assert_eq!(requests.len(), 2);
        assert!(matches!(requests[0], Request::Watch(0xFF00)));
        assert!(matches!(requests[1], Request::SetInstructionListLength(5)));
    }

    #[test]
    fn drains_the_queue_after_call() {
        let (mut gct, ict) = setup();
        ict.watch_adress(0xFF00).unwrap();
        gct.poll_requests();
        assert!(gct.poll_requests().is_empty());
    }

    #[test]
    fn full_capacity_queue_does_not_block() {
        let (mut gct, ict) = setup();
        for _ in 0..50 {
            ict.watch_adress(0xFF00).unwrap();
        }
        let requests = gct.poll_requests();
        assert_eq!(requests.len(), 50);
    }
}

// ---------------------------------------------------------------------------
// InterfaceCT request methods
// (each method must produce the correct Request variant)
// ---------------------------------------------------------------------------

mod interface_requests {
    use super::*;

    #[test]
    fn set_mode_game_produces_request_mode_game() {
        let (mut gct, ict) = setup();
        ict.set_mode(Mode::Game).unwrap();
        let requests = gct.poll_requests();
        assert_eq!(requests.len(), 1);
        assert!(matches!(requests[0], Request::Mode(Mode::Game)));
    }

    #[test]
    fn set_mode_debug_produces_request_mode_debug() {
        let (mut gct, ict) = setup();
        ict.set_mode(Mode::Debug).unwrap();
        let requests = gct.poll_requests();
        assert_eq!(requests.len(), 1);
        assert!(matches!(requests[0], Request::Mode(Mode::Debug)));
    }

    #[test]
    fn ask_fps_counter_produces_request_fps_true() {
        let (mut gct, ict) = setup();
        ict.ask_fps_counter().unwrap();
        let requests = gct.poll_requests();
        assert_eq!(requests.len(), 1);
        assert!(matches!(requests[0], Request::Fps(true)));
    }

    #[test]
    fn disable_fps_counter_produces_request_fps_false() {
        let (mut gct, ict) = setup();
        ict.disable_fps_counter().unwrap();
        let requests = gct.poll_requests();
        assert_eq!(requests.len(), 1);
        assert!(matches!(requests[0], Request::Fps(false)));
    }

    #[test]
    fn watch_adress_produces_request_watch_with_correct_address() {
        let (mut gct, ict) = setup();
        ict.watch_adress(0xFF40).unwrap();
        let requests = gct.poll_requests();
        assert_eq!(requests.len(), 1);
        assert!(matches!(requests[0], Request::Watch(0xFF40)));
    }

    #[test]
    fn execute_instruction_produces_request_execute_with_correct_bytes() {
        let (mut gct, ict) = setup();
        ict.execute_instruction(vec![0xAF]).unwrap();
        let requests = gct.poll_requests();
        assert_eq!(requests.len(), 1);
        assert!(matches!(&requests[0], Request::Execute(bytes) if bytes == &vec![0xAF]));
    }

    #[test]
    fn execute_next_instructions_produces_request_step_with_correct_count() {
        let (mut gct, ict) = setup();
        ict.execute_next_instructions(3).unwrap();
        let requests = gct.poll_requests();
        assert_eq!(requests.len(), 1);
        assert!(matches!(requests[0], Request::Step(3)));
    }

    #[test]
    fn render_frame_produces_request_render_frame_1() {
        let (mut gct, ict) = setup();
        ict.render_frame().unwrap();
        let requests = gct.poll_requests();
        assert_eq!(requests.len(), 1);
        assert!(matches!(requests[0], Request::RenderFrame(1)));
    }

    #[test]
    fn render_frames_produces_request_render_frame_n() {
        let (mut gct, ict) = setup();
        ict.render_frames(5).unwrap();
        let requests = gct.poll_requests();
        assert_eq!(requests.len(), 1);
        assert!(matches!(requests[0], Request::RenderFrame(5)));
    }

    #[test]
    fn set_instruction_list_len_produces_correct_request() {
        let (mut gct, ict) = setup();
        ict.set_instruction_list_len(10).unwrap();
        let requests = gct.poll_requests();
        assert_eq!(requests.len(), 1);
        assert!(matches!(requests[0], Request::SetInstructionListLength(10)));
    }
}

// ---------------------------------------------------------------------------
// InterfaceCT::get_new_image
// ---------------------------------------------------------------------------

mod get_new_image {
    use super::*;

    #[test]
    fn returns_none_when_no_pixel_has_been_written() {
        let (_, mut ict) = setup();
        let mut buffer = [0; FRAME_SIZE_IN_U8];
        assert_eq!(ict.get_new_image(&mut buffer), Ok(None));
    }

    #[test]
    fn returns_some_after_a_pixel_is_written() {
        let (mut gct, mut ict) = setup();
        gct.put_pixel_to_frame(0, Color::Black);
        let mut buffer = [0; FRAME_SIZE_IN_U8];
        assert_eq!(ict.get_new_image(&mut buffer), Ok(Some(())));
    }

    #[test]
    fn correctly_copies_pixels_to_rgb_buffer() {
        let (mut gct, mut ict) = setup();
        gct.put_pixel_to_frame(0, Color::Black);
        let mut buffer = [0; FRAME_SIZE_IN_U8];
        ict.get_new_image(&mut buffer).unwrap();
        assert_eq!(&buffer[0..3], Color::Black.to_rgb());
    }

    #[test]
    fn returns_none_on_second_call_without_new_write() {
        let (mut gct, mut ict) = setup();
        gct.put_pixel_to_frame(0, Color::Black);
        let mut buffer = [0; FRAME_SIZE_IN_U8];
        ict.get_new_image(&mut buffer).unwrap();
        assert_eq!(ict.get_new_image(&mut buffer), Ok(None));
    }
}

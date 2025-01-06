#[cfg(not(feature = "std"))]
extern crate alloc;

#[cfg(not(feature = "std"))]
use alloc::boxed::Box;

use super::{feedback::Feedback, optional_data::OptionalData, output_device::OutputDevice};

const INITIAL_BYTES: [u8; 2] = [0x7E, 0xFF];
const LAST_BYTE: u8 = 0xEF;
const NO_DATA: [u8; 2] = [0x00, 0x00];
const COMMAND_LENGTH: usize = 1;

const NEXT_SONG: u8 = 0x01;
const PREVIOUS_SONG: u8 = 0x02;
const PLAY_WITH_INDEX: u8 = 0x03;
const VOLUME_UP: u8 = 0x04;
const VOLUME_DOWN: u8 = 0x05;
const SET_VOLUME: u8 = 0x06;
const SINGLE_CYCLE_PLAY: u8 = 0x08;
const SELECT_DEVICE: u8 = 0x09;
const SLEEP_MODE: u8 = 0x0A;
const WAKE_UP: u8 = 0x0B;
const RESET: u8 = 0x0C;
const PLAY: u8 = 0x0D;
const PAUSE: u8 = 0x0E;
const PLAY_WITH_FOLDER_AND_FILENAME: u8 = 0x0F;
const STOP_PLAY: u8 = 0x16;
const CYCLE_PLAY_WITH_FOLDER_NAME: u8 = 0x17;
const SET_SINGLE_CYCLE_PLAY: u8 = 0x19;
const SET_DAC: u8 = 0x1A;
const PLAY_WITH_VOLUME: u8 = 0x22;
const PLAY_WITH_FOLDER_NAME: u8 = 0x17;

pub struct Xc3748 {
    output_device: Box<dyn OutputDevice>,
    buffer: [u8; 8],
    feedback: Feedback,
}

impl Xc3748 {
    pub fn new(output_device: Box<dyn OutputDevice>, feedback: Feedback) -> Self {
        Self {
            output_device,
            buffer: [0; 8],
            feedback,
        }
    }

    // Default feedback is off
    fn send_command(&mut self, command: u8, optional_data: OptionalData) {
        // Copy  init byte to buffer
        self.buffer[0..2].copy_from_slice(&INITIAL_BYTES);

        // Copy length of command to buffer
        self.buffer[2] = 0x06;

        // Copy command to buffer
        self.buffer[3] = command;

        // Copy the feedback to buffer
        // 3 to account for the initial bytes and the length of the command
        self.buffer[3 + COMMAND_LENGTH] = self.feedback as u8;

        // Copy optional data to buffer
        match optional_data {
            OptionalData::Data(data) => {
                self.buffer[4 + COMMAND_LENGTH..6 + COMMAND_LENGTH].copy_from_slice(&data);
            }
            OptionalData::None => {
                // Copy no data to buffer
                self.buffer[4 + COMMAND_LENGTH..6 + COMMAND_LENGTH].copy_from_slice(&NO_DATA);
            }
        }

        // Copy the last byte to buffer taking into account the optional data
        // 3 to account for the initial bytes and the length of the command and the feedback
        self.buffer[3 + COMMAND_LENGTH + 1 + 2] = LAST_BYTE;

        // Print the buffer in Decimal
        // println!("{:?}", self.buffer);

        // Print the buffer in Hex
        // if cfg!(debug_assertions) {
        // }

        // println!("{:x?}", self.buffer);

        // Write the buffer to the port
        self.output_device.write_data(&self.buffer);
    }

    pub fn next_song(&mut self) {
        self.send_command(NEXT_SONG, OptionalData::None);
    }

    pub fn previous_song(&mut self) {
        self.send_command(PREVIOUS_SONG, OptionalData::None);
    }

    pub fn play_with_index(&mut self, index: i16) {
        self.send_command(PLAY_WITH_INDEX, OptionalData::Data(index.to_be_bytes()));
    }

    pub fn volume_up(&mut self) {
        self.send_command(VOLUME_UP, OptionalData::None);
    }

    pub fn volume_down(&mut self) {
        self.send_command(VOLUME_DOWN, OptionalData::None);
    }

    pub fn set_volume(&mut self, volume: i16) {
        self.send_command(SET_VOLUME, OptionalData::Data(volume.to_be_bytes()));
    }

    pub fn single_cycle_play(&mut self, index: i16) {
        self.send_command(SINGLE_CYCLE_PLAY, OptionalData::Data(index.to_be_bytes()));
    }

    pub fn select_device(&mut self, device: i16) {
        self.send_command(SELECT_DEVICE, OptionalData::Data(device.to_be_bytes()));
    }

    pub fn sleep_mode(&mut self) {
        self.send_command(SLEEP_MODE, OptionalData::None);
    }

    pub fn wake_up(&mut self) {
        self.send_command(WAKE_UP, OptionalData::None);
    }

    pub fn reset(&mut self) {
        self.send_command(RESET, OptionalData::None);
    }

    pub fn play(&mut self) {
        self.send_command(PLAY, OptionalData::None);
    }

    pub fn pause(&mut self) {
        self.send_command(PAUSE, OptionalData::None);
    }

    pub fn play_with_folder_and_filename(&mut self, folder: i8, filename: i8) {
        self.send_command(
            PLAY_WITH_FOLDER_AND_FILENAME,
            OptionalData::Data([folder as u8, filename as u8]),
        );
    }

    pub fn stop_play(&mut self) {
        self.send_command(STOP_PLAY, OptionalData::None);
    }

    pub fn cycle_play_with_folder_name(&mut self, folder: i16) {
        self.send_command(
            CYCLE_PLAY_WITH_FOLDER_NAME,
            OptionalData::Data(folder.to_be_bytes()),
        );
    }

    pub fn set_single_cycle_play(&mut self, enable: bool) {
        self.send_command(
            SET_SINGLE_CYCLE_PLAY,
            OptionalData::Data([if enable { 0x00 } else { 0x01 }, 0x00]),
        );
    }

    pub fn set_dac(&mut self, enable: bool) {
        self.send_command(
            SET_DAC,
            OptionalData::Data([if enable { 0x00 } else { 0x01 }, 0x00]),
        );
    }

    pub fn play_with_volume(&mut self, volume: i8, index: i8) {
        self.send_command(
            PLAY_WITH_VOLUME,
            OptionalData::Data([volume as u8, index as u8]),
        );
    }

    pub fn play_with_folder_name(&mut self, folder: i8, index: i8) {
        self.send_command(
            PLAY_WITH_FOLDER_NAME,
            OptionalData::Data([folder as u8, index as u8]),
        );
    }
}
